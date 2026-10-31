// Start Menu styler — a native Rust port of the styling engine from the
// Windhawk `windows-11-start-menu-styler` mod, hosted inside
// `StartMenuExperienceHost.exe` by `pecofence_tap.dll`.
//
// The host (`pecofence.exe`) serializes the selected theme into a compact
// line wire-format (`pecofence_core::start_menu_themes::theme_wire`) and
// delivers it via `WM_COPYDATA` to our command window — no Windhawk, no
// external styling engine.
//
// Lock discipline: a `Mutex` is NEVER held across a XAML/COM call —
// `SetValue`/`ReadLocalValue`/registration run property-changed and
// visual-state callbacks synchronously on this thread, and those callbacks
// take the same locks. All COM work happens on the XAML UI thread.

use crate::bindings::{IXamlDiagnostics, InstanceHandle, VisualElement};
use crate::fx_bindings::IPropertyValue as XamlPropertyValue;
use crate::win32;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::{Rc, Weak as RcWeak};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use windows_core::{GUID, HRESULT, IInspectable, Interface};
use xb::{
    DependencyObject, DependencyProperty, DependencyPropertyChangedCallback, FontWeight,
    FrameworkElement, UIElement, VisualTreeHelper,
};

const E_TYPEMISMATCH: HRESULT = HRESULT(0x8002_0005_u32 as _);

/// Dispatcher priority — the enclosing module's `post_ui` maps this onto
/// `CoreDispatcherPriority` (wux) or `DispatcherQueuePriority` (mux).
#[derive(Clone, Copy)]
enum Prio {
    Low,
    Normal,
    High,
}

/// COM objects are only ever touched on the XAML UI thread — `DispatchedHandler`
/// closures need `Send` captures, so payloads crossing threads ride in this.
struct UiBound<T>(T);
unsafe impl<T> Send for UiBound<T> {}
unsafe impl<T> Sync for UiBound<T> {}

// ============================================================================
// Global state
// ============================================================================

// Resolved rules are dispatcher-affine: `resolve_ruleset` materializes
// brushes/styles via `XamlReader.Load`, and those objects can only be
// `SetValue`d onto elements of the dispatcher that created them — like the
// mod's `thread_local` style state, each thread keeps its own resolved copy,
// invalidated by `rules_gen`.
thread_local! {
    static TL_RULES: std::cell::RefCell<(u64, Rc<Vec<ResolvedRuleSet>>)> =
        std::cell::RefCell::new((0, Rc::new(Vec::new())));
}

fn rules_for_thread() -> Rc<Vec<ResolvedRuleSet>> {
    let (g, wire) = {
        let eng = state();
        (eng.rules_gen, eng.wire_rules.clone())
    };
    TL_RULES.with(|tl| {
        let mut tl = tl.borrow_mut();
        if tl.0 != g {
            let wire = wire.unwrap_or_default();
            let mut rs = Vec::with_capacity(wire.len());
            for r in wire.iter() {
                rs.push(resolve_ruleset(r));
            }
            *tl = (g, Rc::new(rs));
        }
        tl.1.clone()
    })
}

/// Which theme wire this engine instance is styling for. The same binary
/// ships in `StartMenuExperienceHost` (`PECO-SMS1`), `explorer` (`PECO-TBS1`)
/// and the Notification Center hosts (`PECO-NCS1`); all route through this
/// module.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WireKind {
    StartMenu,
    Taskbar,
    NotificationCenter,
}

struct State {
    /// Selected theme pending UI-thread apply.
    pending: Option<(WireKind, WireTheme)>,
    /// Whether `pending` picks the classic-variant rules.
    pending_classic: bool,
    /// The applied theme kind (kept for arm/disarm checks).
    applied: Option<WireKind>,
    /// The resource theme-dict merged into `Application.Resources`.
    theme_dict: Option<xb::ResourceDictionary>,
    /// Styled element handle -> bookkeeping.
    elements: BTreeMap<u64, ElemState>,
    /// `var -> captures`, indexed by name (the mod's `g_styleVariables`).
    variables: BTreeMap<String, Vec<Capture>>,
    /// `var -> consumers` (the mod's `g_styleVariableConsumers`).
    consumers: BTreeMap<String, Vec<Consumer>>,
    /// The XAML dispatcher, captured from the first styled element.
    dispatcher: Option<XDispatcher>,
    /// The active theme's raw wire rulesets — resolved lazily per dispatcher
    /// thread (`rules_for_thread`) because resolved values are affine.
    wire_rules: Option<Rc<Vec<WireRuleSet>>>,
    /// Bumped on each theme apply/reset — invalidates per-thread rule caches.
    rules_gen: u64,
    /// Handles of styled elements whose diagnostics instances should be
    /// released when the pump is quiet (`g_elementsToRelease` equivalent).
    to_release: Vec<u64>,
}
unsafe impl Send for State {}

fn state() -> std::sync::MutexGuard<'static, State> {
    static STATE: Mutex<State> = Mutex::new(State {
        pending: None,
        pending_classic: false,
        applied: None,
        theme_dict: None,
        elements: BTreeMap::new(),
        variables: BTreeMap::new(),
        consumers: BTreeMap::new(),
        dispatcher: None,
        wire_rules: None,
        rules_gen: 0,
        to_release: Vec::new(),
    });
    STATE.lock().unwrap_or_else(|p| p.into_inner())
}

/// Every element handle the watcher has seen + its reported `element.Type`
/// (the matcher's `fallbackClassName`) — the sweep list for theme
/// (re)application.
fn known() -> std::sync::MutexGuard<'static, BTreeMap<u64, String>> {
    static KNOWN: Mutex<BTreeMap<u64, String>> = Mutex::new(BTreeMap::new());
    KNOWN.lock().unwrap_or_else(|p| p.into_inner())
}

/// `g_elementPropertyModifying` — set while we write a property so the
/// re-apply callback skips our own writes.
static MODIFYING: AtomicBool = AtomicBool::new(false);

// ============================================================================
// Wire parsing + target/style-rule grammar (plain data; cmd thread)
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
enum Layout {
    #[default]
    Default,
    ForceNew,
    NewSideBySide,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum MKind {
    #[default]
    Element,
    Wildcard,
    Root,
}

/// `ElementMatcher` — `Type#Name[Prop=Value][N]@Vsg`, `*`, `:root`.
#[derive(Default, Clone)]
struct Matcher {
    kind: MKind,
    type_name: String,
    name: String,
    vsg: String,
    index: i32, // 0 = none; one-based otherwise
    props: Vec<(String, String)>,
}

#[derive(Clone)]
struct WireRuleSet {
    matcher: Matcher,
    parents: Vec<Matcher>, // nearest ancestor first
    styles: Vec<String>,
}

#[derive(Default)]
struct WireTheme {
    layout: Layout,
    resvars: Vec<(String, String)>,
    modern: Vec<WireRuleSet>,
    classic: Vec<WireRuleSet>,
}

/// `AdjustTypeName` from the mod (both variants' prefixes merged).
/// `CONTROLS_NS`/`SHAPES_NS` are per-framework constants supplied by the
/// enclosing module (`Windows.UI.Xaml` for the taskbar engine,
/// `Microsoft.UI.Xaml` for the File Explorer engine).
fn adjust_type_name(t: &str) -> String {
    if !t.contains(['.', ':']) {
        if t == "Rectangle" {
            return format!("{SHAPES_NS}Rectangle");
        }
        return format!("{CONTROLS_NS}{t}");
    }
    for (from, to) in [
        ("StartMenu:", "StartMenu."),
        ("muxc:", "Microsoft.UI.Xaml.Controls."),
        ("taskbar:", "Taskbar."),
        ("systemtray:", "SystemTray."),
        ("udk:", "WindowsUdk.UI.Shell."),
    ] {
        if let Some(rest) = t.strip_prefix(from) {
            return format!("{to}{rest}");
        }
    }
    t.to_string()
}

/// `ElementMatcherFromString`.
fn matcher_from_string(s: &str) -> Result<Matcher, String> {
    let mut m = Matcher {
        kind: MKind::Element,
        ..Default::default()
    };
    let t = s.trim();
    if t == "*" {
        m.kind = MKind::Wildcard;
        return Ok(m);
    }
    if t == ":root" {
        m.kind = MKind::Root;
        return Ok(m);
    }
    let bytes = s.as_bytes();
    let mut i = s.find(['#', '@', '[']).unwrap_or(s.len());
    m.type_name = s[..i].trim().to_string();
    if m.type_name.is_empty() {
        return Err("empty type".into());
    }
    while i < s.len() {
        let marker = bytes[i] as char;
        let next = s[i + 1..]
            .find(['#', '@', '['])
            .map(|p| i + 1 + p)
            .unwrap_or(s.len());
        let part = &s[i + 1..next];
        match marker {
            '#' => {
                if !m.name.is_empty() {
                    return Err("more than one name".into());
                }
                m.name = part.trim().to_string();
                if m.name.is_empty() {
                    return Err("empty name".into());
                }
            }
            '@' => {
                if !m.vsg.is_empty() {
                    return Err("more than one visual state group".into());
                }
                m.vsg = part.trim().to_string();
            }
            '[' => {
                let rule = part.trim();
                let Some(rule) = rule.strip_suffix(']') else {
                    return Err("missing ']'".into());
                };
                let rule = rule.trim();
                if rule.is_empty() {
                    return Err("empty property".into());
                }
                if rule.bytes().all(|b| b.is_ascii_digit()) {
                    m.index = rule.parse::<i32>().map_err(|_| "bad index")?;
                } else {
                    let eq = rule.find('=').ok_or("missing '=' in property")?;
                    let key = rule[..eq].trim();
                    if key.is_empty() {
                        return Err("empty property name".into());
                    }
                    m.props.push((key.into(), rule[eq + 1..].trim().into()));
                }
            }
            _ => unreachable!(),
        }
        i = next;
    }
    if m.kind == MKind::Element {
        m.type_name = adjust_type_name(&m.type_name);
    }
    Ok(m)
}

/// `SplitTargetString` — commas split targets, `[,]`-quoted commas don't.
fn split_targets(target: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut in_prop = false;
    for (i, c) in target.char_indices() {
        match c {
            '[' => in_prop = true,
            ']' => in_prop = false,
            ',' if !in_prop => {
                out.push(&target[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&target[start..]);
    out
}

/// `AddElementCustomizationRulesForSingleTarget`.
fn single_ruleset(target: &str) -> Result<(Matcher, Vec<Matcher>), String> {
    let mut leaf: Option<Matcher> = None;
    let mut parents: Vec<Matcher> = Vec::new();
    let mut has_vsg = false;
    let parts: Vec<&str> = target.split(" > ").collect();
    for (idx, part) in parts.iter().rev().enumerate() {
        let is_leftmost = idx == parts.len() - 1;
        let m = matcher_from_string(part)?;
        match m.kind {
            MKind::Wildcard => {
                if idx == 0 {
                    return Err("'*' can't be the matched element".into());
                }
                if is_leftmost {
                    return Err("'*' can't be the leftmost part".into());
                }
                if parents.last().map(|p| p.kind) == Some(MKind::Wildcard) {
                    return Err("'*' can't be adjacent to '*'".into());
                }
            }
            MKind::Root => {
                if idx == 0 {
                    return Err("':root' can't be the matched element".into());
                }
                if !is_leftmost {
                    return Err("':root' must be the leftmost part".into());
                }
                if parents.last().map(|p| p.kind) == Some(MKind::Wildcard) {
                    return Err("':root' must follow a non-wildcard".into());
                }
            }
            MKind::Element => {}
        }
        if !m.vsg.is_empty() {
            if has_vsg {
                return Err("more than one visual state group".into());
            }
            has_vsg = true;
        }
        if idx == 0 {
            leaf = Some(m);
        } else {
            parents.push(m);
        }
    }
    Ok((leaf.ok_or("empty target")?, parents))
}

/// `ParseRule`.
enum ParsedRule {
    Value {
        prop: String,
        state: String,
        is_xaml: bool,
        value: String,
    },
    Capture {
        prop: String,
        var: String,
    },
}

fn valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_rule(s: &str) -> Result<ParsedRule, String> {
    let eq = s.find('=').ok_or("'=' is missing")?;
    let mut name = &s[..eq];
    let value = &s[eq + 1..];
    if let Some(var) = value.strip_prefix('>') {
        if name.ends_with(':') {
            return Err("':=>' is not valid".into());
        }
        if name.contains('@') {
            return Err("'@VisualState' not allowed on a capture".into());
        }
        let prop = name.trim();
        if prop.is_empty() {
            return Err("empty name".into());
        }
        let var = var.trim();
        if var.is_empty() || !valid_identifier(var) {
            return Err("invalid capture variable name".into());
        }
        return Ok(ParsedRule::Capture {
            prop: prop.into(),
            var: var.into(),
        });
    }
    let mut is_xaml = false;
    if name.ends_with(':') {
        is_xaml = true;
        name = &name[..name.len() - 1];
    }
    let (prop, state) = match name.find('@') {
        Some(at) => (&name[..at], name[at + 1..].trim()),
        None => (name, ""),
    };
    let prop = prop.trim();
    if prop.is_empty() {
        return Err("empty name".into());
    }
    Ok(ParsedRule::Value {
        prop: prop.into(),
        state: state.into(),
        is_xaml,
        value: value.trim().into(),
    })
}

/// Parses the `PECO-SMS1`/`PECO-TBS1` wire produced by `theme_wire` in core.
fn parse_wire(text: &str) -> Result<(WireKind, WireTheme), String> {
    let mut theme = WireTheme::default();
    let mut lines = text.lines();
    let kind = match lines.next().and_then(wire_kind) {
        Some(k) => k,
        None => return Err("bad magic".into()),
    };
    let mut in_classic = false;
    // `T` lines spawn one ruleset per comma-split target; the following `S`
    // lines apply to all of them.
    let mut pending: Vec<WireRuleSet> = Vec::new();
    macro_rules! flush {
        () => {
            let dst = if in_classic {
                &mut theme.classic
            } else {
                &mut theme.modern
            };
            dst.append(&mut pending);
        };
    }
    for line in lines {
        match line.as_bytes().first() {
            Some(b'L') if line.starts_with("L ") => {
                theme.layout = match line[2..].trim() {
                    "forceNewLayout" => Layout::ForceNew,
                    "newLayoutSideBySide" => Layout::NewSideBySide,
                    _ => Layout::Default,
                };
            }
            Some(b'C') if line == "C" => {
                flush!();
                in_classic = true;
            }
            Some(b'R') if line.starts_with("R ") => {
                if let Some((k, v)) = line[2..].split_once('=') {
                    theme.resvars.push((k.trim().into(), v.trim().into()));
                }
            }
            Some(b'T') if line.starts_with("T ") => {
                flush!();
                let target = line[2..].trim();
                for single in split_targets(target) {
                    match single_ruleset(single) {
                        Ok((matcher, parents)) => pending.push(WireRuleSet {
                            matcher,
                            parents,
                            styles: Vec::new(),
                        }),
                        Err(e) => crate::tap_log!("target {single:?} rejected: {e}"),
                    }
                }
            }
            Some(b'S') if line.starts_with("S ") => {
                for set in &mut pending {
                    set.styles.push(line[2..].to_string());
                }
            }
            Some(b'E') if line == "E" => break,
            _ => {}
        }
    }
    flush!();
    Ok((kind, theme))
}

// ============================================================================
// `{{expr}}` evaluation + `UnboxedPropertyValue`
// ============================================================================

#[derive(Clone)]
enum Unboxed {
    Str(String),
    Bool(bool),
    Num(f64),
}

impl Unboxed {
    fn as_num(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            Self::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Self::Str(_) => None,
        }
    }
    /// `FormatUnboxedPropertyValue`.
    fn to_text(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            Self::Bool(b) => if *b { "True" } else { "False" }.into(),
            Self::Num(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    format!("{}", *n as i64)
                } else {
                    format!("{n}")
                }
            }
        }
    }
}

/// `TryUnboxPropertyValue` — `IPropertyValue` primitives + int enums.
fn unbox(value: &IInspectable) -> Option<Unboxed> {
    let pv: crate::fx_bindings::IPropertyValue = Interface::cast(value).ok()?;
    use crate::fx_bindings::PropertyType;
    match pv.Type().ok()? {
        PropertyType::String => Some(Unboxed::Str(pv.GetString().ok()?.to_string_lossy())),
        PropertyType::Boolean => Some(Unboxed::Bool(pv.GetBoolean().ok()?)),
        PropertyType::Double => Some(Unboxed::Num(pv.GetDouble().ok()?)),
        PropertyType::Single => Some(Unboxed::Num(pv.GetSingle().ok()? as f64)),
        PropertyType::Int16 => Some(Unboxed::Num(pv.GetInt16().ok()? as f64)),
        PropertyType::UInt16 => Some(Unboxed::Num(pv.GetUInt16().ok()? as f64)),
        PropertyType::Int32 => Some(Unboxed::Num(pv.GetInt32().ok()? as f64)),
        PropertyType::UInt32 => Some(Unboxed::Num(pv.GetUInt32().ok()? as f64)),
        PropertyType::Int64 => Some(Unboxed::Num(pv.GetInt64().ok()? as f64)),
        PropertyType::UInt64 => Some(Unboxed::Num(pv.GetUInt64().ok()? as f64)),
        PropertyType::Char16 => Some(Unboxed::Str(
            char::from_u32(pv.GetChar16().ok()? as u32)
                .map(|c| c.to_string())
                .unwrap_or_default(),
        )),
        _ => None,
    }
}

/// `StyleVariableValue` — a capture's recorded value.
#[derive(Clone)]
struct CapVal {
    /// Invariant text — formatted primitive, or the class name for opaque
    /// captures (diagnostics only).
    string_form: String,
    /// Numeric form — only when the source was a numeric primitive.
    numeric: Option<f64>,
    /// `string_form` may be inlined verbatim into XAML (numeric/bool/string).
    substitutable: bool,
}

/// One element's capture of a variable (`StyleVariableCapture`).
#[derive(Clone)]
struct Capture {
    /// Capturing element's handle.
    owner: u64,
    /// Spine-root key — confines the variable to its tree's scope
    /// (`0` = no node; only matches other unscoped consumers).
    scope: usize,
    val: CapVal,
}

/// A `{{…}}` consumer registration (`StyleVariableConsumer`).
#[derive(Clone)]
struct Consumer {
    handle: u64,
    dp_raw: usize,
    /// The consumer element's reported type — re-resolve fallback.
    fallback: String,
    /// Spine-root key of the consumer — propagation stays in scope.
    scope: usize,
}

/// `(name, owner)` — `variableDependencies` entries; `0` owner = unresolved.
type Dep = (String, u64);

/// `StyleExpressionValue`.
#[derive(Clone)]
enum ExprVal {
    Num(f64),
    Str(String),
}

/// `std::runtime_error` (Fail) / `StyleVariableSkipRequested` (Skip).
enum EvalErr {
    Fail,
    Skip,
}

/// `StyleVariableExpressionEvaluator` — direct port of the mod's grammar:
/// numbers, backtick strings, identifiers, parens, `+ - * /`, `^`-less unary,
/// `< <= == >= > !=`, `cond ? a : b`, `min`/`max`/`skip()`. `m_live` suppresses
/// value errors + dep capture on the dead ternary branch.
struct Eval<'a> {
    s: &'a [u8],
    pos: usize,
    live: bool,
    /// Consumer's spine node for capture ranking.
    node: Option<Rc<TreeNode>>,
    /// Consumer's scope key.
    scope: usize,
    /// `outDeps` — `(name, winning-owner-or-0)` per live lookup.
    deps: &'a mut Vec<Dep>,
}

impl Eval<'_> {
    fn ws(&mut self) {
        while self.pos < self.s.len() && matches!(self.s[self.pos], b' ' | b'\t' | b'\r' | b'\n') {
            self.pos += 1;
        }
    }
    fn eat(&mut self, c: u8) -> bool {
        self.ws();
        if self.s.get(self.pos) == Some(&c) {
            self.pos += 1;
            return true;
        }
        false
    }
    fn eat_op(&mut self, op: &str) -> bool {
        self.ws();
        if self.s[self.pos..].starts_with(op.as_bytes()) {
            self.pos += op.len();
            return true;
        }
        false
    }
    fn req_num(&self, v: &ExprVal) -> Result<f64, EvalErr> {
        match v {
            ExprVal::Num(n) => Ok(*n),
            // Dead branch: tolerate strings (reported as 0) rather than abort.
            ExprVal::Str(_) if !self.live => Ok(0.0),
            _ => Err(EvalErr::Fail),
        }
    }
    fn vals_eq(a: &ExprVal, b: &ExprVal) -> bool {
        match (a, b) {
            (ExprVal::Num(x), ExprVal::Num(y)) => x == y,
            (ExprVal::Str(x), ExprVal::Str(y)) => x == y,
            _ => false,
        }
    }
    /// `Evaluate` — returns the formatted result text.
    fn evaluate(&mut self) -> Result<String, EvalErr> {
        self.pos = 0;
        self.ws();
        let v = self.parse_expr()?;
        self.ws();
        if self.pos != self.s.len() {
            return Err(EvalErr::Fail);
        }
        Ok(match v {
            ExprVal::Num(n) => fmt_num(n),
            ExprVal::Str(s) => s,
        })
    }
    fn parse_expr(&mut self) -> Result<ExprVal, EvalErr> {
        self.parse_ternary()
    }
    fn parse_ternary(&mut self) -> Result<ExprVal, EvalErr> {
        let cond = self.parse_equality()?;
        if !self.eat(b'?') {
            return Ok(cond);
        }
        let cond_true = self.req_num(&cond)? != 0.0;
        let prev = self.live;
        self.live = prev && cond_true;
        let then_v = self.parse_expr()?;
        self.live = prev;
        if !self.eat(b':') {
            return Err(EvalErr::Fail);
        }
        self.live = prev && !cond_true;
        let else_v = self.parse_ternary()?;
        self.live = prev;
        Ok(if cond_true { then_v } else { else_v })
    }
    fn parse_equality(&mut self) -> Result<ExprVal, EvalErr> {
        let mut v = self.parse_relational()?;
        loop {
            if self.eat_op("==") {
                let r = self.parse_relational()?;
                v = ExprVal::Num(if Self::vals_eq(&v, &r) { 1.0 } else { 0.0 });
            } else if self.eat_op("!=") {
                let r = self.parse_relational()?;
                v = ExprVal::Num(if Self::vals_eq(&v, &r) { 0.0 } else { 1.0 });
            } else {
                break;
            }
        }
        Ok(v)
    }
    fn parse_relational(&mut self) -> Result<ExprVal, EvalErr> {
        let mut v = self.parse_additive()?;
        loop {
            let f: fn(f64, f64) -> bool = if self.eat_op("<=") {
                |a, b| a <= b
            } else if self.eat_op(">=") {
                |a, b| a >= b
            } else if self.eat_op("<") {
                |a, b| a < b
            } else if self.eat_op(">") {
                |a, b| a > b
            } else {
                break;
            };
            let lhs = self.req_num(&v)?;
            let rhs_v = self.parse_additive()?;
            let rhs = self.req_num(&rhs_v)?;
            v = ExprVal::Num(if f(lhs, rhs) { 1.0 } else { 0.0 });
        }
        Ok(v)
    }
    fn parse_additive(&mut self) -> Result<ExprVal, EvalErr> {
        let mut v = self.parse_term()?;
        loop {
            if self.eat(b'+') || self.eat(b'-') {
                let sub = self.s[self.pos - 1] == b'-';
                let lhs = self.req_num(&v)?;
                let rhs_v = self.parse_term()?;
                let rhs = self.req_num(&rhs_v)?;
                v = ExprVal::Num(if sub { lhs - rhs } else { lhs + rhs });
            } else {
                break;
            }
        }
        Ok(v)
    }
    fn parse_term(&mut self) -> Result<ExprVal, EvalErr> {
        let mut v = self.parse_factor()?;
        loop {
            if self.eat(b'*') {
                let lhs = self.req_num(&v)?;
                let rhs_v = self.parse_factor()?;
                let rhs = self.req_num(&rhs_v)?;
                v = ExprVal::Num(lhs * rhs);
            } else if self.eat(b'/') {
                let lhs = self.req_num(&v)?;
                let rhs_v = self.parse_factor()?;
                let rhs = self.req_num(&rhs_v)?;
                if rhs == 0.0 {
                    if self.live {
                        return Err(EvalErr::Fail);
                    }
                    v = ExprVal::Num(lhs);
                } else {
                    v = ExprVal::Num(lhs / rhs);
                }
            } else {
                break;
            }
        }
        Ok(v)
    }
    fn parse_factor(&mut self) -> Result<ExprVal, EvalErr> {
        self.ws();
        if self.eat(b'+') {
            let v = self.parse_factor()?;
            return Ok(ExprVal::Num(self.req_num(&v)?));
        }
        if self.eat(b'-') {
            let v = self.parse_factor()?;
            return Ok(ExprVal::Num(-self.req_num(&v)?));
        }
        self.parse_primary()
    }
    fn parse_primary(&mut self) -> Result<ExprVal, EvalErr> {
        self.ws();
        let Some(&c) = self.s.get(self.pos) else {
            return Err(EvalErr::Fail);
        };
        match c {
            b'(' => {
                self.pos += 1;
                let v = self.parse_expr()?;
                self.ws();
                if !self.eat(b')') {
                    return Err(EvalErr::Fail);
                }
                Ok(v)
            }
            b'`' => self.parse_string_literal(),
            b'0'..=b'9' | b'.' => Ok(ExprVal::Num(self.parse_number()?)),
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.parse_ident_or_call(),
            _ => Err(EvalErr::Fail),
        }
    }
    /// Backtick literal — `` `` `` encodes one literal backtick.
    fn parse_string_literal(&mut self) -> Result<ExprVal, EvalErr> {
        self.pos += 1;
        let mut out = String::new();
        while self.pos < self.s.len() {
            let c = self.s[self.pos];
            if c == b'`' {
                if self.s.get(self.pos + 1) == Some(&b'`') {
                    out.push('`');
                    self.pos += 2;
                    continue;
                }
                self.pos += 1;
                return Ok(ExprVal::Str(out));
            }
            // Copy the UTF-8 sequence verbatim.
            let len = if c < 0x80 {
                1
            } else if c < 0xE0 {
                2
            } else if c < 0xF0 {
                3
            } else {
                4
            };
            let end = (self.pos + len).min(self.s.len());
            out.push_str(core::str::from_utf8(&self.s[self.pos..end]).unwrap_or_default());
            self.pos = end;
        }
        Err(EvalErr::Fail)
    }
    fn parse_number(&mut self) -> Result<f64, EvalErr> {
        let start = self.pos;
        let mut saw_digit = false;
        let mut saw_dot = false;
        while self.pos < self.s.len() {
            match self.s[self.pos] {
                b'0'..=b'9' => {
                    saw_digit = true;
                    self.pos += 1;
                }
                b'.' if !saw_dot => {
                    saw_dot = true;
                    self.pos += 1;
                }
                _ => break,
            }
        }
        if self.pos < self.s.len() && matches!(self.s[self.pos], b'e' | b'E') {
            self.pos += 1;
            if matches!(self.s.get(self.pos), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            while matches!(self.s.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if !saw_digit {
            return Err(EvalErr::Fail);
        }
        core::str::from_utf8(&self.s[start..self.pos])
            .ok()
            .and_then(|t| t.parse().ok())
            .ok_or(EvalErr::Fail)
    }
    fn parse_ident_or_call(&mut self) -> Result<ExprVal, EvalErr> {
        let start = self.pos;
        while self.pos < self.s.len()
            && (self.s[self.pos].is_ascii_alphanumeric() || self.s[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let ident = core::str::from_utf8(&self.s[start..self.pos]).unwrap_or("");
        self.ws();
        if self.s.get(self.pos) == Some(&b'(') {
            self.pos += 1;
            if ident == "skip" {
                if !self.eat(b')') {
                    return Err(EvalErr::Fail);
                }
                if self.live {
                    return Err(EvalErr::Skip);
                }
                return Ok(ExprVal::Num(0.0));
            }
            let a_v = self.parse_expr()?;
            let a = self.req_num(&a_v)?;
            if !self.eat(b',') {
                return Err(EvalErr::Fail);
            }
            let b_v = self.parse_expr()?;
            let b = self.req_num(&b_v)?;
            if !self.eat(b')') {
                return Err(EvalErr::Fail);
            }
            return match ident {
                "min" => Ok(ExprVal::Num(a.min(b))),
                "max" => Ok(ExprVal::Num(a.max(b))),
                _ if self.live => Err(EvalErr::Fail),
                _ => Ok(ExprVal::Num(0.0)),
            };
        }
        self.lookup_variable(ident)
    }
    /// `LookupVariable` — ranked capture read + dep registration.
    fn lookup_variable(&mut self, name: &str) -> Result<ExprVal, EvalErr> {
        if !self.live {
            return Ok(ExprVal::Str(String::new()));
        }
        let res = find_winning(name, self.node.as_ref(), self.scope);
        self.deps
            .push((name.to_string(), res.as_ref().map_or(0, |(_, o)| *o)));
        match res {
            None => Ok(ExprVal::Str(String::new())),
            Some((v, _)) => {
                if let Some(n) = v.numeric {
                    Ok(ExprVal::Num(n))
                } else if v.substitutable {
                    Ok(ExprVal::Str(v.string_form))
                } else {
                    Err(EvalErr::Fail)
                }
            }
        }
    }
}

/// `FormatDoubleInvariant` (`.17g`-ish — the mod uses `%.17g` trimmed; Rust's
/// shortest-roundtrip display is equivalent for the catalog's values).
fn fmt_num(n: f64) -> String {
    if n.fract() == 0.0 && n.is_finite() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

/// `EvaluateStyleVariableExpression` — bare-ident fast path + full evaluate.
/// Deps are appended to `deps` even on failure (committed before resolution).
fn eval_expr_text(
    expr: &str,
    node: Option<&Rc<TreeNode>>,
    scope: usize,
    deps: &mut Vec<Dep>,
) -> Result<Option<String>, EvalErr> {
    let t = expr.trim();
    if t.is_empty() {
        return Ok(None);
    }
    if valid_identifier(t) {
        let res = find_winning(t, node, scope);
        deps.push((t.to_string(), res.as_ref().map_or(0, |(_, o)| *o)));
        return Ok(match res {
            Some((v, _)) if v.substitutable => Some(v.string_form),
            _ => None,
        });
    }
    let mut ev = Eval {
        s: t.as_bytes(),
        pos: 0,
        live: true,
        node: node.cloned(),
        scope,
        deps,
    };
    match ev.evaluate() {
        Ok(text) => Ok(Some(text)),
        Err(EvalErr::Fail) => Ok(None),
        Err(EvalErr::Skip) => Err(EvalErr::Skip),
    }
}

/// `ExpandStyleVariables` — innermost-first `{{…}}` substitution: the first
/// `}}` pairs with the rightmost `{{` before it. `Err(Skip)` unwinds live
/// `skip()` calls; `Ok(None)` = expansion failed (style stays unapplied but
/// recorded deps still commit).
fn expand_variables(
    raw: &str,
    node: Option<&Rc<TreeNode>>,
    scope: usize,
    deps: &mut Vec<Dep>,
) -> Result<Option<String>, EvalErr> {
    let mut out = raw.to_string();
    let mut scan_from = 0usize;
    loop {
        let bytes = out.as_bytes();
        let mut close = None;
        let mut i = scan_from;
        while i + 1 < bytes.len() {
            if bytes[i] == b'}' && bytes[i + 1] == b'}' {
                close = Some(i);
                break;
            }
            i += 1;
        }
        let Some(close) = close else { break };
        let mut open = None;
        if close >= 2 {
            let mut j = close - 1;
            loop {
                if bytes[j - 1] == b'{' && bytes[j] == b'{' {
                    open = Some(j - 1);
                    break;
                }
                if j == 1 {
                    break;
                }
                j -= 1;
            }
        }
        let Some(open) = open else {
            crate::tap_log!("unmatched '}}' at {close}");
            return Ok(None);
        };
        let expanded = eval_expr_text(&out[open + 2..close], node, scope, deps)?;
        let Some(expanded) = expanded else {
            return Ok(None);
        };
        out.replace_range(open..close + 2, &expanded);
        scan_from = open + expanded.len();
    }
    Ok(Some(out))
}

// ============================================================================
// `ElementTreeNode` spine pool — LCA-ranked `=>` capture resolution.
// Ported from the taskbar mod: each element gets a lazily built parent-spine;
// captures rank by lowest-common-ancestor depth vs. the consuming element.
// ============================================================================

/// `ElementTreeNode` — one node of the parent-spine. The weak ref proves a
/// pool hit isn't a recycled address.
struct TreeNode {
    /// `None` when the object refused `downgrade` — spine then can't verify
    /// liveness and treats the node as live-for-this-call only.
    weak: Option<windows_core::Weak<DependencyObject>>,
    parent: Option<Rc<TreeNode>>,
    depth: u32,
    /// The depth-0 node this spine hangs from — kept alive by the chain.
    root: core::cell::Cell<*const TreeNode>,
}

thread_local! {
    /// `g_elementTreeNodes` — keyed by the object's `IUnknown` pointer (the
    /// only COM-guaranteed stable identity).
    static TREE_NODES: std::cell::RefCell<HashMap<usize, RcWeak<TreeNode>>> =
        std::cell::RefCell::new(HashMap::new());
    /// `g_styleVariablePropagationDepth` / `g_pendingStyleVariablePropagations`.
    static PROP_DEPTH: core::cell::Cell<u32> = const { core::cell::Cell::new(0) };
    static PROP_QUEUE: std::cell::RefCell<Vec<(String, Option<u64>, usize)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// `ElementIdentityKey`.
fn identity_key(d: &DependencyObject) -> Option<usize> {
    Interface::cast::<windows_core::IUnknown>(d)
        .ok()
        .map(|u| Interface::as_raw(&u) as usize)
}

/// The diagnostics-reported `element.Type`, keyed by `identity_key` — leaf
/// matchers get it as `fallback`, but `match_parents` walks ancestors via
/// `VisualTreeHelper` where no report arrives. Without it, parent matchers
/// on internal names (`Microsoft.UI.Xaml.Internal.RootScrollViewer`, whose
/// runtime class reports as `Controls.ScrollViewer`) can never match.
fn reported_types() -> std::sync::MutexGuard<'static, HashMap<usize, String>> {
    static M: std::sync::OnceLock<Mutex<HashMap<usize, String>>> = std::sync::OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|p| p.into_inner())
}

/// The reported type for an element walked outside the advise callback.
fn reported_fallback(el: &FrameworkElement) -> String {
    Interface::cast::<DependencyObject>(el)
        .ok()
        .and_then(|d| identity_key(&d))
        .and_then(|k| reported_types().get(&k).cloned())
        .unwrap_or_default()
}

/// `IsStaleSpine` — a depth-0 node whose object since gained a parent stops
/// short of the real root; rebuild.
fn is_stale_spine(node: &Rc<TreeNode>) -> bool {
    let mut n = node.clone();
    while let Some(p) = &n.parent {
        n = p.clone();
    }
    match n.weak.as_ref().and_then(|w| w.upgrade()) {
        Some(obj) => VisualTreeHelper::GetParent(&obj).is_ok(),
        None => false,
    }
}

/// `GetOrCreateElementTreeNode`.
fn node_for(dobj: &DependencyObject) -> Option<Rc<TreeNode>> {
    let mut missing: Vec<DependencyObject> = Vec::new();
    let mut base: Option<Rc<TreeNode>> = None;
    let mut cur = Some(dobj.clone());
    while let Some(d) = cur {
        let key = identity_key(&d)?;
        let hit = TREE_NODES.with(|p| p.borrow().get(&key).and_then(|w| w.upgrade()));
        if let Some(existing) = hit {
            let live = matches!(&existing.weak, Some(w) if w.upgrade().is_some());
            if live && !is_stale_spine(&existing) {
                base = Some(existing);
                break;
            }
            TREE_NODES.with(|p| p.borrow_mut().remove(&key));
        }
        missing.push(d.clone());
        cur = VisualTreeHelper::GetParent(&d).ok();
    }
    for d in missing.iter().rev() {
        let key = identity_key(d)?;
        let node = Rc::new(TreeNode {
            weak: d.downgrade().ok(),
            parent: base.clone(),
            depth: base.as_ref().map_or(0, |b| b.depth + 1),
            root: core::cell::Cell::new(core::ptr::null()),
        });
        node.root
            .set(base.as_ref().map_or(Rc::as_ptr(&node), |b| b.root.get()));
        TREE_NODES.with(|p| p.borrow_mut().insert(key, Rc::downgrade(&node)));
        base = Some(node);
    }
    base
}

/// `EnsureElementTreeNode`.
fn ensure_node(es: &mut ElemState) -> Option<Rc<TreeNode>> {
    if es.tree_node.as_ref().is_none_or(|n| is_stale_spine(n)) {
        let dobj: DependencyObject = Interface::cast(&es.el).ok()?;
        es.tree_node = node_for(&dobj);
    }
    es.tree_node.clone()
}

/// The scope key for a node — the spine-root's address (`0` = unscoped).
fn scope_of(node: Option<&Rc<TreeNode>>) -> usize {
    node.map_or(0, |n| n.root.get() as usize)
}

/// `ElementTreeLcaDepth`.
fn lca_depth(a: Option<Rc<TreeNode>>, b: Option<Rc<TreeNode>>) -> i32 {
    let (mut a, mut b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        _ => return -1,
    };
    while a.depth > b.depth {
        match a.parent.clone() {
            Some(p) => a = p,
            None => return -1,
        }
    }
    while b.depth > a.depth {
        match b.parent.clone() {
            Some(p) => b = p,
            None => return -1,
        }
    }
    while !Rc::ptr_eq(&a, &b) {
        a = match a.parent.clone() {
            Some(p) => p,
            None => return -1,
        };
        b = match b.parent.clone() {
            Some(p) => p,
            None => return -1,
        };
    }
    a.depth as i32
}

/// `StyleVariableCaptureRank` — `(-lcaDepth, captureDepth)`; smaller wins.
fn capture_rank(consumer: Option<&Rc<TreeNode>>, capture: Option<&Rc<TreeNode>>) -> (i32, i64) {
    (
        -lca_depth(consumer.cloned(), capture.cloned()),
        capture.map_or(i64::MAX, |n| n.depth as i64),
    )
}

/// Snapshot a variable's captures + their owners' spine nodes, filtered to
/// `scope` (`SnapshotStyleVariableCaptures`).
fn capture_candidates(name: &str, scope: usize) -> Vec<(u64, Option<Rc<TreeNode>>, CapVal)> {
    let mut eng = state();
    let raw: Vec<(u64, CapVal)> = eng
        .variables
        .get(name)
        .map(|caps| {
            caps.iter()
                .filter(|c| c.scope == scope)
                .map(|c| (c.owner, c.val.clone()))
                .collect()
        })
        .unwrap_or_default();
    let mut out = Vec::with_capacity(raw.len());
    for (owner, val) in raw {
        let node = eng.elements.get_mut(&owner).and_then(ensure_node);
        out.push((owner, node, val));
    }
    out
}

/// `FindWinningCapture` — single capture short-circuits the ranking; ties go
/// to the last-registered (`<=` over insertion order).
fn find_winning(
    name: &str,
    consumer_node: Option<&Rc<TreeNode>>,
    scope: usize,
) -> Option<(CapVal, u64)> {
    let candidates = capture_candidates(name, scope);
    match candidates.len() {
        0 => None,
        1 => {
            let (owner, _, val) = candidates.into_iter().next().unwrap();
            Some((val, owner))
        }
        _ => {
            let mut best: Option<((i32, i64), CapVal, u64)> = None;
            for (owner, node, val) in candidates {
                let rank = capture_rank(consumer_node, node.as_ref());
                if best.as_ref().is_none_or(|(b, _, _)| rank <= *b) {
                    best = Some((rank, val, owner));
                }
            }
            best.map(|(_, v, o)| (v, o))
        }
    }
}

/// `PickWinningCaptureOwner`.
fn pick_winning_owner(
    candidates: &[(u64, Option<Rc<TreeNode>>, CapVal)],
    consumer_node: Option<&Rc<TreeNode>>,
) -> u64 {
    let mut best: Option<((i32, i64), u64)> = None;
    for (owner, node, _) in candidates {
        let rank = capture_rank(consumer_node, node.as_ref());
        if best.is_none_or(|(b, _)| rank <= b) {
            best = Some((rank, *owner));
        }
    }
    best.map_or(0, |(_, o)| o)
}

// ============================================================================
// `IReference<FontWeight>` — `Setter.Value()` hands FontWeight back as a bare
// int; `SetValue` requires the boxed `Windows.UI.Text.FontWeight` (the mod's
// own workaround). Hand-rolled: IReference<FontWeight>'s vtable = IPropertyValue
// + `get_Value`, IID from the parameterized-interface signature.
// ============================================================================

#[repr(transparent)]
#[derive(Clone)]
struct IFontWeightRef(windows_core::IUnknown);

#[repr(C)]
struct IFontWeightRef_Vtbl {
    base__: crate::fx_bindings::IPropertyValue_Vtbl,
    Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FontWeight) -> HRESULT,
}

unsafe impl Interface for IFontWeightRef {
    type Vtable = IFontWeightRef_Vtbl;
    /// Per-engine IID — wux boxes `Windows.UI.Text.FontWeight`, mux boxes
    /// `Microsoft.UI.Text.FontWeight`; each supplied by the module.
    const IID: GUID = FONTWEIGHT_IID;
}

trait IFontWeightRef_Impl: crate::fx_bindings::IPropertyValue_Impl {
    fn Value(&self) -> windows_core::Result<FontWeight>;
}

impl IFontWeightRef_Vtbl {
    pub const fn new<Identity: IFontWeightRef_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: IFontWeightRef_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut FontWeight,
        ) -> HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFontWeightRef_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: crate::fx_bindings::IPropertyValue_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontWeightRef as Interface>::IID
    }
}

impl windows_core::RuntimeName for IFontWeightRef {
    const NAME: &'static str = FONTWEIGHT_RUNTIME_NAME;
}

#[windows_core::implement(IFontWeightRef, XamlPropertyValue)]
struct FontWeightBox(u16);

macro_rules! type_mismatch {
    () => {
        Err(windows_core::Error::from_hresult(E_TYPEMISMATCH))
    };
}

impl IFontWeightRef_Impl for FontWeightBox_Impl {
    fn Value(&self) -> windows_core::Result<FontWeight> {
        Ok(FontWeight { Weight: self.0 })
    }
}

#[allow(clippy::too_many_lines)]
impl crate::fx_bindings::IPropertyValue_Impl for FontWeightBox_Impl {
    fn Type(&self) -> windows_core::Result<crate::fx_bindings::PropertyType> {
        Ok(crate::fx_bindings::PropertyType::OtherType)
    }
    fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        Ok(false)
    }
    fn GetUInt16(&self) -> windows_core::Result<u16> {
        Ok(self.0)
    }
    fn GetUInt8(&self) -> windows_core::Result<u8> {
        type_mismatch!()
    }
    fn GetInt16(&self) -> windows_core::Result<i16> {
        type_mismatch!()
    }
    fn GetInt32(&self) -> windows_core::Result<i32> {
        type_mismatch!()
    }
    fn GetUInt32(&self) -> windows_core::Result<u32> {
        type_mismatch!()
    }
    fn GetInt64(&self) -> windows_core::Result<i64> {
        type_mismatch!()
    }
    fn GetUInt64(&self) -> windows_core::Result<u64> {
        type_mismatch!()
    }
    fn GetSingle(&self) -> windows_core::Result<f32> {
        type_mismatch!()
    }
    fn GetDouble(&self) -> windows_core::Result<f64> {
        type_mismatch!()
    }
    fn GetChar16(&self) -> windows_core::Result<u16> {
        type_mismatch!()
    }
    fn GetBoolean(&self) -> windows_core::Result<bool> {
        type_mismatch!()
    }
    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        type_mismatch!()
    }
    fn GetGuid(&self) -> windows_core::Result<GUID> {
        type_mismatch!()
    }
    fn GetDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        type_mismatch!()
    }
    fn GetTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        type_mismatch!()
    }
    fn GetPoint(&self) -> windows_core::Result<crate::fx_bindings::Point> {
        type_mismatch!()
    }
    fn GetSize(&self) -> windows_core::Result<crate::fx_bindings::Size> {
        type_mismatch!()
    }
    fn GetRect(&self) -> windows_core::Result<crate::fx_bindings::Rect> {
        type_mismatch!()
    }
    fn GetUInt8Array(&self, _: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetInt16Array(&self, _: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetUInt16Array(&self, _: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetInt32Array(&self, _: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetUInt32Array(&self, _: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetInt64Array(&self, _: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetUInt64Array(&self, _: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetSingleArray(&self, _: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetDoubleArray(&self, _: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetChar16Array(&self, _: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetBooleanArray(&self, _: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetStringArray(
        &self,
        _: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetInspectableArray(
        &self,
        _: &mut windows_core::Array<IInspectable>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetGuidArray(&self, _: &mut windows_core::Array<GUID>) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetDateTimeArray(
        &self,
        _: &mut windows_core::Array<windows_time::DateTime>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetTimeSpanArray(
        &self,
        _: &mut windows_core::Array<windows_time::TimeSpan>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetPointArray(
        &self,
        _: &mut windows_core::Array<crate::fx_bindings::Point>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetSizeArray(
        &self,
        _: &mut windows_core::Array<crate::fx_bindings::Size>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
    fn GetRectArray(
        &self,
        _: &mut windows_core::Array<crate::fx_bindings::Rect>,
    ) -> windows_core::Result<()> {
        type_mismatch!()
    }
}

// ============================================================================
// `<WindhawkBlur …/>` — `ParseNonXamlPropertyOverrideValue`, routed through
// `effects::make_blur_brush_any`.
// ============================================================================

#[derive(Clone)]
struct BlurParams {
    amount: f32,
    tint: [f32; 4], // r,g,b,a 0..1
}

fn parse_blur(value: &str) -> Option<BlurParams> {
    let s = value.trim();
    let body = s.strip_prefix("<WindhawkBlur ")?.strip_suffix("/>")?;
    let mut amount = 0.0f32;
    let mut tint_rgba = [0u8, 0, 0, 0];
    let mut tint_opacity: Option<f32> = None;
    let mut theme_key: Option<String> = None;
    // Quote-aware split: `TintColor="{ThemeResource X}"` contains a space
    // inside the quoted value — a naive `split(' ')` would sever it and the
    // tint would silently degrade to transparent.
    let mut attrs: Vec<String> = Vec::new();
    {
        let mut cur = String::new();
        let mut in_q = false;
        for ch in body.chars() {
            match ch {
                '"' => {
                    in_q = !in_q;
                    cur.push(ch);
                }
                c if c.is_whitespace() && !in_q => {
                    if !cur.is_empty() {
                        attrs.push(core::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            attrs.push(cur);
        }
    }
    for part in &attrs {
        let part = part.as_str();
        if let Some(rest) = part.strip_prefix("TintColor=\"{ThemeResource ") {
            theme_key = rest.strip_suffix("}\"").map(str::to_string);
            continue;
        }
        let Some((k, v)) = part.split_once('=') else {
            continue;
        };
        match k {
            "BlurAmount" => amount = v.trim_matches('"').parse().unwrap_or(0.0),
            "TintColor" => {
                let v = v.trim_matches('"');
                if let Some(hex) = v.strip_prefix('#')
                    && let Ok(n) = u32::from_str_radix(hex, 16)
                {
                    tint_rgba = match hex.len() {
                        8 => [
                            ((n >> 16) & 0xFF) as u8,
                            ((n >> 8) & 0xFF) as u8,
                            (n & 0xFF) as u8,
                            ((n >> 24) & 0xFF) as u8,
                        ],
                        6 => [
                            ((n >> 16) & 0xFF) as u8,
                            ((n >> 8) & 0xFF) as u8,
                            (n & 0xFF) as u8,
                            255,
                        ],
                        _ => tint_rgba,
                    };
                }
            }
            "TintOpacity" => tint_opacity = v.trim_matches('"').parse().ok(),
            // LuminosityOpacity/TintSaturation/Noise*/FallbackColor have no
            // channel in our blur+tint graph — tint+blur alone still reads
            // correctly for the catalog's acrylic borders.
            _ => {}
        }
    }
    if let Some(o) = tint_opacity {
        tint_rgba[3] = (o.clamp(0.0, 1.0) * 255.0) as u8;
    }
    if let Some(key) = theme_key
        && let Some(c) = lookup_theme_resource_color(&key)
    {
        tint_rgba = c;
    }
    Some(BlurParams {
        amount,
        tint: [
            tint_rgba[0] as f32 / 255.0,
            tint_rgba[1] as f32 / 255.0,
            tint_rgba[2] as f32 / 255.0,
            tint_rgba[3] as f32 / 255.0,
        ],
    })
}

/// `{ThemeResource X}` color lookup — returns `[r,g,b,a]`.
fn lookup_theme_resource_color(key: &str) -> Option<[u8; 4]> {
    let app = xb::Application::Current().ok()?;
    let res = app.Resources().ok()?;
    let map: windows_collections::IMap<IInspectable, IInspectable> = Interface::cast(&res).ok()?;
    let key_obj =
        crate::fx_bindings::PropertyValue::CreateString(&windows_core::HSTRING::from(key)).ok()?;
    let val = map.Lookup(&key_obj).ok()?;
    let pv: crate::fx_bindings::IPropertyValue = Interface::cast(&val).ok()?;
    use crate::fx_bindings::PropertyType;
    match pv.Type().ok()? {
        PropertyType::UInt32 => {
            let n = pv.GetUInt32().ok()?;
            Some([
                ((n >> 16) & 0xFF) as u8,
                ((n >> 8) & 0xFF) as u8,
                (n & 0xFF) as u8,
                ((n >> 24) & 0xFF) as u8,
            ])
        }
        _ => None,
    }
}

// ============================================================================
// XamlReader resolution — `GetStyleFromXamlSetters` /
// `GetResolvedPropertyOverrides` / `GetResolvedPropertyValues`.
// ============================================================================

/// `EscapeXmlAttribute`.
fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// Loads a synthetic `<ResourceDictionary><Style>…</Style></ResourceDictionary>`
/// through `XamlReader`; falls back to `FrameworkElement` when the type name
/// can't be constructed (the stowed-exception path).
fn style_from_setters(
    type_name: &str,
    fallback: &str,
    setters_xaml: &str,
) -> windows_core::Result<xb::Style> {
    let wrap = |tt: &str| -> String {
        let (ns_decl, target) = match tt.rfind('.') {
            Some(dot) => (
                format!("xmlns:windhawkstyler=\"using:{}\"", escape_xml(&tt[..dot])),
                format!("windhawkstyler:{}", escape_xml(&tt[dot + 1..])),
            ),
            None => (String::new(), escape_xml(tt)),
        };
        format!(
            "<ResourceDictionary \
             xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\" \
             xmlns:x=\"http://schemas.microsoft.com/winfx/2006/xaml\" \
             xmlns:muxc=\"using:Microsoft.UI.Xaml.Controls\" {ns_decl}>\
             <Style TargetType=\"{target}\">{setters_xaml}</Style>\
             </ResourceDictionary>"
        )
    };
    let load = |xaml: &str| -> windows_core::Result<xb::Style> {
        let obj = xb::XamlReader::Load(&windows_core::HSTRING::from(xaml))?;
        let dict: xb::ResourceDictionary = Interface::cast(&obj)?;
        let iter: windows_collections::IIterable<
            windows_collections::IKeyValuePair<IInspectable, IInspectable>,
        > = Interface::cast(&dict)?;
        let pair = iter.First()?.Current()?;
        Interface::cast(&pair.Value()?)
    };
    match load(&wrap(type_name)) {
        Ok(s) => Ok(s),
        Err(e) => {
            if fallback.is_empty() || fallback == type_name {
                Err(e)
            } else {
                load(&wrap(fallback))
            }
        }
    }
}

enum StyleVal {
    /// Resolved value; `None` = `DependencyProperty.UnsetValue` -> ClearValue.
    Ready(Option<IInspectable>),
    /// `{{expr}}` template — re-resolved at apply/propagation time.
    Dynamic(String, bool),
    /// `<WindhawkBlur …/>`.
    Blur(BlurParams),
}

impl Clone for StyleVal {
    fn clone(&self) -> Self {
        match self {
            Self::Ready(v) => Self::Ready(v.clone()),
            Self::Dynamic(s, x) => Self::Dynamic(s.clone(), *x),
            Self::Blur(p) => Self::Blur(p.clone()),
        }
    }
}

struct ResolvedStyle {
    dp: DependencyProperty,
    name: String,
    is_xaml: bool,
    /// `""` = applies in every visual state.
    state: String,
    val: StyleVal,
}

/// All COM members are only touched on the XAML UI thread.
unsafe impl Send for ResolvedRuleSet {}

struct ResolvedRuleSet {
    matcher: Matcher,
    preds: Vec<(DependencyProperty, IInspectable)>,
    parents: Vec<(Matcher, Vec<(DependencyProperty, IInspectable)>)>,
    rules: Vec<ResolvedStyle>,
    captures: Vec<(DependencyProperty, String, String)>, // (dp, prop_name, var)
}

/// `Setter::Property()` returns the setter's own `DependencyProperty`
/// projection; the element APIs take this bindings' — same IID, one QI away.
fn cast_dp<T: Interface>(dp: &T) -> Option<DependencyProperty> {
    Interface::cast(dp).ok()
}

/// `GetResolvedPropertyValues` — synthetic `<Style>` with a `<Setter>` per
/// `[Prop=Value]` predicate.
fn resolve_preds(m: &Matcher) -> Vec<(DependencyProperty, IInspectable)> {
    if m.props.is_empty() {
        return Vec::new();
    }
    let mut xaml = String::new();
    for (k, v) in &m.props {
        xaml += &format!(
            "<Setter Property=\"{}\" Value=\"{}\"/>",
            escape_xml(k),
            escape_xml(v)
        );
    }
    let fallback = NS_FE;
    let Ok(style) = style_from_setters(&m.type_name, fallback, &xaml) else {
        return Vec::new();
    };
    let Ok(setters) = style.Setters() else {
        return Vec::new();
    };
    let Ok(vec) = Interface::cast::<windows_collections::IVector<xb::SetterBase>>(&setters) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for i in 0..m.props.len() as u32 {
        if let Ok(sb) = vec.GetAt(i)
            && let Ok(setter) = Interface::cast::<xb::Setter>(&sb)
            && let (Ok(dp), Ok(v)) = (setter.Property(), setter.Value())
            && let Some(dp) = cast_dp(&dp)
        {
            out.push((dp, v));
        }
    }
    out
}

/// `GetResolvedPropertyOverrides` — one `<Style>` for the whole ruleset:
/// value rules first (dynamic/capture placeholder `{x:Null}` — only the DP is
/// needed there), captures last.
fn resolve_ruleset(rs: &WireRuleSet) -> ResolvedRuleSet {
    let mut value_rules: Vec<ParsedRule> = Vec::new();
    let mut capture_props: Vec<(String, String)> = Vec::new();
    for s in &rs.styles {
        match parse_rule(s) {
            Ok(ParsedRule::Capture { prop, var }) => capture_props.push((prop, var)),
            Ok(v) => value_rules.push(v),
            Err(e) => crate::tap_log!("style {s:?} rejected: {e}"),
        }
    }
    let mut xaml = String::new();
    let mut blur_slots: Vec<Option<BlurParams>> = Vec::new();
    for r in &value_rules {
        let ParsedRule::Value {
            prop,
            is_xaml,
            value,
            ..
        } = r
        else {
            continue;
        };
        let dynamic = value.contains("{{");
        let blur = if *is_xaml && !dynamic {
            parse_blur(value)
        } else {
            None
        };
        blur_slots.push(blur.clone());
        let e = escape_xml(prop);
        if dynamic || blur.is_some() || (*is_xaml && value.is_empty()) {
            xaml += &format!("<Setter Property=\"{e}\" Value=\"{{x:Null}}\"/>");
        } else if !*is_xaml {
            xaml += &format!("<Setter Property=\"{e}\" Value=\"{}\"/>", escape_xml(value));
        } else {
            xaml +=
                &format!("<Setter Property=\"{e}\"><Setter.Value>{value}</Setter.Value></Setter>");
        }
    }
    for (prop, _) in &capture_props {
        xaml += &format!(
            "<Setter Property=\"{}\" Value=\"{{x:Null}}\"/>",
            escape_xml(prop)
        );
    }
    let mut resolved = ResolvedRuleSet {
        matcher: Matcher {
            kind: rs.matcher.kind,
            type_name: rs.matcher.type_name.clone(),
            name: rs.matcher.name.clone(),
            vsg: rs.matcher.vsg.clone(),
            index: rs.matcher.index,
            props: rs.matcher.props.clone(),
        },
        preds: resolve_preds(&rs.matcher),
        parents: rs
            .parents
            .iter()
            .map(|m| {
                (
                    Matcher {
                        kind: m.kind,
                        type_name: m.type_name.clone(),
                        name: m.name.clone(),
                        vsg: m.vsg.clone(),
                        index: m.index,
                        props: m.props.clone(),
                    },
                    resolve_preds(m),
                )
            })
            .collect(),
        rules: Vec::new(),
        captures: Vec::new(),
    };
    let Ok(style) = style_from_setters(&rs.matcher.type_name, NS_FE, &xaml) else {
        crate::tap_log!("resolve failed for {:?}", rs.matcher.type_name);
        return resolved;
    };
    let Ok(setters) = style.Setters() else {
        return resolved;
    };
    let Ok(setters) = Interface::cast::<windows_collections::IVector<xb::SetterBase>>(&setters)
    else {
        return resolved;
    };
    let mut idx = 0u32;
    for (i, r) in value_rules.iter().enumerate() {
        let ParsedRule::Value {
            prop,
            state,
            is_xaml,
            value,
        } = r
        else {
            continue;
        };
        let Ok(sb) = setters.GetAt(idx) else { break };
        idx += 1;
        let Ok(setter) = Interface::cast::<xb::Setter>(&sb) else {
            continue;
        };
        let Ok(dp) = setter.Property() else { continue };
        let Some(dp) = cast_dp(&dp) else { continue };
        let val = if value.contains("{{") {
            StyleVal::Dynamic(value.clone(), *is_xaml)
        } else if let Some(blur) = blur_slots[i].clone() {
            StyleVal::Blur(blur)
        } else if *is_xaml && value.is_empty() {
            StyleVal::Ready(None)
        } else {
            StyleVal::Ready(setter.Value().ok())
        };
        resolved.rules.push(ResolvedStyle {
            dp,
            name: prop.clone(),
            is_xaml: *is_xaml,
            state: state.clone(),
            val,
        });
    }
    for (prop, var) in &capture_props {
        let Ok(sb) = setters.GetAt(idx) else { break };
        idx += 1;
        if let Ok(setter) = Interface::cast::<xb::Setter>(&sb)
            && let Ok(dp) = setter.Property()
            && let Some(dp) = cast_dp(&dp)
        {
            resolved.captures.push((dp, prop.clone(), var.clone()));
        }
    }
    resolved
}

/// `ResolveExpandedSinglePropertyValue` — the synthetic `<Style>` uses the
/// consumer element's runtime type, with the reported type as fallback.
fn resolve_expanded(
    type_name: &str,
    fallback: &str,
    prop: &str,
    expanded: &str,
    is_xaml: bool,
) -> Option<StyleVal> {
    if is_xaml {
        if let Some(blur) = parse_blur(expanded) {
            return Some(StyleVal::Blur(blur));
        }
        if expanded.trim().is_empty() {
            return Some(StyleVal::Ready(None));
        }
    }
    let xaml = if !is_xaml {
        format!(
            "<Setter Property=\"{}\" Value=\"{}\"/>",
            escape_xml(prop),
            escape_xml(expanded)
        )
    } else {
        format!(
            "<Setter Property=\"{}\"><Setter.Value>{expanded}</Setter.Value></Setter>",
            escape_xml(prop)
        )
    };
    let fallback = if fallback.is_empty() { NS_FE } else { fallback };
    let style = style_from_setters(type_name, fallback, &xaml).ok()?;
    let vec =
        Interface::cast::<windows_collections::IVector<xb::SetterBase>>(&style.Setters().ok()?)
            .ok()?;
    let setter = Interface::cast::<xb::Setter>(&vec.GetAt(0).ok()?).ok()?;
    Some(StyleVal::Ready(setter.Value().ok()))
}

// ============================================================================
// Element matching — `TestElementMatcher` + `matchParents`
// ============================================================================

fn class_name(o: &IInspectable) -> String {
    o.GetRuntimeClassName()
        .map(|h| h.to_string_lossy())
        .unwrap_or_default()
}

/// `class_name` for any COM type (QI to `IInspectable` first).
fn class_name_el<T: Interface>(t: &T) -> String {
    Interface::cast::<IInspectable>(t)
        .and_then(|o| o.GetRuntimeClassName())
        .map(|h| h.to_string_lossy())
        .unwrap_or_default()
}

fn is_unset(v: &IInspectable) -> bool {
    match DependencyProperty::UnsetValue() {
        Ok(u) => Interface::as_raw(v) == Interface::as_raw(&u),
        Err(_) => false,
    }
}

/// `ReadLocalValueWithWorkaround` — the (type, name, property) triples that
/// return null from ReadLocalValue on SMEH get `GetValue` instead.
fn read_local(
    el: &DependencyObject,
    dp: &DependencyProperty,
    prop_name: &str,
) -> Option<IInspectable> {
    let fe: FrameworkElement = Interface::cast(el).ok()?;
    let name = fe.Name().map(|n| n.to_string_lossy()).unwrap_or_default();
    let class = class_name_el(&fe);
    let use_get = (prop_name == "Background"
        && class == "Windows.UI.Xaml.Controls.Border"
        && name == "AcrylicBorder")
        || (prop_name == "MaxHeight"
            && class == "Windows.UI.Xaml.Controls.Grid"
            && name == "FrameRoot")
        || (prop_name == "Width"
            && ((class == "Windows.UI.Xaml.Controls.Grid" && name == "MainMenu")
                || (class == "Windows.UI.Xaml.Controls.GridView" && name == "PinnedList")));
    let mut value = if use_get {
        el.GetValue(dp).ok()
    } else {
        el.ReadLocalValue(dp).ok()
    };
    if !use_get
        && value.as_ref().map(is_unset).unwrap_or(true)
        && class == "Windows.UI.Xaml.Controls.Grid"
        && name == "SideBySidePinnedWrapper"
        && let Ok(v2) = el.GetValue(dp)
        && class_name(&v2) == "Windows.UI.Xaml.Controls.ColumnDefinitionCollection"
    {
        value = Some(v2);
    }
    // `BindingExpression` values can't be SetValue'd back — take the base.
    if let Some(v) = &value {
        let cn = class_name(v);
        if (cn.ends_with(".Data.BindingExpressionBase") || cn.ends_with(".Data.BindingExpression"))
            && let Ok(base) = el.GetAnimationBaseValue(dp)
        {
            value = Some(base);
        }
    }
    value
}

/// `TestElementMatcher` — `fallback` is the reported `element.Type` for the
/// leaf (a second alias for the type check); `""` for parents.
fn test_matcher(
    el: &FrameworkElement,
    m: &Matcher,
    preds: &[(DependencyProperty, IInspectable)],
    fallback: &str,
) -> bool {
    if m.kind != MKind::Element {
        return false;
    }
    if !m.type_name.is_empty()
        && m.type_name != class_name_el(el)
        && (fallback.is_empty() || m.type_name != fallback)
    {
        return false;
    }
    if !m.name.is_empty() && el.Name().map(|n| n.to_string_lossy()).unwrap_or_default() != m.name {
        return false;
    }
    if m.index > 0 {
        let dobj: DependencyObject = match Interface::cast(el) {
            Ok(d) => d,
            Err(_) => return false,
        };
        let Ok(parent) = VisualTreeHelper::GetParent(&dobj) else {
            return false;
        };
        let i = m.index - 1;
        if i < 0
            || VisualTreeHelper::GetChildrenCount(&parent)
                .map(|c| i >= c)
                .unwrap_or(true)
            || VisualTreeHelper::GetChild(&parent, i)
                .map(|c| Interface::as_raw(&c) != Interface::as_raw(&dobj))
                .unwrap_or(true)
        {
            return false;
        }
    }
    let dobj: DependencyObject = match Interface::cast(el) {
        Ok(d) => d,
        Err(_) => return false,
    };
    for (dp, expected) in preds {
        let Some(value) = read_local(&dobj, dp, "") else {
            return false;
        };
        if is_unset(&value) {
            return false;
        }
        let (Some(e), Some(v)) = (unbox(expected), unbox(&value)) else {
            return false;
        };
        let same = match (&e, &v) {
            (Unboxed::Str(a), Unboxed::Str(b)) => a == b,
            (Unboxed::Bool(a), Unboxed::Bool(b)) => a == b,
            _ => match (e.as_num(), v.as_num()) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
        };
        if !same {
            return false;
        }
    }
    true
}

/// `matchParents` — the recursive wildcard-backtracking walk. Returns the
/// ancestor each parent matcher was tested against (index-aligned with
/// `parents`; `None` for `*`/`:root` which test nothing) — a `@vsg` on a
/// parent matcher resolves its group on **that** ancestor, per
/// `TestElementMatcher`'s `GetVisualStateGroup(element, name)`.
fn match_parents(
    el: &FrameworkElement,
    parents: &[(Matcher, Vec<(DependencyProperty, IInspectable)>)],
) -> Option<Vec<Option<FrameworkElement>>> {
    fn go(
        el: &FrameworkElement,
        parents: &[(Matcher, Vec<(DependencyProperty, IInspectable)>)],
        mi: usize,
        out: &mut Vec<Option<FrameworkElement>>,
    ) -> bool {
        if mi >= parents.len() {
            return true;
        }
        let (m, preds) = &parents[mi];
        match m.kind {
            MKind::Root => {
                let dobj: DependencyObject = match Interface::cast(el) {
                    Ok(d) => d,
                    Err(_) => return false,
                };
                if VisualTreeHelper::GetParent(&dobj).is_ok() {
                    return false;
                }
                go(el, parents, mi + 1, out)
            }
            MKind::Wildcard => {
                let mut cur: FrameworkElement = el.clone();
                loop {
                    let dobj: DependencyObject = match Interface::cast(&cur) {
                        Ok(d) => d,
                        Err(_) => return false,
                    };
                    let Ok(parent) = VisualTreeHelper::GetParent(&dobj)
                        .and_then(|p| Interface::cast::<FrameworkElement>(&p))
                    else {
                        return false;
                    };
                    cur = parent;
                    let fb = reported_fallback(&cur);
                    if test_matcher(&cur, &parents[mi + 1].0, &parents[mi + 1].1, &fb)
                        && go(&cur, parents, mi + 2, out)
                    {
                        out[mi + 1] = Some(cur);
                        return true;
                    }
                }
            }
            MKind::Element => {
                let dobj: DependencyObject = match Interface::cast(el) {
                    Ok(d) => d,
                    Err(_) => return false,
                };
                let Ok(parent) = VisualTreeHelper::GetParent(&dobj)
                    .and_then(|p| Interface::cast::<FrameworkElement>(&p))
                else {
                    return false;
                };
                let fb = reported_fallback(&parent);
                if test_matcher(&parent, m, preds, &fb) && go(&parent, parents, mi + 1, out) {
                    out[mi] = Some(parent);
                    return true;
                }
                false
            }
        }
    }
    let mut out = vec![None; parents.len()];
    go(el, parents, 0, &mut out).then_some(out)
}

// ============================================================================
// `SetOrClearValue` + per-element apply bookkeeping
// ============================================================================

/// `SetOrClearValue` — returns the applied value (`lastAppliedValue`), or
/// `None` when the property was cleared / the write was deferred.
/// `initial_apply` gates the AllAppsRoot delay workaround.
fn set_or_clear(
    el: &DependencyObject,
    dp: &DependencyProperty,
    prop_name: &str,
    val: &StyleVal,
    initial_apply: bool,
) -> Option<IInspectable> {
    let value: Option<IInspectable> = match val {
        StyleVal::Ready(v) => v.clone(),
        StyleVal::Blur(p) => {
            let Ok(ui) = Interface::cast::<UIElement>(el) else {
                return None;
            };
            match fxe::make_blur_brush_any(&ui, p.amount, p.tint) {
                Ok(b) => Interface::cast(&b).ok(),
                Err(e) => {
                    crate::tap_log!("blur brush failed: {e:?}");
                    return None;
                }
            }
        }
        StyleVal::Dynamic(..) => return None,
    };
    let Some(mut value) = value else {
        MODIFYING.store(true, Ordering::Relaxed);
        let r = el.ClearValue(dp);
        MODIFYING.store(false, Ordering::Relaxed);
        if let Err(e) = r {
            crate::tap_log!("ClearValue {prop_name}: {e:?}");
        }
        return None;
    };
    if is_unset(&value) {
        MODIFYING.store(true, Ordering::Relaxed);
        let _ = el.ClearValue(dp);
        MODIFYING.store(false, Ordering::Relaxed);
        return None;
    }
    // AllAppsRoot Visibility must be deferred — setting it inside the apply
    // sweep truncates the all-apps list (windhawk-mods#1335).
    if initial_apply && prop_name == "Visibility" {
        if let Ok(fe) = Interface::cast::<FrameworkElement>(el)
            && class_name_el(&fe) == "Windows.UI.Xaml.Controls.Grid"
            && fe
                .Name()
                .map(|n| n.to_string_lossy() == "AllAppsRoot")
                .unwrap_or(false)
            && let Some(disp) = elem_dispatcher(&fe)
        {
            let payload = UiBound((el.clone(), dp.clone(), value.clone()));
            post_ui(&disp, Prio::High, move || {
                // Move `payload` wholesale — field paths would capture the
                // tuple's !Send COM objects instead of the `Send` wrapper.
                let p = &payload;
                let (el_c, dp_c, v_c) = &p.0;
                MODIFYING.store(true, Ordering::Relaxed);
                let _ = el_c.SetValue(dp_c, v_c);
                MODIFYING.store(false, Ordering::Relaxed);
                Ok(())
            });
            return None;
        }
    }
    // `Setter.Value()` hands FontWeight back as a bare int; `SetValue` wants
    // `IReference<FontWeight>` — rebox (the mod's own workaround).
    if prop_name == "FontWeight"
        && let Ok(pv) = Interface::cast::<crate::fx_bindings::IPropertyValue>(&value)
        && let Ok(w) = pv.GetInt32()
    {
        let boxed: IFontWeightRef = FontWeightBox(w as u16).into();
        if let Ok(v) = Interface::cast(&boxed) {
            value = v;
        }
    }
    // Column/RowDefinition collections are single-owner — clone per element.
    let cn = class_name(&value);
    if (cn.ends_with(".Controls.ColumnDefinitionCollection")
        || cn.ends_with(".Controls.RowDefinitionCollection"))
        && let Some(cloned) = clone_definitions(&value)
    {
        value = cloned;
    }
    MODIFYING.store(true, Ordering::Relaxed);
    let r = el.SetValue(dp, &value);
    MODIFYING.store(false, Ordering::Relaxed);
    if let Err(e) = r {
        crate::tap_log!("SetValue {prop_name}: {e:?}");
        return None;
    }
    Some(value)
}

/// Per-element clones for definition collections (the mod clones through a
/// scratch `Grid` — same here).
fn clone_definitions(value: &IInspectable) -> Option<IInspectable> {
    let grid = xb::Grid::new().ok()?;
    if let Ok(cols) = Interface::cast::<xb::ColumnDefinitionCollection>(value) {
        let target = grid.ColumnDefinitions().ok()?;
        let iter: windows_collections::IIterable<xb::ColumnDefinition> =
            Interface::cast(&cols).ok()?;
        for c in iter {
            let cloned = xb::ColumnDefinition::new().ok()?;
            let _ = cloned.SetWidth(c.Width().unwrap_or_default());
            let _ = cloned.SetMinWidth(c.MinWidth().unwrap_or(0.0));
            let _ = cloned.SetMaxWidth(c.MaxWidth().unwrap_or(f64::INFINITY));
            let vec: windows_collections::IVector<xb::ColumnDefinition> =
                Interface::cast(&target).ok()?;
            let _ = vec.Append(&cloned);
        }
        return Interface::cast(&target).ok();
    }
    if let Ok(rows) = Interface::cast::<xb::RowDefinitionCollection>(value) {
        let target = grid.RowDefinitions().ok()?;
        let iter: windows_collections::IIterable<xb::RowDefinition> =
            Interface::cast(&rows).ok()?;
        for r in iter {
            let cloned = xb::RowDefinition::new().ok()?;
            let _ = cloned.SetHeight(r.Height().unwrap_or_default());
            let _ = cloned.SetMinHeight(r.MinHeight().unwrap_or(0.0));
            let _ = cloned.SetMaxHeight(r.MaxHeight().unwrap_or(f64::INFINITY));
            let vec: windows_collections::IVector<xb::RowDefinition> =
                Interface::cast(&target).ok()?;
            let _ = vec.Append(&cloned);
        }
        return Interface::cast(&target).ok();
    }
    None
}

/// Per-(element, dp) bookkeeping — keyed `(vsg_name, dp_raw)` so the same DP
/// can live in different visual-state buckets (`ElementPropertyCustomizationState`).
struct PropSlot {
    /// Value before our first write — recorded verbatim, `UnsetValue` box
    /// included; `None` = the read failed (nothing to restore).
    original: Option<IInspectable>,
    /// The value the property should currently show (as applied).
    last_applied: Option<IInspectable>,
    /// The resolved value to re-apply on host stomp (`customValue`).
    custom: Option<StyleVal>,
    /// Re-apply registration token (0 = none).
    token: i64,
    /// `state -> resolved value` (for VSG state-change re-evaluation).
    state_map: BTreeMap<String, StyleVal>,
    /// Active dynamic template + its `(name, owner)` dependencies.
    dyn_template: Option<(String, bool)>,
    deps: Vec<Dep>,
    /// Expansion/resolution failed last time — any variable change retries.
    last_resolve_failed: bool,
}

struct CaptureSlot {
    dp: Option<DependencyProperty>,
    var: String,
    token: i64,
}

struct VsgSub {
    vsg: xb::VisualStateGroup,
    #[allow(dead_code)]
    revoker: Option<windows_core::EventRevoker>,
}

struct ElemState {
    el: FrameworkElement,
    /// The reported `element.Type` — `fallbackClassName` for re-resolves.
    fallback: String,
    /// `(vsg_name, dp_raw) -> slot`.
    props: BTreeMap<(String, usize), PropSlot>,
    /// `dp_raw -> (dp, prop_name)` lookup for callbacks.
    dp_meta: BTreeMap<usize, (DependencyProperty, String)>,
    captures: Vec<CaptureSlot>,
    vsg_subs: Vec<VsgSub>,
    size_revoker: Option<windows_core::EventRevoker>,
    /// The element's spine node — built lazily on first use.
    tree_node: Option<Rc<TreeNode>>,
}

fn get_vsg(el: &FrameworkElement, name: &str) -> Option<xb::VisualStateGroup> {
    let sel: xb::FrameworkElement = Interface::cast(el).ok()?;
    let groups = xb::VisualStateManager::GetVisualStateGroups(&sel).ok()?;
    for v in &groups {
        if v.Name().map(|n| n.to_string_lossy()).unwrap_or_default() == name {
            return Some(v);
        }
    }
    None
}

/// `IsLayoutDrivenSizeProperty` — DP-changed callbacks don't fire for these;
/// `SizeChanged` is the notification source instead.
fn is_layout_driven(prop_name: &str) -> bool {
    matches!(prop_name, "ActualWidth" | "ActualHeight")
}

/// `UpdateStyleVariableConsumers` — remove this `(handle, dp)` from every
/// `old_deps` name; add it to every `new_deps` name (with the consumer's own
/// fallback + scope).
fn update_consumers(
    consumers: &mut BTreeMap<String, Vec<Consumer>>,
    handle: u64,
    dp_raw: usize,
    fallback: &str,
    scope: usize,
    old_deps: &[Dep],
    new_deps: &[Dep],
) {
    for (name, _) in old_deps {
        let Some(list) = consumers.get_mut(name) else {
            continue;
        };
        list.retain(|c| !(c.handle == handle && c.dp_raw == dp_raw));
        if list.is_empty() {
            consumers.remove(name);
        }
    }
    for (name, _) in new_deps {
        let list = consumers.entry(name.clone()).or_default();
        if !list
            .iter()
            .any(|c| c.handle == handle && c.dp_raw == dp_raw)
        {
            list.push(Consumer {
                handle,
                dp_raw,
                fallback: fallback.to_string(),
                scope,
            });
        }
    }
}

/// `SameNumericValue` — NaN compares equal (a NaN capture like Height=Auto
/// must not report "changed" on every read).
fn same_numeric(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x == y || (x.is_nan() && y.is_nan()),
        _ => false,
    }
}

/// `SameLocalValue` — unboxed values compare by content (XAML re-boxes on
/// every read); anything else boxed (incl. any `IReference<T>` struct box)
/// counts as unchanged; reference types compare by identity.
fn same_local_value(a: &Option<IInspectable>, b: &Option<IInspectable>) -> bool {
    let (a, b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        (None, None) => return true,
        _ => return false,
    };
    if Interface::as_raw(a) == Interface::as_raw(b) {
        return true;
    }
    let (ua, ub) = (unbox(a), unbox(b));
    if ua.is_some() || ub.is_some() {
        return match (&ua, &ub) {
            (Some(x), Some(y)) => match (x, y) {
                (Unboxed::Str(p), Unboxed::Str(q)) => p == q,
                (Unboxed::Bool(p), Unboxed::Bool(q)) => p == q,
                _ => {
                    same_numeric(x.as_num(), y.as_num())
                        && matches!(
                            (x, y),
                            (Unboxed::Num(_), Unboxed::Num(_))
                                | (Unboxed::Bool(_), Unboxed::Bool(_))
                        )
                }
            },
            _ => false,
        };
    }
    let is_boxed = |v: &IInspectable| {
        Interface::cast::<crate::fx_bindings::IPropertyValue>(v).is_ok()
            || class_name(v).starts_with("Windows.Foundation.IReference`1<")
    };
    is_boxed(a) && is_boxed(b)
}

/// `ReadCapturedStyleVariableValue` — the EFFECTIVE value (`GetValue`), so
/// layout-driven DPs (ActualWidth/Height) capture too.
fn read_captured(el: &FrameworkElement, dp: &DependencyProperty) -> CapVal {
    let empty = CapVal {
        string_form: String::new(),
        numeric: None,
        substitutable: false,
    };
    let Ok(dobj) = Interface::cast::<DependencyObject>(el) else {
        return empty;
    };
    let Ok(v) = dobj.GetValue(dp) else {
        return empty;
    };
    if is_unset(&v) {
        return empty;
    }
    match unbox(&v) {
        Some(u) => CapVal {
            string_form: u.to_text(),
            numeric: u.as_num(),
            substitutable: true,
        },
        None => CapVal {
            string_form: class_name(&v),
            numeric: None,
            substitutable: false,
        },
    }
}

/// `AdoptExternalValueAsOriginal` — a local value not matching our last write
/// came from elsewhere; it becomes the new pre-style value.
fn adopt_external(handle: u64, vsg: &str, dp_raw: usize) {
    let (el, dp, name) = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            return;
        };
        let Some(slot) = es.props.get(&(vsg.to_string(), dp_raw)) else {
            return;
        };
        if slot.custom.is_none() {
            return;
        }
        let Some((dp, name)) = es.dp_meta.get(&dp_raw) else {
            return;
        };
        (es.el.clone(), dp.clone(), name.clone())
    };
    let dobj: DependencyObject = match Interface::cast(&el) {
        Ok(d) => d,
        Err(_) => return,
    };
    let local = read_local(&dobj, &dp, &name);
    let changed = {
        let eng = state();
        let Some(slot) = eng
            .elements
            .get(&handle)
            .and_then(|e| e.props.get(&(vsg.to_string(), dp_raw)))
        else {
            return;
        };
        !same_local_value(&local, &slot.last_applied)
    };
    if changed {
        let mut eng = state();
        if let Some(slot) = eng
            .elements
            .get_mut(&handle)
            .and_then(|e| e.props.get_mut(&(vsg.to_string(), dp_raw)))
        {
            slot.original = local;
        }
    }
}

/// `UnapplyStyleValue` — adopt external writes (only when we hold a custom
/// value), restore the pre-style value, forget what was applied; the dynamic
/// template stays so a later variable change reapplies.
fn unapply_slot(handle: u64, vsg: &str, dp_raw: usize) {
    let key = (vsg.to_string(), dp_raw);
    let (el, dp, name) = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            return;
        };
        let Some(slot) = es.props.get(&key) else {
            return;
        };
        if slot.custom.is_none() && slot.original.is_none() {
            return;
        }
        let Some((dp, name)) = es.dp_meta.get(&dp_raw) else {
            return;
        };
        (es.el.clone(), dp.clone(), name.clone())
    };
    let dobj: DependencyObject = match Interface::cast(&el) {
        Ok(d) => d,
        Err(_) => return,
    };
    // AdoptExternalValueAsOriginal — gated on a custom value existing.
    let local = read_local(&dobj, &dp, &name);
    let original = {
        let mut eng = state();
        let Some(slot) = eng
            .elements
            .get_mut(&handle)
            .and_then(|e| e.props.get_mut(&key))
        else {
            return;
        };
        if slot.custom.is_some() && !same_local_value(&local, &slot.last_applied) {
            slot.original = local;
        }
        slot.original.clone()
    };
    if let Some(orig) = &original {
        set_or_clear(
            &dobj,
            &dp,
            &name,
            &StyleVal::Ready(Some(orig.clone())),
            false,
        );
    }
    let mut eng = state();
    if let Some(slot) = eng
        .elements
        .get_mut(&handle)
        .and_then(|e| e.props.get_mut(&key))
    {
        slot.original = None;
        slot.last_applied = None;
        slot.custom = None;
    }
}

/// `ExpandStyleVariables` + `ResolveExpandedSinglePropertyValue` for one
/// dynamic template — the pure half of `ResolveDynamicStyleValue`. `deps`
/// receives the freshly scanned dependencies even on failure (the mod commits
/// them before resolving). `Err(())` = `skip()` reached.
fn eval_dynamic(
    node: Option<&Rc<TreeNode>>,
    scope: usize,
    el_type: &str,
    fallback: &str,
    prop: &str,
    raw: &str,
    is_xaml: bool,
    deps: &mut Vec<Dep>,
) -> Result<Option<StyleVal>, ()> {
    let expanded = expand_variables(raw, node, scope, deps).map_err(|_| ())?;
    let Some(text) = expanded else {
        return Ok(None);
    };
    Ok(resolve_expanded(el_type, fallback, prop, &text, is_xaml))
}

/// `ResolveDynamicStyleValue` — re-evaluate the slot's stored template,
/// commit the new dependency set, handle skip/unapply/fail bookkeeping.
/// Returns the resolved value when one was produced.
fn resolve_dyn(handle: u64, vsg: &str, dp_raw: usize) -> Option<StyleVal> {
    let key = (vsg.to_string(), dp_raw);
    let (el, _dp, name, fallback, node, scope, raw, is_xaml, old_deps) = {
        let mut eng = state();
        let Some(es) = eng.elements.get_mut(&handle) else {
            return None;
        };
        let node = ensure_node(es);
        let Some((dp, name)) = es.dp_meta.get(&dp_raw).cloned() else {
            return None;
        };
        let Some(slot) = es.props.get_mut(&key) else {
            return None;
        };
        let Some((raw, is_xaml)) = slot.dyn_template.clone() else {
            return None;
        };
        let fallback = es.fallback.clone();
        (
            es.el.clone(),
            dp,
            name,
            fallback,
            node.clone(),
            scope_of(node.as_ref()),
            raw,
            is_xaml,
            std::mem::take(&mut slot.deps),
        )
    };
    let mut new_deps = Vec::new();
    let el_type = class_name_el(&el);
    let r = eval_dynamic(
        node.as_ref(),
        scope,
        &el_type,
        &fallback,
        &name,
        &raw,
        is_xaml,
        &mut new_deps,
    );
    {
        let mut eng = state();
        update_consumers(
            &mut eng.consumers,
            handle,
            dp_raw,
            &fallback,
            scope,
            &old_deps,
            &new_deps,
        );
        if let Some(slot) = eng
            .elements
            .get_mut(&handle)
            .and_then(|e| e.props.get_mut(&key))
        {
            slot.deps = new_deps;
            slot.last_resolve_failed = matches!(r, Ok(None));
        }
    }
    match r {
        Err(()) => {
            // skip() — put the property back; the template lives on.
            unapply_slot(handle, vsg, dp_raw);
            None
        }
        Ok(v) => v,
    }
}

/// `commit_resolved` — the shared "resolved value -> element" tail of
/// propagate/vsg-apply: adopt external writes, record the original on first
/// write, apply, record custom + last-applied.
fn commit_resolved(handle: u64, vsg: &str, dp_raw: usize, val: StyleVal) {
    let key = (vsg.to_string(), dp_raw);
    let (el, dp, name) = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            return;
        };
        let Some((dp, name)) = es.dp_meta.get(&dp_raw).cloned() else {
            return;
        };
        (es.el.clone(), dp, name)
    };
    let dobj: DependencyObject = match Interface::cast(&el) {
        Ok(d) => d,
        Err(_) => return,
    };
    adopt_external(handle, vsg, dp_raw);
    let need_orig = {
        let eng = state();
        eng.elements
            .get(&handle)
            .and_then(|e| e.props.get(&key))
            .is_some_and(|s| s.original.is_none())
    };
    if need_orig {
        let local = read_local(&dobj, &dp, &name);
        let mut eng = state();
        if let Some(slot) = eng
            .elements
            .get_mut(&handle)
            .and_then(|e| e.props.get_mut(&key))
        {
            slot.original = local;
        }
    }
    let last = set_or_clear(&dobj, &dp, &name, &val, false);
    let mut eng = state();
    if let Some(slot) = eng
        .elements
        .get_mut(&handle)
        .and_then(|e| e.props.get_mut(&key))
    {
        slot.custom = Some(val);
        slot.last_applied = last;
    }
}

/// Registers the "host stomped our value -> re-apply" callback on a dp.
/// Callbacks identify the element by handle — the sender isn't needed.
fn register_reapply(handle: u64, el: &DependencyObject, dp: &DependencyProperty) -> i64 {
    let dp_c = dp.clone();
    el.RegisterPropertyChangedCallback(
        dp,
        &DependencyPropertyChangedCallback::new(move |_, _| {
            if !MODIFYING.load(Ordering::Relaxed) {
                reapply_dp(handle, &dp_c);
            }
            Ok(())
        }),
    )
    .unwrap_or(0)
}

/// The host wrote the property itself — adopt the external value as the new
/// original, then put our `customValue` back.
fn reapply_dp(handle: u64, dp: &DependencyProperty) {
    let dp_raw = Interface::as_raw(dp) as usize;
    let (el, vsg_name, dp_c, name, custom) = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            return;
        };
        let Some(((vn, _), slot)) = es.props.iter().find(|((_, d), _)| *d == dp_raw) else {
            return;
        };
        let Some(custom) = slot.custom.clone() else {
            return;
        };
        let Some((dp, name)) = es.dp_meta.get(&dp_raw) else {
            return;
        };
        (es.el.clone(), vn.clone(), dp.clone(), name.clone(), custom)
    };
    let dobj: DependencyObject = match Interface::cast(&el) {
        Ok(d) => d,
        Err(_) => return,
    };
    adopt_external(handle, &vsg_name, dp_raw);
    let last = set_or_clear(&dobj, &dp_c, &name, &custom, false);
    let mut eng = state();
    if let Some(slot) = eng
        .elements
        .get_mut(&handle)
        .and_then(|e| e.props.get_mut(&(vsg_name, dp_raw)))
    {
        slot.last_applied = last;
    }
}

/// `SetStyleVariableIfChangedAndPropagate` — re-read the capture, compare
/// against its own previous value, notify dependents on change.
fn capture_changed(handle: u64, var: &str, dp: &DependencyProperty) {
    let (el, scope) = {
        let mut eng = state();
        let Some(es) = eng.elements.get_mut(&handle) else {
            return;
        };
        let node = ensure_node(es);
        (es.el.clone(), scope_of(node.as_ref()))
    };
    let val = read_captured(&el, dp);
    {
        let mut eng = state();
        let Some(caps) = eng.variables.get_mut(var) else {
            return;
        };
        let Some(c) = caps.iter_mut().find(|c| c.owner == handle) else {
            return;
        };
        if c.val.string_form == val.string_form
            && same_numeric(c.val.numeric, val.numeric)
            && c.val.substitutable == val.substitutable
        {
            return;
        }
        c.val = val;
    }
    propagate(var, Some(handle), scope);
}

/// `PropagateStyleVariableChange` — nested calls queue; the outermost frame
/// drains (coalescing bursts into one pass). 32 rounds cap against oscillation.
fn propagate(var: &str, changed_owner: Option<u64>, scope: usize) {
    if PROP_DEPTH.with(|d| d.get()) > 0 {
        PROP_QUEUE.with(|q| {
            let entry = (var.to_string(), changed_owner, scope);
            let mut q = q.borrow_mut();
            if !q.contains(&entry) {
                q.push(entry);
            }
        });
        return;
    }
    PROP_DEPTH.with(|d| d.set(1));
    propagate_core(var, changed_owner, scope);
    for round in 0..32 {
        let pending = PROP_QUEUE.with(|q| core::mem::take(&mut *q.borrow_mut()));
        if pending.is_empty() {
            break;
        }
        if round >= 31 {
            crate::tap_log!(
                "style variables did not settle; dropping {} queued update(s)",
                pending.len()
            );
            break;
        }
        for (v, o, s) in pending {
            propagate_core(&v, o, s);
        }
    }
    PROP_DEPTH.with(|d| d.set(0));
}

/// `PropagateStyleVariableChangeCore` — re-evaluate each affected consumer.
fn propagate_core(var: &str, changed_owner: Option<u64>, scope: usize) {
    let consumers: Vec<Consumer> = state()
        .consumers
        .get(var)
        .map(|v| v.iter().filter(|c| c.scope == scope).cloned().collect())
        .unwrap_or_default();
    // Ranking varies per consumer — snapshot captures once for the pass
    // (only needed when the capture SET changed, not on a value change).
    let candidates = if changed_owner.is_none() {
        capture_candidates(var, scope)
    } else {
        Vec::new()
    };
    for consumer in consumers {
        let (el, node) = {
            let mut eng = state();
            let Some(es) = eng.elements.get_mut(&consumer.handle) else {
                continue;
            };
            let node = ensure_node(es);
            (es.el.clone(), node)
        };
        let winning_owner = match changed_owner {
            Some(_) => 0,
            None => pick_winning_owner(&candidates, node.as_ref()),
        };
        // Collect the slots for this dp that are affected (mod iterates the
        // element's whole per-VSG map).
        let jobs: Vec<(
            String,
            usize,
            DependencyProperty,
            String,
            String,
            bool,
            Vec<Dep>,
        )> = {
            let eng = state();
            let Some(es) = eng.elements.get(&consumer.handle) else {
                continue;
            };
            es.props
                .iter()
                .filter(|((_, d), s)| *d == consumer.dp_raw && s.dyn_template.is_some())
                .filter(|(_, s)| {
                    if s.last_resolve_failed {
                        return true;
                    }
                    s.deps.iter().any(|(n, o)| {
                        n == var
                            && match changed_owner {
                                Some(co) => *o == co,
                                None => *o != winning_owner,
                            }
                    })
                })
                .filter_map(|((vn, d), s)| {
                    let (raw, ix) = s.dyn_template.clone()?;
                    let (dp, name) = es.dp_meta.get(d)?.clone();
                    Some((vn.clone(), *d, dp, name, raw, ix, s.deps.clone()))
                })
                .collect()
        };
        if jobs.is_empty() {
            continue;
        }
        let el_type = class_name_el(&el);
        let scope = scope_of(node.as_ref());
        for (vn, dp_raw, dp, name, raw, ix, old_deps) in jobs {
            let mut new_deps = Vec::new();
            let r = eval_dynamic(
                node.as_ref(),
                scope,
                &el_type,
                &consumer.fallback,
                &name,
                &raw,
                ix,
                &mut new_deps,
            );
            let key = (vn.clone(), dp_raw);
            {
                let mut eng = state();
                update_consumers(
                    &mut eng.consumers,
                    consumer.handle,
                    dp_raw,
                    &consumer.fallback,
                    scope,
                    &old_deps,
                    &new_deps,
                );
                if let Some(slot) = eng
                    .elements
                    .get_mut(&consumer.handle)
                    .and_then(|e| e.props.get_mut(&key))
                {
                    slot.deps = new_deps;
                    slot.last_resolve_failed = matches!(r, Ok(None));
                }
            }
            match r {
                Err(()) => unapply_slot(consumer.handle, &vn, dp_raw),
                Ok(None) => {}
                Ok(Some(v)) => {
                    let _ = (dp, name); // identities already live in dp_meta
                    commit_resolved(consumer.handle, &vn, dp_raw, v);
                }
            }
        }
    }
}

/// `ApplyCustomizationsForVisualStateGroup`'s `CurrentStateChanged` — each
/// property picks `map[new]`, falls back to `map[""]` only when the OLD state
/// had a rule, else unapplies. Dynamic picks re-resolve (with skip/unapply);
/// static picks drop the dynamic bookkeeping first.
fn on_vsg_state_changed(handle: u64, vsg_name: String, old_state: String, new_state: String) {
    MODIFYING.store(true, Ordering::Relaxed);
    // Collect (dp_raw, action) under lock.
    let jobs: Vec<(usize, StyleVal)> = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            MODIFYING.store(false, Ordering::Relaxed);
            return;
        };
        let mut jobs = Vec::new();
        for ((vn, dp_raw), slot) in es.props.iter() {
            if *vn != vsg_name {
                continue;
            }
            let pick = slot.state_map.get(&new_state).cloned().or_else(|| {
                if slot.state_map.contains_key(&old_state) {
                    slot.state_map.get("").cloned()
                } else {
                    None
                }
            });
            if let Some(v) = pick {
                jobs.push((*dp_raw, v));
            }
        }
        jobs
    };
    // Slots with no applicable rule in the new state → unapply. A `""` rule
    // without an old-state rule is the `continue` case (applies constantly —
    // nothing to do), NOT unapply.
    let unapply: Vec<usize> = {
        let eng = state();
        let Some(es) = eng.elements.get(&handle) else {
            MODIFYING.store(false, Ordering::Relaxed);
            return;
        };
        es.props
            .iter()
            .filter(|((vn, _), s)| {
                *vn == vsg_name
                    && !s.state_map.contains_key(&new_state)
                    && !s.state_map.contains_key("")
            })
            .map(|((_, d), _)| *d)
            .collect()
    };
    for (dp_raw, val) in jobs {
        let resolved = match &val {
            StyleVal::Dynamic(raw, ix) => {
                // Fresh template into the slot, then resolve.
                {
                    let mut eng = state();
                    if let Some(slot) = eng
                        .elements
                        .get_mut(&handle)
                        .and_then(|e| e.props.get_mut(&(vsg_name.clone(), dp_raw)))
                    {
                        slot.dyn_template = Some((raw.clone(), *ix));
                    }
                }
                resolve_dyn(handle, &vsg_name, dp_raw)
            }
            _ => {
                // Dynamic -> static transition: drop consumer registrations.
                let cleared = {
                    let mut eng = state();
                    eng.elements
                        .get_mut(&handle)
                        .and_then(|e| e.props.get_mut(&(vsg_name.clone(), dp_raw)))
                        .map(|s| {
                            let old = std::mem::take(&mut s.deps);
                            s.dyn_template = None;
                            s.last_resolve_failed = false;
                            old
                        })
                };
                if let Some(old) = cleared {
                    let mut eng = state();
                    update_consumers(&mut eng.consumers, handle, dp_raw, "", 0, &old, &[]);
                }
                Some(val)
            }
        };
        if let Some(v) = resolved {
            commit_resolved(handle, &vsg_name, dp_raw, v);
        }
    }
    for dp_raw in unapply {
        // Clear dynamic bookkeeping, then put the original back.
        let cleared = {
            let mut eng = state();
            eng.elements
                .get_mut(&handle)
                .and_then(|e| e.props.get_mut(&(vsg_name.clone(), dp_raw)))
                .map(|s| {
                    let old = std::mem::take(&mut s.deps);
                    s.dyn_template = None;
                    s.last_resolve_failed = false;
                    old
                })
        };
        if let Some(old) = cleared {
            let mut eng = state();
            update_consumers(&mut eng.consumers, handle, dp_raw, "", 0, &old, &[]);
        }
        unapply_slot(handle, &vsg_name, dp_raw);
    }
    MODIFYING.store(false, Ordering::Relaxed);
}

/// `ApplyStyleAndVisualStates` for one element: bucket rules by the VSG the
/// match resolved (a parent's `@vsg` resolves on the matched ancestor —
/// `TestElementMatcher` calls `GetVisualStateGroup` on the element it
/// tested), pick the current state's value, apply, subscribe.
/// `fallback_type` = the reported `element.Type` (`fallbackClassName`).
fn apply_element(
    handle: u64,
    el: &FrameworkElement,
    fallback_type: &str,
    matched: &[(usize, Vec<Option<FrameworkElement>>)],
    eng_rules: &[ResolvedRuleSet],
) {
    // Phase A — gather buckets keyed by (vsg name, owning element). The mod's
    // `overridesPerVSG` is keyed by the resolved VisualStateGroup object:
    // the last `@vsg` write wins during matching, i.e. the FARTHEST
    // ancestor's, else the leaf's. Properties dedupe GLOBALLY by DP (first
    // in reverse order wins); all of the winning ruleset's `@state` entries
    // still merge into its state map. Captures dedupe by variable name.
    // Bucket key: (name, owner identity) — `0` = the styled element itself.
    let mut buckets: BTreeMap<
        (String, usize),
        (
            Option<FrameworkElement>,
            Vec<(
                usize,
                DependencyProperty,
                String,
                BTreeMap<String, StyleVal>,
            )>,
        ),
    > = BTreeMap::new();
    let mut capture_specs: Vec<(DependencyProperty, String, String)> = Vec::new();
    {
        let mut seen_dps: BTreeSet<usize> = BTreeSet::new();
        for &(ri, ref ancestors) in matched {
            let rs = &eng_rules[ri];
            let (mut vsg_name, mut vsg_owner): (String, Option<FrameworkElement>) =
                if rs.matcher.vsg.is_empty() {
                    (String::new(), None)
                } else {
                    (rs.matcher.vsg.clone(), Some(el.clone()))
                };
            for (i, (m, _)) in rs.parents.iter().enumerate() {
                if !m.vsg.is_empty() {
                    vsg_name = m.vsg.clone();
                    vsg_owner = ancestors.get(i).cloned().flatten();
                }
            }
            let owner_key = vsg_owner
                .as_ref()
                .and_then(|o| Interface::cast::<DependencyObject>(o).ok())
                .and_then(|d| identity_key(&d))
                .unwrap_or(0);
            let (_, entries) = buckets
                .entry((vsg_name, owner_key))
                .or_insert_with(|| (vsg_owner, Vec::new()));
            // `seen_dps` claims a DP for the whole element; `local` claims it
            // for THIS ruleset so its other @state entries still merge.
            let mut local: HashMap<usize, usize> = HashMap::new();
            for r in &rs.rules {
                let dp_raw = Interface::as_raw(&r.dp) as usize;
                if !seen_dps.insert(dp_raw) {
                    if let Some(&pos) = local.get(&dp_raw) {
                        entries[pos]
                            .3
                            .entry(r.state.clone())
                            .or_insert_with(|| r.val.clone());
                    }
                    continue;
                }
                local.insert(dp_raw, entries.len());
                let mut sm = BTreeMap::new();
                sm.insert(r.state.clone(), r.val.clone());
                entries.push((dp_raw, r.dp.clone(), r.name.clone(), sm));
            }
            for (dp, prop, var) in &rs.captures {
                if !capture_specs.iter().any(|(_, _, v)| v == var) {
                    capture_specs.push((dp.clone(), prop.clone(), var.clone()));
                }
            }
        }
    }
    // Phase B — all COM work on a local ElemState (callbacks fire re-entrant
    // during SetValue/registration; they look the element up by handle and
    // no-op until it's inserted at the end).
    let dobj: DependencyObject = match Interface::cast(el) {
        Ok(d) => d,
        Err(_) => return,
    };
    let mut es = ElemState {
        el: el.clone(),
        fallback: fallback_type.to_string(),
        props: BTreeMap::new(),
        dp_meta: BTreeMap::new(),
        captures: Vec::new(),
        vsg_subs: Vec::new(),
        size_revoker: None,
        tree_node: node_for(&dobj),
    };
    let el_type = class_name_el(el);
    let scope = scope_of(es.tree_node.as_ref());
    for ((vsg_name, _), (vsg_owner, dps)) in &buckets {
        // The group resolves on the element the `@vsg` matcher matched —
        // an ancestor's `@vsg` tracks the ancestor's group, not ours.
        let vsg = if vsg_name.is_empty() {
            None
        } else {
            vsg_owner.as_ref().and_then(|o| get_vsg(o, vsg_name))
        };
        let current = vsg
            .as_ref()
            .and_then(|v| v.CurrentState().ok())
            .and_then(|s| s.Name().ok())
            .map(|h| h.to_string_lossy())
            .unwrap_or_default();
        for (dp_raw, dp, name, state_map) in dps {
            let key = (vsg_name.clone(), *dp_raw);
            let pick = if current.is_empty() {
                state_map.get("")
            } else {
                state_map.get(&current).or_else(|| state_map.get(""))
            };
            let mut slot = PropSlot {
                original: None,
                last_applied: None,
                custom: None,
                token: 0,
                state_map: state_map.clone(),
                dyn_template: None,
                deps: Vec::new(),
                last_resolve_failed: false,
            };
            if let Some(pick) = pick {
                let resolved = match pick {
                    StyleVal::Dynamic(raw, is_xaml) => {
                        slot.dyn_template = Some((raw.clone(), *is_xaml));
                        let mut deps = Vec::new();
                        let r = eval_dynamic(
                            es.tree_node.as_ref(),
                            scope,
                            &el_type,
                            fallback_type,
                            name,
                            raw,
                            *is_xaml,
                            &mut deps,
                        );
                        {
                            let mut eng = state();
                            update_consumers(
                                &mut eng.consumers,
                                handle,
                                *dp_raw,
                                fallback_type,
                                scope,
                                &[],
                                &deps,
                            );
                        }
                        slot.deps = deps;
                        match r {
                            Err(()) => {
                                slot.last_resolve_failed = false;
                                None
                            }
                            Ok(None) => {
                                slot.last_resolve_failed = true;
                                None
                            }
                            Ok(v) => v,
                        }
                    }
                    other => Some(other.clone()),
                };
                if let Some(v) = resolved {
                    slot.original = read_local(&dobj, dp, name);
                    slot.custom = Some(v.clone());
                    slot.last_applied = set_or_clear(&dobj, dp, name, &v, true);
                }
            }
            // The re-apply callback registers for every slotted property —
            // it no-ops until `custom` is set.
            slot.token = register_reapply(handle, &dobj, dp);
            es.props.insert(key, slot);
            es.dp_meta.insert(*dp_raw, (dp.clone(), name.clone()));
        }
        // Subscribe once per bucket's VSG — the args carry the state names.
        if let Some(vsg) = vsg {
            let vn = vsg_name.clone();
            let revoker = vsg
                .CurrentStateChanged(move |_, args| {
                    let (old, new) = args
                        .ok()
                        .ok()
                        .map(|e| {
                            (
                                e.OldState()
                                    .ok()
                                    .and_then(|s| s.Name().ok())
                                    .map(|h| h.to_string_lossy())
                                    .unwrap_or_default(),
                                e.NewState()
                                    .ok()
                                    .and_then(|s| s.Name().ok())
                                    .map(|h| h.to_string_lossy())
                                    .unwrap_or_default(),
                            )
                        })
                        .unwrap_or_default();
                    on_vsg_state_changed(handle, vn.clone(), old, new);
                })
                .ok();
            es.vsg_subs.push(VsgSub { vsg, revoker });
        }
    }
    // `SetUpCapturesForElement` — seed all captures in one batch, then
    // propagate each seeded name (a set change re-scores every consumer).
    let mut seeded: Vec<String> = Vec::new();
    let mut size_specs: Vec<(DependencyProperty, String)> = Vec::new();
    for (dp, prop, var) in &capture_specs {
        let val = read_captured(el, dp);
        {
            let mut eng = state();
            eng.variables.entry(var.clone()).or_default().push(Capture {
                owner: handle,
                scope,
                val,
            });
        }
        seeded.push(var.clone());
        if is_layout_driven(prop) {
            size_specs.push((dp.clone(), var.clone()));
            continue;
        }
        let var_c = var.clone();
        let dp_c = dp.clone();
        let token = dobj
            .RegisterPropertyChangedCallback(
                dp,
                &DependencyPropertyChangedCallback::new(move |_, _| {
                    capture_changed(handle, &var_c, &dp_c);
                    Ok(())
                }),
            )
            .unwrap_or(0);
        es.captures.push(CaptureSlot {
            dp: Some(dp.clone()),
            var: var.clone(),
            token,
        });
    }
    if !size_specs.is_empty() {
        let specs = size_specs;
        let revoker = el
            .SizeChanged(move |_, _| {
                for (dp, var) in &specs {
                    capture_changed(handle, var, dp);
                }
            })
            .ok();
        es.size_revoker = revoker;
    }
    // Phase C — insert + seed propagation (set-change: changedOwner=None).
    crate::tap_log!(
        "styled {fallback_type} handle={handle} props={} caps={}",
        es.props.len(),
        es.captures.len()
    );
    {
        let mut eng = state();
        eng.elements.insert(handle, es);
    }
    for var in seeded {
        propagate(&var, None, scope);
    }
}

// ============================================================================
// Resource variables + layout detour
// ============================================================================

/// `ProcessResourceVariablesFromSettings` — the themed-dict path the catalog
/// uses (`key@Dark`/`key@Light`), `{ThemeResource x}` references, `key:=xaml`.
/// Returns the merged theme dict for removal on restore.
fn apply_resvars(entries: &[(String, String)]) -> Option<xb::ResourceDictionary> {
    if entries.is_empty() {
        return None;
    }
    let app = xb::Application::Current().ok()?;
    let resources = app.Resources().ok()?;
    let res_map: windows_collections::IMap<IInspectable, IInspectable> =
        Interface::cast(&resources).ok()?;
    // g_resourceVariablesThemeDict: one dict with Dark/Light children, merged.
    let theme_dict = xb::ResourceDictionary::new().ok()?;
    let theme_dicts = theme_dict.ThemeDictionaries().ok()?;
    let dark = xb::ResourceDictionary::new().ok()?;
    let light = xb::ResourceDictionary::new().ok()?;
    let dark_map: windows_collections::IMap<IInspectable, IInspectable> =
        Interface::cast(&dark).ok()?;
    let light_map: windows_collections::IMap<IInspectable, IInspectable> =
        Interface::cast(&light).ok()?;
    let box_str =
        |s: &str| crate::fx_bindings::PropertyValue::CreateString(&windows_core::HSTRING::from(s));
    let dark_key = box_str("Dark").ok()?;
    let light_key = box_str("Light").ok()?;
    let dark_obj: IInspectable = Interface::cast(&dark).ok()?;
    let light_obj: IInspectable = Interface::cast(&light).ok()?;
    let _ = theme_dicts.Insert(&dark_key, &dark_obj);
    let _ = theme_dicts.Insert(&light_key, &light_obj);
    for (key_raw, value) in entries {
        let (key_part, is_xaml) = match key_raw.strip_suffix(':') {
            Some(k) => (k, true),
            None => (key_raw.as_str(), false),
        };
        let trimmed = value.trim();
        // Non-themed `key=value` (no @Dark/@Light, no `:=`, no ThemeResource)
        // would need XamlBindingHelper::ConvertValue — the catalog never uses
        // it; skip with a log.
        let themed = key_part.split_once('@').map(|(k, _)| k);
        let is_themeref = trimmed.starts_with("{ThemeResource ") && trimmed.ends_with('}');
        if themed.is_none() && !is_xaml && !is_themeref {
            crate::tap_log!("resvar {key_part}: non-themed value unsupported");
            continue;
        }
        let (key, target) = match key_part.split_once('@') {
            Some((k, "Dark")) => (k, &dark_map),
            Some((k, "Light")) => (k, &light_map),
            _ => (key_part, &dark_map),
        };
        let resolved: Option<IInspectable> = if is_themeref {
            let ref_key = trimmed["{ThemeResource ".len()..trimmed.len() - 1].trim();
            box_str(ref_key).ok().and_then(|k| res_map.Lookup(&k).ok())
        } else if is_xaml {
            let xaml =
                format!("<Setter Property=\"Tag\"><Setter.Value>{trimmed}</Setter.Value></Setter>");
            style_from_setters(NS_FE, "", &xaml)
                .ok()
                .and_then(|s| s.Setters().ok())
                .and_then(|ss| {
                    Interface::cast::<windows_collections::IVector<xb::SetterBase>>(&ss).ok()
                })
                .and_then(|vec| vec.GetAt(0).ok())
                .and_then(|sb| Interface::cast::<xb::Setter>(&sb).ok())
                .and_then(|s| s.Value().ok())
        } else {
            box_str(trimmed).ok()
        };
        let Some(k) = box_str(key).ok() else {
            continue;
        };
        if target.HasKey(&k).unwrap_or(false) {
            continue;
        }
        let _ = target.Insert(
            &k,
            &resolved.unwrap_or_else(|| {
                crate::fx_bindings::PropertyValue::CreateString(&windows_core::HSTRING::from(""))
                    .unwrap_or_else(|_| crate::fx_bindings::PropertyValue::CreateEmpty().unwrap())
            }),
        );
    }
    if let Ok(merged) = resources.MergedDictionaries() {
        let _ = merged.Append(&theme_dict);
    }
    Some(theme_dict)
}

// ---- `RtlQueryFeatureConfiguration` detour (layout override) ----

#[repr(C)]
struct RtlFeatureConfiguration {
    feature_id: u32,
    /// `group:4 | enabledState:2 | enabledStateOptions:1 | unused1:1 |
    /// variant:6 | variantPayloadKind:2 | unused2:16`
    flags: u32,
    payload: u32,
}

type RtlQueryFn =
    unsafe extern "system" fn(u32, i32, *mut i64, *mut RtlFeatureConfiguration) -> i32;

static LAYOUT_DETOUR: Mutex<Option<retour::RawDetour>> = Mutex::new(None);
static LAYOUT_TRAMP: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
/// 0 = Default (no override), 1 = ForceNew, 2 = NewSideBySide.
static DETOUR_LAYOUT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
/// `g_windowsDefaultIsNewLayout` — probed BEFORE the detour ever arms.
static DEFAULT_IS_NEW: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

unsafe extern "system" fn rtl_query_hook(
    feature_id: u32,
    group: i32,
    variant: *mut i64,
    config: *mut RtlFeatureConfiguration,
) -> i32 {
    let original: RtlQueryFn =
        unsafe { core::mem::transmute(LAYOUT_TRAMP.load(Ordering::Acquire)) };
    let ret = unsafe { original(feature_id, group, variant, config) };
    if config.is_null() {
        return ret;
    }
    let layout = DETOUR_LAYOUT.load(Ordering::Relaxed);
    unsafe {
        let set_state = |cfg: *mut RtlFeatureConfiguration, enabled: bool| {
            (*cfg).flags = ((*cfg).flags & !(0b11 << 4)) | ((enabled as u32) << 4);
        };
        match feature_id {
            // Phone-Link flyout (StartDocked.dll ≤10.0.26100.8328).
            48697323 => {
                if layout > 0 {
                    set_state(config, true);
                }
            }
            // The revamped start menu itself (49221331 is probe-only).
            47205210 | 49402389 => {
                if layout > 0 {
                    set_state(config, true);
                }
            }
            _ => {}
        }
    }
    ret
}

fn rtl_query(feature_id: u32) -> Option<bool> {
    let ntdll = unsafe { win32::GetModuleHandleW(windows_core::w!("ntdll.dll")) };
    if ntdll.is_null() {
        return None;
    }
    let f = unsafe {
        win32::GetProcAddress(
            ntdll,
            windows_core::PCSTR(c"RtlQueryFeatureConfiguration".as_ptr() as *const u8),
        )
    };
    if f.is_null() {
        return None;
    }
    let f: RtlQueryFn = unsafe { core::mem::transmute(f) };
    let mut cfg = RtlFeatureConfiguration {
        feature_id: 0,
        flags: 0,
        payload: 0,
    };
    let mut stamp = 0i64;
    if unsafe { f(feature_id, 1, &mut stamp, &mut cfg) } < 0 {
        return None;
    }
    match (cfg.flags >> 4) & 0b11 {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    }
}

fn windows_default_is_new_layout() -> bool {
    *DEFAULT_IS_NEW.get_or_init(|| {
        rtl_query(47205210).unwrap_or(true)
            && rtl_query(49221331).unwrap_or(true)
            && rtl_query(49402389).unwrap_or(true)
    })
}

fn arm_layout_detour(layout: Layout) {
    DETOUR_LAYOUT.store(
        match layout {
            Layout::ForceNew => 1,
            Layout::NewSideBySide => 2,
            Layout::Default => 0,
        },
        Ordering::Release,
    );
    let mut guard = LAYOUT_DETOUR.lock().unwrap_or_else(|p| p.into_inner());
    if guard.is_some() {
        let _ = unsafe { guard.as_ref().unwrap().enable() };
        return;
    }
    let ntdll = unsafe { win32::GetModuleHandleW(windows_core::w!("ntdll.dll")) };
    if ntdll.is_null() {
        return;
    }
    let target = unsafe {
        win32::GetProcAddress(
            ntdll,
            windows_core::PCSTR(c"RtlQueryFeatureConfiguration".as_ptr() as *const u8),
        )
    };
    if target.is_null() {
        return;
    }
    let Ok(detour) =
        (unsafe { retour::RawDetour::new(target as *const (), rtl_query_hook as *const ()) })
    else {
        return;
    };
    LAYOUT_TRAMP.store(
        detour.trampoline() as *const () as *mut (),
        Ordering::Release,
    );
    if unsafe { detour.enable() }.is_err() {
        return;
    }
    *guard = Some(detour);
}

fn disarm_layout_detour() {
    DETOUR_LAYOUT.store(0, Ordering::Release);
    let guard = LAYOUT_DETOUR.lock().unwrap_or_else(|p| p.into_inner());
    if let Some(d) = guard.as_ref() {
        unsafe {
            let _ = d.disable();
        }
    }
}

/// `%TEMP%\peco_smeh_exit.tick` — the mod's 10 s exit throttle.
fn throttled_exit() {
    let path = std::env::temp_dir().join("peco_smeh_exit.tick");
    let last: u64 = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let now = unsafe { win32::GetTickCount64() };
    if now.saturating_sub(last) > 10_000 {
        let _ = std::fs::write(&path, now.to_string());
        crate::tap_log!("layout override -> ExitProcess");
        unsafe { win32::ExitProcess(0) }
    }
}

// ============================================================================
// `IXamlDiagnosticsTestHooks` — `UnregisterInstance` releases the runtime
// object cached for a reported handle (xamlom doesn't declare this iface;
// the IID comes from the mod).
// ============================================================================

#[derive(Clone)]
#[repr(transparent)]
struct TestHooks(windows_core::IUnknown);

#[repr(C)]
struct TestHooksVtbl {
    base__: windows_core::IUnknown_Vtbl,
    UnregisterInstance:
        unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle) -> HRESULT,
    #[allow(non_snake_case)]
    TryGetDispatcherQueueForObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        InstanceHandle,
        *mut *mut core::ffi::c_void,
    ) -> HRESULT,
}

unsafe impl Interface for TestHooks {
    type Vtable = TestHooksVtbl;
    const IID: GUID = GUID::from_u128(0x735941a2_3ee3_495a_8da9_972627003075);
}

impl TestHooks {
    /// Returns Ok when the handle's cached object was released.
    unsafe fn unregister(&self, handle: InstanceHandle) -> windows_core::Result<()> {
        unsafe {
            (Interface::vtable(self).UnregisterInstance)(Interface::as_raw(self), handle).ok()
        }
    }
}

/// `ElementHasState` — a handle still recorded somewhere must not be released
/// (its removal is never re-reported once the runtime object is gone).
fn element_has_state(handle: u64) -> bool {
    let eng = state();
    eng.elements.contains_key(&handle)
        || eng
            .variables
            .values()
            .any(|v| v.iter().any(|c| c.owner == handle))
        || eng
            .consumers
            .values()
            .any(|v| v.iter().any(|c| c.handle == handle))
}

static DRAIN_ARMED: AtomicBool = AtomicBool::new(false);

/// `QueueDiagnosticsRelease` — releases run on the dispatcher once the tree
/// walk finishes (releasing inside a report destroys elements mid-walk).
fn queue_release(handle: u64) {
    if handle == 0 {
        return;
    }
    {
        let mut eng = state();
        eng.to_release.push(handle);
    }
    if DRAIN_ARMED.swap(true, Ordering::SeqCst) {
        return;
    }
    // `drain_releases` calls `GetIInspectableFromHandle`/`UnregisterInstance`
    // — diagnostics-service calls that may marshal to the element's owning
    // dispatcher. Running them ON a dispatcher can therefore self-deadlock
    // the island's UI pump (mux replay also holds `state()` while parked on
    // the pump). A short-delay worker keeps the batching semantics without
    // ever touching a UI queue.
    std::thread::spawn(|| {
        unsafe { crate::win32::Sleep(150) };
        DRAIN_ARMED.store(false, Ordering::SeqCst);
        drain_releases();
    });
}

/// `FlushDiagnosticsReleases` — dedupe, skip handles that still have state,
/// `UnregisterInstance` the rest, forget the ones that actually died.
fn drain_releases() {
    let mut pending = {
        let mut eng = state();
        core::mem::take(&mut eng.to_release)
    };
    if pending.is_empty() {
        return;
    }
    pending.sort_unstable();
    pending.dedup();
    let Some(diag) = mod_diagnostics() else {
        return;
    };
    let hooks: TestHooks = match Interface::cast(&diag) {
        Ok(h) => h,
        Err(_) => return,
    };
    for h in pending {
        if element_has_state(h) {
            continue;
        }
        // A released object may still live via other refs — only forget the
        // handle when the release actually destroyed it.
        let weak = unsafe { diag.GetIInspectableFromHandle(InstanceHandle(h)) }
            .ok()
            .and_then(|o| o.downgrade().ok());
        if unsafe { hooks.unregister(InstanceHandle(h)) }.is_err() {
            continue;
        }
        if weak.as_ref().is_none_or(|w| w.upgrade().is_none()) {
            known().remove(&h);
        }
    }
}

// ============================================================================
// Restore + entry points (command.rs / site.rs / lib.rs)
// ============================================================================

/// `CleanupCustomizations` — unregister capture callbacks, drop the element's
/// captures (propagating each removal), unregister reapply callbacks + drop
/// consumer entries, restore originals, drop the element state.
fn cleanup_element(handle: u64, es: &ElemState) {
    let dobj = Interface::cast::<DependencyObject>(&es.el).ok();
    // 1. RestoreCapturesForElement — unregister per-DP capture callbacks.
    if let Some(d) = &dobj {
        for cap in &es.captures {
            if let (Some(dp), token) = (&cap.dp, cap.token)
                && token != 0
            {
                let _ = d.UnregisterPropertyChangedCallback(dp, token);
            }
        }
    }
    // es.size_revoker / VsgSub.revokers revoke on drop.
    // 2. Drop this element's captures from the registry.
    let mut removed: BTreeSet<(String, usize)> = BTreeSet::new();
    {
        let mut eng = state();
        for cap in &es.captures {
            let Some(list) = eng.variables.get_mut(&cap.var) else {
                continue;
            };
            let mut scope = None;
            list.retain(|c| {
                if c.owner == handle {
                    scope = Some(c.scope);
                    false
                } else {
                    true
                }
            });
            if scope.is_some() {
                removed.insert((cap.var.clone(), scope.unwrap()));
            }
            if list.is_empty() {
                eng.variables.remove(&cap.var);
            }
        }
    }
    // 3. RestoreCustomizationsForVisualStateGroup — unregister reapply
    // callbacks, drop consumer registrations, restore originals.
    {
        let mut eng = state();
        for ((_vn, dp_raw), slot) in &es.props {
            if !slot.deps.is_empty() {
                update_consumers(&mut eng.consumers, handle, *dp_raw, "", 0, &slot.deps, &[]);
            }
        }
    }
    if let Some(d) = &dobj {
        for ((_, dp_raw), slot) in &es.props {
            let Some((dp, name)) = es.dp_meta.get(dp_raw) else {
                continue;
            };
            if slot.token != 0 {
                let _ = d.UnregisterPropertyChangedCallback(dp, slot.token);
            }
            if let Some(orig) = &slot.original {
                MODIFYING.store(true, Ordering::Relaxed);
                let _ = set_or_clear(d, dp, name, &StyleVal::Ready(Some(orig.clone())), false);
                MODIFYING.store(false, Ordering::Relaxed);
            }
        }
    }
    // 4. Propagate each capture removal — dropping one of several captures
    // still changes which one wins for nearby consumers.
    for (var, scope) in removed {
        propagate(&var, None, scope);
    }
}

/// One element's undo pass — unregister callbacks + restore originals.
fn restore_element(es: &ElemState) {
    let dobj: DependencyObject = match Interface::cast(&es.el) {
        Ok(d) => d,
        Err(_) => return,
    };
    for cap in &es.captures {
        if let (Some(dp), token) = (&cap.dp, cap.token)
            && token != 0
        {
            let _ = dobj.UnregisterPropertyChangedCallback(dp, token);
        }
    }
    for ((_vn, dp_raw), slot) in &es.props {
        let Some((dp, name)) = es.dp_meta.get(&dp_raw) else {
            continue;
        };
        if slot.token != 0 {
            let _ = dobj.UnregisterPropertyChangedCallback(dp, slot.token);
        }
        MODIFYING.store(true, Ordering::Relaxed);
        match &slot.original {
            Some(orig) => {
                let _ = set_or_clear(&dobj, dp, name, &StyleVal::Ready(Some(orig.clone())), false);
            }
            None => {
                let _ = dobj.ClearValue(dp);
            }
        }
        MODIFYING.store(false, Ordering::Relaxed);
    }
}

/// Undoes everything applied — `CleanupCustomizations` for all elements +
/// resvar removal (used on theme switch/reset; propagation is pointless once
/// the whole registry is going away, so this skips it).
fn restore_all() {
    let elements: Vec<ElemState> = {
        let mut eng = state();
        eng.variables.clear();
        eng.consumers.clear();
        core::mem::take(&mut eng.elements).into_values().collect()
    };
    for es in elements {
        if MARSHAL_ON_ELEMENT {
            // Elements may live on different island dispatchers — restore
            // each on its own UI thread.
            if let Some(d) = elem_dispatcher(&es.el).or_else(|| state().dispatcher.clone()) {
                let payload = UiBound(es);
                post_ui(&d, Prio::Normal, move || {
                    // Borrow `payload` first — `payload.0` alone would be a
                    // disjoint capture of the !Send ElemState.
                    let p = &payload;
                    restore_element(&p.0);
                    Ok(())
                });
                continue;
            }
        }
        restore_element(&es);
    }
    // Unmerge the theme dictionary.
    let dict = state().theme_dict.take();
    if let Some(dict) = dict
        && let Ok(app) = xb::Application::Current()
        && let Ok(res) = app.Resources()
        && let Ok(merged) = res.MergedDictionaries()
    {
        let mut idx = 0u32;
        if merged.IndexOf(&dict, &mut idx).unwrap_or(false) {
            let _ = merged.RemoveAt(idx);
        }
    }
}

/// Command-thread entry: parse the wire, handle the layout override
/// (StartMenu only), dispatch the apply sweep to the XAML UI thread.
pub fn apply_wire(text: &str) {
    let (kind, theme) = match parse_wire(text) {
        Ok(t) => t,
        Err(e) => {
            crate::tap_log!("styler: bad wire {e}");
            return;
        }
    };
    crate::tap_log!(
        "styler: {kind:?} theme layout={:?} modern={} classic={} resvars={}",
        theme.layout,
        theme.modern.len(),
        theme.classic.len(),
        theme.resvars.len()
    );
    if kind == WireKind::StartMenu {
        // The mod's `g_windowsDefaultIsNewLayout`/`g_isRedesignedStartMenu`:
        //   redesigned = forceNew | sideBySide | (default && windowsDefaultIsNew)
        //   classic    = !redesigned
        let win_default_new = windows_default_is_new_layout();
        let forced = matches!(theme.layout, Layout::ForceNew | Layout::NewSideBySide);
        if forced && !win_default_new {
            arm_layout_detour(theme.layout);
            // The menu is rebuilt with the override — exit so SMEH reconstructs
            // it under the forced feature flags (relaunches automatically; the
            // host re-injects and re-sends the theme).
            throttled_exit();
        }
        let mut eng = state();
        let is_redesigned = forced || win_default_new;
        eng.pending_classic = !is_redesigned && !theme.classic.is_empty();
        eng.pending = Some((kind, theme));
    } else {
        let mut eng = state();
        eng.pending_classic = false;
        eng.pending = Some((kind, theme));
    }
    dispatch_apply();
}

/// `windowsDefault` theme / reset — restore originals, disarm the detour.
pub fn reset() {
    crate::tap_log!("styler: reset");
    {
        let mut eng = state();
        eng.pending = None;
        eng.applied = None;
        eng.wire_rules = None;
        eng.rules_gen += 1;
        eng.to_release.clear();
    }
    let dispatcher = state().dispatcher.clone();
    if let Some(dispatcher) = dispatcher {
        post_ui(&dispatcher, Prio::Normal, || {
            restore_all();
            Ok(())
        });
    }
    // Reverting to Windows Default may need the layout back too.
    disarm_layout_detour();
}

/// `dispatch_apply` for the styler — posts `apply_theme_ui` to the captured
/// dispatcher; without one the pending theme sits until the first element.
fn dispatch_apply() {
    let Some(dispatcher) = state().dispatcher.clone() else {
        crate::tap_log!("styler: no dispatcher yet — apply deferred");
        return;
    };
    post_ui(&dispatcher, Prio::Normal, || {
        apply_theme_ui();
        Ok(())
    });
}

/// The apply sweep — UI thread.
fn apply_theme_ui() {
    restore_all();
    let (kind, theme, use_classic) = {
        let mut eng = state();
        let Some((kind, theme)) = eng.pending.take() else {
            return;
        };
        eng.applied = Some(kind);
        (kind, theme, eng.pending_classic)
    };
    let wire_rules = if use_classic && !theme.classic.is_empty() {
        &theme.classic
    } else {
        &theme.modern
    };
    crate::tap_log!("styler: {kind:?} resolving {} rulesets", wire_rules.len());
    {
        // Resolution happens lazily per dispatcher thread
        // (`rules_for_thread`) — values materialized via `XamlReader.Load`
        // here would be affine to this dispatcher and fail `SetValue` on
        // elements living on sibling island dispatchers.
        let mut eng = state();
        eng.wire_rules = Some(Rc::new(wire_rules.clone()));
        eng.rules_gen += 1;
    }
    // Resource vars before element styles — `{ThemeResource}` lookups during
    // application must resolve (the mod applies them in the same order).
    let dict = apply_resvars(&theme.resvars);
    state().theme_dict = dict;
    // Re-advise the watcher — `RefreshExplorerTapStyle`'s approach: reported
    // handles were already released back to the diagnostics service, so a
    // handle-replay sweep resolves nothing; `Unadvise`+`Advise` makes the
    // service re-report the whole tree and `on_element` styles each element
    // against the new rules with live handles. Runs on a worker thread —
    // `Advise` replays onto the UI thread, and we're already on it inside a
    // dispatcher item, so a blocking advise would self-deadlock.
    let pair = mod_service_watcher();
    match pair {
        Some((service, watcher)) => {
            // Agile proxies — same justification as `start_advise`'s move.
            struct SendPair(
                crate::bindings::IVisualTreeService3,
                crate::bindings::IVisualTreeServiceCallback2,
            );
            unsafe impl Send for SendPair {}
            unsafe impl Sync for SendPair {}
            let pair = SendPair(service, watcher);
            std::thread::spawn(move || {
                // Borrow `pair` as a whole — destructuring would capture the
                // non-Send fields individually.
                let pair = &pair;
                unsafe {
                    let _ = pair.0.UnadviseVisualTreeChange(&*pair.1);
                    let hr = pair.0.AdviseVisualTreeChange(&*pair.1);
                    crate::tap_log!("styler: re-advise hr={hr:?}");
                }
                // The re-report marshals onto the UI thread — count styled
                // elements after it settles (verification telemetry).
                unsafe { crate::win32::Sleep(800) };
                crate::tap_log!(
                    "styler: re-advise done — {} elements styled",
                    state().elements.len()
                );
            });
        }
        None => {
            crate::tap_log!("styler: no watcher — kicking install/advise");
            crate::site::ensure_watching();
        }
    }
}

/// Tests an element against every ruleset (reverse file order → later rules
/// win); applies on match. `fallback_type` = the reported `element.Type`.
fn style_element(handle: u64, el: &FrameworkElement, fallback_type: &str, _parent: u64) -> bool {
    let eng_rules = rules_for_thread();
    let matched: Vec<(usize, Vec<Option<FrameworkElement>>)> = {
        (0..eng_rules.len())
            .rev()
            .filter_map(|ri| {
                let r = &eng_rules[ri];
                if !test_matcher(el, &r.matcher, &r.preds, fallback_type) {
                    return None;
                }
                match_parents(el, &r.parents).map(|ancestors| (ri, ancestors))
            })
            .collect()
    };
    if matched.is_empty() {
        return false;
    }
    apply_element(handle, el, fallback_type, &matched, &eng_rules);
    true
}

/// `OnVisualTreeChange` (Add/Update) — capture the dispatcher, run a
/// deferred theme apply, register the handle, queue the handle+parent for
/// release, style on match.
pub fn on_element(
    diagnostics: &IXamlDiagnostics,
    element: &VisualElement,
    parent: u64,
    reported_type: &str,
) {
    known().insert(element.Handle.0, reported_type.to_string());
    queue_release(element.Handle.0);
    queue_release(parent);
    // Snapshot engine state first — `GetIInspectableFromHandle` and
    // `DependencyObject::DispatcherQueue` may marshal to the element's
    // dispatcher for WinUI3 elements, so they must never run while holding
    // `state()`: a dispatcher-side drain (`drain_releases`) blocking on that
    // same mutex would deadlock the island's UI pump permanently.
    let (need_dispatcher, has_pending, has_applied) = {
        let eng = state();
        (
            eng.dispatcher.is_none(),
            eng.pending.is_some(),
            eng.applied.is_some(),
        )
    };
    // A theme landed after the dispatcher was captured — apply it now.
    if has_pending && !need_dispatcher {
        dispatch_apply();
        return;
    }
    // Nothing applied and nothing to learn from this element.
    if !has_applied && !need_dispatcher && !has_pending {
        return;
    }
    let Ok(obj) = (unsafe { diagnostics.GetIInspectableFromHandle(element.Handle) }) else {
        return;
    };
    let Ok(el) = Interface::cast::<FrameworkElement>(&obj) else {
        return;
    };
    // Record the reported type against the element's IUnknown identity so
    // ancestor walks in `match_parents` can fall back to it too.
    if let Ok(dobj) = Interface::cast::<DependencyObject>(&obj)
        && let Some(k) = identity_key(&dobj)
    {
        reported_types().insert(k, reported_type.to_string());
    }
    // The element's own dispatcher — a marshalling call for mux elements, so
    // it stays outside the `state()` critical section too. Skipped entirely
    // when neither the capture path nor the marshal path needs it.
    let el_disp = if need_dispatcher || MARSHAL_ON_ELEMENT {
        elem_dispatcher(&el)
    } else {
        None
    };
    {
        let mut eng = state();
        if eng.dispatcher.is_none() {
            eng.dispatcher = el_disp.clone();
        }
        // A theme landed before the dispatcher existed — apply it now.
        if eng.pending.is_some() && eng.dispatcher.is_some() {
            drop(eng);
            // `resolve_ruleset` materializes values (XamlReader.Load, brushes)
            // which are dispatcher-affine — it must run on the UI thread, so
            // go through `dispatch_apply` rather than a direct call.
            dispatch_apply();
            return;
        }
        if eng.applied.is_none() {
            return;
        }
    }
    if MARSHAL_ON_ELEMENT {
        // `SetValue` on the diagnostics callback thread throws
        // RPC_E_WRONG_THREAD — marshal the whole match+apply to the
        // element's own dispatcher.
        let handle = element.Handle.0;
        let ty = reported_type.to_string();
        if let Some(d) = el_disp.or_else(|| state().dispatcher.clone()) {
            let payload = UiBound(el);
            post_ui(&d, Prio::Normal, move || {
                let p = &payload;
                let _ = style_element(handle, &p.0, &ty, parent);
                Ok(())
            });
            return;
        }
    }
    let _ = style_element(element.Handle.0, &el, reported_type, parent);
}

/// `OnVisualTreeChange` (Remove) — `CleanupCustomizations`: drop state,
/// unregister, restore originals, propagate capture removal.
pub fn on_element_removed(handle: u64) {
    known().remove(&handle);
    queue_release(handle);
    let es = {
        let mut eng = state();
        eng.elements.remove(&handle)
    };
    if let Some(es) = es {
        if MARSHAL_ON_ELEMENT
            && let Some(d) = elem_dispatcher(&es.el).or_else(|| state().dispatcher.clone())
        {
            let payload = UiBound(es);
            post_ui(&d, Prio::Normal, move || {
                let p = &payload;
                cleanup_element(handle, &p.0);
                Ok(())
            });
            return;
        }
        cleanup_element(handle, &es);
    }
}

/// Whether a theme is currently armed — platform keeps the DLL resident
/// while this is true.
pub fn is_armed() -> bool {
    state().applied.is_some()
}

/// Full teardown on unload/host-death.
pub fn shutdown() {
    let dispatcher = state().dispatcher.clone();
    if let Some(dispatcher) = dispatcher {
        let done = unsafe {
            win32::CreateEventW(
                core::ptr::null(),
                windows_core::BOOL(1),
                windows_core::BOOL(0),
                windows_core::PCWSTR::null(),
            )
        };
        let done_addr = done as usize;
        post_ui(&dispatcher, Prio::Normal, move || {
            restore_all();
            if done_addr != 0 {
                unsafe {
                    let _ = win32::SetEvent(done_addr as win32::HANDLE);
                }
            }
            Ok(())
        });
        unsafe {
            let _ = win32::WaitForSingleObject(done, 3_000);
            let _ = win32::CloseHandle(done);
        }
    }
    {
        let mut eng = state();
        eng.pending = None;
        eng.applied = None;
        eng.dispatcher = None;
        eng.wire_rules = None;
        eng.rules_gen += 1;
        eng.to_release.clear();
    }
    known().clear();
    TL_RULES.with(|t| *t.borrow_mut() = (0, Rc::new(Vec::new())));
    TREE_NODES.with(|p| p.borrow_mut().clear());
    PROP_QUEUE.with(|q| q.borrow_mut().clear());
    disarm_layout_detour();
}
