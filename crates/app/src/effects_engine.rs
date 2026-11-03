//! Native desktop particle & weather engine (`تأثيرات سطح المكتب الحية`).
//!
//! Pure simulation + `DrawingSession` calls: no window, COM or allocation
//! knowledge lives here. A single fixed pool of [`MAX_PARTICLES`] records
//! (~28 KB) is reused by every effect; `tick` and `draw` never allocate.
//! Device-bound resources live in [`FxCache`] — a mutable [`Brush`] whose
//! color is rewritten per particle plus gradient brushes pre-baked at 8
//! opacity levels — created lazily on first use after a device rebuild.

use pecofence_render::{
    Bitmap, Brush, ColorF, DrawingSession, Ellipse, GradientStop, LinearGradient, Matrix3x2,
    RadialGradient, Rect, TextFormat, Vector2,
};
use windows_core::Result;

/// Upper bound of live particles; `intensity` scales the active prefix.
pub const MAX_PARTICLES: usize = 2048;

/// Per-gradient alpha quantization (0.125 steps) — smooth enough for particles.
const ALPHA_BUCKETS: usize = 8;

/// The selectable effects; `"none"` never reaches this enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Effect {
    #[default]
    Rain,
    Thunderstorm,
    Snow,
    Fireflies,
    Embers,
    Sakura,
    Mist,
    Aurora,
    Starfield,
    Matrix,
    Ripples,
    Constellation,
    MagicTrail,
    ShatterClicks,
    QuantumSwarm,
    CyberGridReveal,
    RepulsionDust,
}

impl Effect {
    /// Settings id → effect. `None` for `"none"`/unknown — the host
    /// keeps `"none"` mapped to "destroy the overlay", never entering here.
    pub fn from_id(id: &str) -> Option<Self> {
        Some(match id.trim() {
            "rain" => Self::Rain,
            "thunderstorm" => Self::Thunderstorm,
            "snow" => Self::Snow,
            "fireflies" => Self::Fireflies,
            "embers" => Self::Embers,
            "sakura" => Self::Sakura,
            "mist" => Self::Mist,
            "aurora" => Self::Aurora,
            "starfield" => Self::Starfield,
            "matrix" => Self::Matrix,
            "ripples" => Self::Ripples,
            "constellation" => Self::Constellation,
            "magic_trail" => Self::MagicTrail,
            "shatter_clicks" => Self::ShatterClicks,
            "quantum_swarm" => Self::QuantumSwarm,
            "cyber_grid_reveal" => Self::CyberGridReveal,
            "repulsion_dust" => Self::RepulsionDust,
            _ => return None,
        })
    }

    /// Cursor-driven effects may sleep the tick loop while the pointer is still.
    pub fn wants_cursor(&self) -> bool {
        matches!(
            self,
            Self::Ripples
                | Self::Constellation
                | Self::MagicTrail
                | Self::ShatterClicks
                | Self::QuantumSwarm
                | Self::CyberGridReveal
                | Self::RepulsionDust
        )
    }
}

/// One particle: 56 bytes; the pool is allocated once — indices are the
/// identity, `k`/`f2` are per-effect type and scratch channels.
#[derive(Clone, Copy, Default)]
struct P {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
    a: f32,
    rot: f32,
    vr: f32,
    ph: f32,
    fq: f32,
    age: f32,
    life: f32,
    k: f32,
    f2: f32,
}

#[inline]
fn bucket(a: f32) -> usize {
    ((a.clamp(0.0, 1.0) * ALPHA_BUCKETS as f32) as usize).min(ALPHA_BUCKETS - 1)
}

/// Device-bound drawing resources. One mutable [`Brush`] per draw routine
/// (per-particle color via `set_color`, zero COM churn) and gradient brushes
/// pre-baked per alpha bucket — gradients have no runtime-opacity setter in
/// the canvas API, so buckets cover the opacity range once per device gen.
#[derive(Default)]
pub struct FxCache {
    /// Warm-lime → amber radial glow (fireflies).
    glow_warm: Vec<RadialGradient>,
    /// Orange → red radial glow (embers).
    glow_ember: Vec<RadialGradient>,
    /// Cool white → transparent radial glow (mist blobs).
    glow_mist: Vec<RadialGradient>,
    /// Petal-pink radial (sakura).
    petal: Vec<RadialGradient>,
    /// Emerald → cyan → violet vertical column (aurora ribbons).
    ribbon: Vec<LinearGradient>,
    /// White → transparent horizontal tail (meteor).
    tail: Vec<LinearGradient>,
    /// Reused mutable solid brush for the current draw routine.
    pen: Option<Brush>,
    /// Matrix glyph strip: 24 cells × 20 px in one row.
    glyphs: Option<Bitmap>,
}

impl FxCache {
    /// One mutable brush; callers `set_color` per particle — no COM churn.
    fn pen(&mut self, s: &DrawingSession<'_>) -> Option<&Brush> {
        if self.pen.is_none() {
            self.pen = s.create_solid_brush(cf(1.0, 1.0, 1.0, 1.0)).ok();
        }
        self.pen.as_ref()
    }

    fn radial(
        s: &DrawingSession<'_>,
        stops: &[GradientStop],
        out: &mut Vec<RadialGradient>,
    ) -> bool {
        if out.is_empty() {
            for i in 0..ALPHA_BUCKETS {
                let m = (i + 1) as f32 / ALPHA_BUCKETS as f32;
                let scaled: Vec<GradientStop> = stops
                    .iter()
                    .map(|st| {
                        let mut c = st.color;
                        c.a *= m;
                        GradientStop {
                            position: st.position,
                            color: c,
                        }
                    })
                    .collect();
                match s.create_radial_gradient(v2(0.0, 0.0), 1.0, 1.0, &scaled) {
                    Ok(g) => out.push(g),
                    Err(_) => {
                        out.clear();
                        return false;
                    }
                }
            }
        }
        true
    }

    fn linear(
        s: &DrawingSession<'_>,
        start: Vector2,
        end: Vector2,
        stops: &[GradientStop],
        out: &mut Vec<LinearGradient>,
    ) -> bool {
        if out.is_empty() {
            for i in 0..ALPHA_BUCKETS {
                let m = (i + 1) as f32 / ALPHA_BUCKETS as f32;
                let scaled: Vec<GradientStop> = stops
                    .iter()
                    .map(|st| {
                        let mut c = st.color;
                        c.a *= m;
                        GradientStop {
                            position: st.position,
                            color: c,
                        }
                    })
                    .collect();
                match s.create_linear_gradient(start, end, &scaled) {
                    Ok(g) => out.push(g),
                    Err(_) => {
                        out.clear();
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// The engine: simulation state + draw routines, no window handles.
pub struct Engine {
    effect: Effect,
    intensity: f32,
    w: f32,
    h: f32,
    t: f32,
    rng: u64,
    ps: Vec<P>,
    // Thunderstorm / starfield transient actors.
    bolt: [Vector2; 64],
    bolt_n: usize,
    bolt_age: f32,
    next_bolt: f32,
    flash: f32,
    met: P,
    met_live: bool,
    next_met: f32,
    // Cursor channel (ripples + constellation + the interactive effects).
    cursor: (f32, f32),
    cursor_prev: (f32, f32),
    button_down: bool,
    button_prev: bool,
    /// Round-robin scan head for cursor-spawned slots (trail sparks, shards).
    emit_i: usize,
}

#[inline]
fn v2(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
}

#[inline]
fn cf(r: f32, g: f32, b: f32, a: f32) -> ColorF {
    ColorF { r, g, b, a }
}

#[inline]
fn ellipse(x: f32, y: f32, rx: f32, ry: f32) -> Ellipse {
    Ellipse {
        center: v2(x, y),
        radius_x: rx,
        radius_y: ry,
    }
}

#[inline]
fn stop(pos: f32, c: ColorF) -> GradientStop {
    GradientStop {
        position: pos,
        color: c,
    }
}

impl Engine {
    pub fn new(effect: Effect, intensity: f32, w: f32, h: f32) -> Self {
        let mut e = Self {
            effect,
            intensity: intensity.clamp(0.1, 1.0),
            w,
            h,
            t: 0.0,
            rng: 0x9E3779B97F4A7C15 ^ (effect as u64 + 1).wrapping_mul(0xD1B54A32D192ED03),
            ps: vec![P::default(); MAX_PARTICLES],
            bolt: [Vector2 { x: 0.0, y: 0.0 }; 64],
            bolt_n: 0,
            bolt_age: 1.0,
            next_bolt: 0.0,
            flash: 0.0,
            met: P::default(),
            met_live: false,
            next_met: 0.0,
            cursor: (0.0, 0.0),
            cursor_prev: (0.0, 0.0),
            button_down: false,
            button_prev: false,
            emit_i: 0,
        };
        e.seed();
        e
    }

    pub fn effect(&self) -> Effect {
        self.effect
    }

    /// xorshift64: no `rand` dependency, deterministic per effect.
    #[inline]
    fn rnd(&mut self) -> f32 {
        let mut s = self.rng;
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        self.rng = s;
        (s >> 40) as f32 / 16777215.0
    }

    #[inline]
    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * self.rnd()
    }

    /// Resize (DIP). Out-of-bounds particles wrap back in.
    pub fn set_size(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
        for p in &mut self.ps {
            if p.x > w {
                p.x %= w.max(1.0);
            }
            if p.y > h {
                p.y %= h.max(1.0);
            }
        }
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.1, 1.0);
        self.seed();
    }

    /// Cursor position in surface coordinates each tick.
    pub fn set_cursor(&mut self, x: f32, y: f32) {
        self.cursor = (x, y);
    }

    /// Primary mouse button state (async poll — the overlay is click-through
    /// so it never sees WM_LBUTTON*).
    pub fn set_button_down(&mut self, down: bool) {
        self.button_down = down;
    }

    /// First-fill the pool so the effect starts fully formed.
    fn seed(&mut self) {
        self.t = 0.0;
        let n = self.target_count();
        for i in 0..MAX_PARTICLES {
            self.ps[i] = if i < n {
                self.spawn(i, true)
            } else {
                P::default()
            };
        }
    }

    /// Live particle count for the current intensity — a per-effect base
    /// scaled by the 10–100 % slider. `Aurora` and `CyberGridReveal` draw
    /// procedural geometry, not particles.
    fn target_count(&self) -> usize {
        let base = match self.effect {
            Effect::Rain | Effect::Thunderstorm => 400,
            Effect::Snow => 320,
            Effect::Fireflies => 90,
            Effect::Embers => 180,
            Effect::Sakura => 200,
            Effect::Mist => 8,
            Effect::Aurora | Effect::CyberGridReveal => 0,
            Effect::Starfield => 160,
            Effect::Matrix => 120,
            Effect::Ripples => 200,
            Effect::Constellation => 90,
            Effect::MagicTrail => 260,
            Effect::ShatterClicks => 220,
            Effect::QuantumSwarm => 100,
            Effect::RepulsionDust => MAX_PARTICLES,
        };
        ((base as f32 * self.intensity) as usize).min(MAX_PARTICLES)
    }

    /// (Re)spawn slot `i`; `anywhere` fills the whole screen on seed.
    fn spawn(&mut self, i: usize, anywhere: bool) -> P {
        let (w, h) = (self.w, self.h);
        let mut p = P {
            x: self.range(0.0, w),
            y: 0.0,
            ph: self.range(0.0, std::f32::consts::TAU),
            fq: self.range(0.5, 2.5),
            life: 1.0,
            ..P::default()
        };
        match self.effect {
            Effect::Rain | Effect::Thunderstorm => {
                if anywhere {
                    p.y = self.range(0.0, h);
                    // Jittered grid: one drop per slot across the width — the
                    // pure-uniform seed could leave visible bare bands.
                    let n = self.target_count().max(1) as f32;
                    p.x = (i as f32 + self.rnd()) * (w / n);
                } else {
                    p.y = self.range(-40.0, -8.0);
                }
                p.vy = self.range(520.0, 780.0);
                p.vx = p.vy * 0.21; // ~12° slant
                p.r = self.range(18.0, 35.0);
                p.a = self.range(0.25, 0.55);
                p.k = self.range(1.2, 1.8);
            }
            Effect::Snow => {
                p.y = if anywhere {
                    self.range(0.0, h)
                } else {
                    self.range(-30.0, -4.0)
                };
                p.k = (i % 3) as f32;
                p.r = [3.5, 2.2, 1.2][p.k as usize] * self.range(0.85, 1.2);
                p.vy = self.range(50.0, 130.0) * (1.0 - p.k * 0.22);
                p.a = self.range(0.5, 0.95);
            }
            Effect::Fireflies => {
                p.y = self.range(0.0, h);
                p.vx = self.range(-14.0, 14.0);
                p.vy = self.range(-10.0, 10.0);
                p.r = self.range(4.0, 9.0);
                p.fq = self.range(0.6, 2.2);
                p.k = if self.rnd() < 0.3 { 1.0 } else { 0.0 };
            }
            Effect::Embers => {
                p.y = if anywhere {
                    self.range(0.0, h)
                } else {
                    h + self.range(2.0, 24.0)
                };
                p.vy = -self.range(72.0, 210.0);
                p.vx = self.range(-24.0, 24.0);
                p.r = self.range(1.6, 4.2);
                p.life = self.range(3.0, 7.0);
                p.fq = self.range(4.0, 9.0);
            }
            Effect::Sakura => {
                p.y = if anywhere {
                    self.range(0.0, h)
                } else {
                    self.range(-36.0, -6.0)
                };
                p.vx = self.range(20.0, 60.0);
                p.vy = self.range(26.0, 62.0);
                p.r = self.range(4.5, 9.5);
                p.vr = self.range(-1.6, 1.6);
                p.fq = self.range(1.0, 3.0);
            }
            Effect::Mist => {
                p.x = self.range(-w * 0.2, w);
                p.y = self.range(h * 0.55, h * 1.02);
                p.r = self.range(160.0, 420.0);
                p.f2 = self.range(60.0, 140.0);
                p.vx = self.range(-14.0, 14.0);
                p.a = self.range(0.04, 0.09);
            }
            Effect::Aurora => {}
            Effect::Starfield => {
                p.y = self.range(0.0, h * 0.9);
                p.r = self.range(0.6, 1.8);
                p.fq = self.range(0.5, 3.0);
                p.vx = self.range(-2.0, 2.0);
            }
            Effect::Matrix => {
                p.x = (i as f32 * 47.0 + self.range(0.0, 30.0)) % w.max(1.0);
                p.y = if anywhere {
                    self.range(0.0, h)
                } else {
                    self.range(-200.0, -20.0)
                };
                p.vy = self.range(150.0, 420.0);
                p.k = self.range(0.0, 23.99);
                p.f2 = self.range(6.0, 14.0);
            }
            Effect::Ripples => {
                p.life = 0.0;
            }
            Effect::Constellation => {
                p.y = self.range(0.0, h);
                p.vx = self.range(-9.0, 9.0);
                p.vy = self.range(-7.0, 7.0);
                p.r = self.range(1.2, 2.4);
            }
            // Born at the cursor, not seeded — `life = 0` keeps slots dead
            // until `tick_magic_trail`/`tick_shatter` claim them.
            Effect::MagicTrail | Effect::ShatterClicks => {
                p.life = 0.0;
            }
            Effect::QuantumSwarm => {
                p.y = self.range(0.0, h);
                p.vx = self.range(-26.0, 26.0);
                p.vy = self.range(-26.0, 26.0);
                p.r = self.range(1.6, 3.4);
                p.fq = self.range(0.4, 1.3);
                p.a = self.range(0.45, 0.9);
                p.life = 1.0;
            }
            Effect::RepulsionDust => {
                p.y = self.range(0.0, h);
                p.vx = self.range(-6.0, 6.0);
                p.vy = -self.range(4.0, 16.0); // ambient rise
                p.r = self.range(0.7, 1.7);
                p.a = self.range(0.08, 0.24);
                p.fq = self.range(0.3, 1.4);
                p.life = 1.0;
            }
            Effect::CyberGridReveal => {}
        }
        p
    }

    /// True while the effect has nothing animating — the host sleeps the
    /// tick loop. `wake_check` re-arms it on cursor motion / a button press.
    pub fn idle(&self) -> bool {
        let still = self.cursor == self.cursor_prev;
        let dead = self.ps[..self.target_count().min(self.ps.len())]
            .iter()
            .all(|p| p.life <= 0.0);
        match self.effect {
            Effect::Ripples => still && dead,
            // Trail sparks and shards decay on their own once the burst ends.
            Effect::MagicTrail => dead,
            // A held button must keep the loop alive so the press-edge ticks.
            Effect::ShatterClicks => dead && !self.button_down,
            // The grid is static — a resting cursor means the same frame.
            Effect::CyberGridReveal => still,
            // Swarm and dust drift forever; everything else never sleeps.
            _ => false,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        let dt = dt.min(0.1);
        self.t += dt;
        let n = self.target_count().min(self.ps.len());
        match self.effect {
            Effect::Rain | Effect::Thunderstorm => self.tick_rain(n, dt),
            Effect::Snow => self.tick_snow(n, dt),
            Effect::Fireflies => self.tick_fireflies(n, dt),
            Effect::Embers => self.tick_embers(n, dt),
            Effect::Sakura => self.tick_sakura(n, dt),
            Effect::Mist => self.tick_mist(n, dt),
            Effect::Aurora => {}
            Effect::Starfield => self.tick_starfield(n, dt),
            Effect::Matrix => self.tick_matrix(n, dt),
            Effect::Ripples => self.tick_ripples(n, dt),
            Effect::Constellation => self.tick_constellation(n, dt),
            Effect::MagicTrail => self.tick_magic_trail(n, dt),
            Effect::ShatterClicks => self.tick_shatter(n, dt),
            Effect::QuantumSwarm => self.tick_swarm(n, dt),
            Effect::CyberGridReveal => {}
            Effect::RepulsionDust => self.tick_dust(n, dt),
        }
        if self.flash > 0.0 {
            self.flash = (self.flash - dt * 4.0).max(0.0);
        }
        if self.bolt_n > 0 {
            self.bolt_age += dt;
            if self.bolt_age > 0.5 {
                self.bolt_n = 0;
            }
        }
        self.cursor_prev = self.cursor;
        self.button_prev = self.button_down;
    }

    /// Next reusable (dead) slot in `0..n`; the round-robin scan caps at `n`.
    fn claim_dead_slot(&mut self, n: usize) -> Option<usize> {
        for _ in 0..n {
            let i = self.emit_i % n.max(1);
            self.emit_i = (self.emit_i + 1) % n.max(1);
            if self.ps[i].life <= 0.0 {
                return Some(i);
            }
        }
        None
    }

    fn respawn(&mut self, i: usize) {
        self.ps[i] = self.spawn(i, false);
    }

    fn tick_rain(&mut self, n: usize, dt: f32) {
        let h = self.h;
        // Pure background fall — drops pass through fences and the clock.
        for i in 0..n {
            let p = &mut self.ps[i];
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            if p.y > h + 40.0 || p.x > self.w + 40.0 {
                self.respawn(i);
            }
        }
        if self.effect == Effect::Thunderstorm {
            self.next_bolt -= dt;
            if self.next_bolt <= 0.0 {
                self.next_bolt = self.range(5.0, 11.0);
                self.strike_lightning();
            }
        }
    }

    /// Midpoint-displacement fractal bolt into the fixed `bolt` buffer
    /// plus two short branches; `flash` lights the sky and decays in `tick`.
    fn strike_lightning(&mut self) {
        let (w, h) = (self.w, self.h);
        let x0 = self.range(w * 0.1, w * 0.9);
        let x1 = (x0 + self.range(-200.0, 200.0)).clamp(0.0, w);
        let y1 = h * 0.7;
        let mut pts: [Vector2; 64] = [v2(0.0, 0.0); 64];
        pts[0] = v2(x0, 0.0);
        pts[1] = v2(x1, y1);
        let mut seg = 2usize;
        for it in 0..5 {
            let mut src = seg - 1;
            let displace = 90.0f32 / (it + 1) as f32;
            while src > 0 && seg + 1 < 64 {
                let a = pts[src - 1];
                let b = pts[src];
                let mid = v2(
                    (a.x + b.x) * 0.5 + self.range(-displace, displace),
                    (a.y + b.y) * 0.5,
                );
                // Shift right of `src` one slot, drop `mid` into the hole.
                let mut j = seg;
                while j > src {
                    pts[j] = pts[j - 1];
                    j -= 1;
                }
                pts[src] = mid;
                src -= 1;
                seg += 1;
            }
        }
        self.bolt_n = seg.min(64);
        self.bolt[..self.bolt_n].copy_from_slice(&pts[..self.bolt_n]);
        // Two branch stubs after the main bolt, NaN-separated.
        for b in 0..2 {
            if self.bolt_n + 4 >= 64 {
                break;
            }
            let o = 8 + b * 14;
            let a = pts[o.min(self.bolt_n.max(1) - 1)];
            let dir = if b == 0 { -1.0 } else { 1.0 };
            let (mut bx, mut by) = (a.x, a.y);
            for _ in 0..3 {
                bx += dir * self.range(14.0, 42.0);
                by += self.range(18.0, 50.0);
                self.bolt[self.bolt_n] = v2(bx, by);
                self.bolt_n += 1;
            }
            self.bolt[self.bolt_n] = v2(f32::NAN, f32::NAN);
            self.bolt_n += 1;
        }
        self.bolt_age = 0.0;
        self.flash = 0.16;
    }

    fn tick_snow(&mut self, n: usize, dt: f32) {
        let h = self.h;
        // Flakes drift down past every UI element — no settling on roofs.
        for i in 0..n {
            let mut respawn = false;
            {
                let p = &mut self.ps[i];
                if p.life <= 0.5 {
                    p.life -= dt;
                    if p.life <= 0.0 {
                        respawn = true;
                    }
                } else {
                    p.ph += dt * p.fq;
                    p.x += (p.vx + p.ph.sin() * 24.0) * dt;
                    p.y += p.vy * dt;
                    if p.y > h + 12.0 {
                        respawn = true;
                    }
                }
            }
            if respawn {
                self.respawn(i);
            }
        }
    }

    fn tick_fireflies(&mut self, n: usize, dt: f32) {
        for i in 0..n {
            let p = &mut self.ps[i];
            p.ph += dt * p.fq;
            p.x += (p.vx + p.ph.sin() * 22.0) * dt;
            p.y += (p.vy + (p.ph * 0.7).cos() * 16.0) * dt;
            let m = p.r * 2.0;
            if p.x < -m {
                p.x = self.w + m;
            } else if p.x > self.w + m {
                p.x = -m;
            }
            if p.y < -m {
                p.y = self.h + m;
            } else if p.y > self.h + m {
                p.y = -m;
            }
        }
    }

    fn tick_embers(&mut self, n: usize, dt: f32) {
        for i in 0..n {
            let dead = {
                let p = &mut self.ps[i];
                p.age += dt;
                p.ph += dt * p.fq;
                p.x += (p.vx + p.ph.sin() * 18.0) * dt;
                p.y += p.vy * dt;
                p.life -= dt;
                p.life <= 0.0 || p.y < -12.0
            };
            if dead {
                self.respawn(i);
            }
        }
    }

    fn tick_sakura(&mut self, n: usize, dt: f32) {
        let h = self.h;
        for i in 0..n {
            let mut respawn = false;
            {
                let p = &mut self.ps[i];
                p.ph += dt * p.fq;
                p.rot += p.vr * dt;
                p.x += (p.vx + p.ph.sin() * 26.0) * dt;
                p.y += p.vy * dt;
                if p.y > h + 14.0 || p.x > self.w + 20.0 {
                    respawn = true;
                }
            }
            if respawn {
                self.respawn(i);
            }
        }
    }

    fn tick_mist(&mut self, n: usize, dt: f32) {
        for i in 0..n {
            let p = &mut self.ps[i];
            p.x += p.vx * dt;
            let m = p.r * 1.4;
            if p.x < -m {
                p.x = self.w + m * 0.8;
            } else if p.x > self.w + m {
                p.x = -m * 0.8;
            }
        }
    }

    fn tick_starfield(&mut self, n: usize, dt: f32) {
        for i in 0..n {
            let p = &mut self.ps[i];
            p.ph += dt * p.fq;
            p.x += p.vx * dt;
            if p.x < 0.0 {
                p.x = self.w;
            } else if p.x > self.w {
                p.x = 0.0;
            }
        }
        if self.met_live {
            let m = &mut self.met;
            m.x += m.vx * dt;
            m.y += m.vy * dt;
            m.life -= dt * 1.4;
            if m.life <= 0.0 || m.x > self.w + 200.0 || m.y > self.h + 100.0 {
                self.met_live = false;
            }
        } else {
            self.next_met -= dt;
            if self.next_met <= 0.0 {
                self.next_met = self.range(5.0, 10.0);
                let mut m = P::default();
                m.x = self.range(0.0, self.w * 0.7);
                m.y = self.range(0.0, self.h * 0.25);
                let sp = self.range(700.0, 1100.0);
                let ang = self.range(0.5, 0.9);
                m.vx = sp * ang.cos();
                m.vy = sp * ang.sin();
                m.life = self.range(0.7, 1.2);
                m.r = self.range(1.6, 2.6);
                self.met = m;
                self.met_live = true;
            }
        }
    }

    fn tick_matrix(&mut self, n: usize, dt: f32) {
        let h = self.h;
        for i in 0..n {
            let mut respawn = false;
            {
                let p = &mut self.ps[i];
                p.y += p.vy * dt;
                if p.y > h + p.f2 * 20.0 {
                    respawn = true;
                }
            }
            if respawn {
                self.respawn(i);
            }
        }
    }

    fn tick_ripples(&mut self, n: usize, dt: f32) {
        let (cx, cy) = self.cursor;
        let (px, py) = self.cursor_prev;
        let moved = ((cx - px).powi(2) + (cy - py).powi(2)).sqrt() > 12.0;
        if moved {
            for i in 0..n {
                if self.ps[i].life <= 0.0 {
                    let p = &mut self.ps[i];
                    p.x = cx;
                    p.y = cy;
                    p.r = 2.0;
                    p.life = 1.0;
                    break;
                }
            }
        }
        for i in 0..n {
            let p = &mut self.ps[i];
            if p.life > 0.0 {
                p.life -= dt * 0.9;
                p.r += dt * 95.0;
                if p.life <= 0.0 {
                    p.life = 0.0;
                }
            }
        }
    }

    fn tick_constellation(&mut self, n: usize, dt: f32) {
        let (cx, cy) = self.cursor;
        for i in 0..n {
            let p = &mut self.ps[i];
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            let dx = cx - p.x;
            let dy = cy - p.y;
            let d2 = dx * dx + dy * dy;
            if d2 < 32400.0 && d2 > 4.0 {
                let d = d2.sqrt();
                let pull = 14.0 * dt / d;
                p.x += dx * pull;
                p.y += dy * pull;
            }
            if p.x < -8.0 {
                p.x = self.w + 8.0;
            } else if p.x > self.w + 8.0 {
                p.x = -8.0;
            }
            if p.y < -8.0 {
                p.y = self.h + 8.0;
            } else if p.y > self.h + 8.0 {
                p.y = -8.0;
            }
        }
    }

    /// Neon trail: sparks born along the cursor segment (interpolated so fast
    /// flicks never gap), then drag, shrink and fade out.
    fn tick_magic_trail(&mut self, n: usize, dt: f32) {
        let (cx, cy) = self.cursor;
        let (px, py) = self.cursor_prev;
        let dist = ((cx - px) * (cx - px) + (cy - py) * (cy - py)).sqrt();
        // ~1 spark per 7 px travelled, capped per tick.
        let emit = ((dist / 7.0) as usize + usize::from(dist > 1.0)).min(9);
        for k in 0..emit {
            let t = (k + 1) as f32 / emit as f32;
            let (jx, jy) = (self.range(-2.0, 2.0), self.range(-2.0, 2.0));
            let (vx, vy) = (self.range(-26.0, 26.0), self.range(-26.0, 26.0));
            let (r, hue, span) = (self.range(1.8, 4.6), self.rnd(), self.range(0.75, 1.0));
            let Some(i) = self.claim_dead_slot(n) else {
                break;
            };
            let p = &mut self.ps[i];
            p.x = px + (cx - px) * t + jx;
            p.y = py + (cy - py) * t + jy;
            p.vx = vx;
            p.vy = vy;
            p.r = r;
            p.ph = hue; // 0 → cyan, 1 → violet
            p.life = 1.0;
            p.fq = span; // lifetime scale (seconds-ish)
        }
        let drag = (1.0 - 3.2 * dt).max(0.0);
        for i in 0..n {
            let p = &mut self.ps[i];
            if p.life <= 0.0 {
                continue;
            }
            p.life -= dt / (1.4 * p.fq).max(0.3);
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= drag;
            p.vy *= drag;
        }
    }

    /// Click shatter: on the press edge a burst of glass shards jumps out of
    /// the cursor, spins, and falls under gravity.
    fn tick_shatter(&mut self, n: usize, dt: f32) {
        if self.button_down && !self.button_prev {
            let (cx, cy) = self.cursor;
            const BURST: usize = 18;
            for _ in 0..BURST {
                let ang = self.range(0.0, std::f32::consts::TAU);
                let sp = self.range(150.0, 430.0);
                let lift = self.range(60.0, 240.0);
                let (rot, vr) = (
                    self.range(0.0, std::f32::consts::TAU),
                    self.range(-9.0, 9.0),
                );
                let (r, a, life) = (
                    self.range(3.0, 9.0),
                    self.range(0.35, 0.75),
                    self.range(0.8, 1.4),
                );
                let Some(i) = self.claim_dead_slot(n) else {
                    break;
                };
                let p = &mut self.ps[i];
                p.x = cx;
                p.y = cy;
                p.vx = ang.cos() * sp;
                p.vy = ang.sin() * sp - lift;
                p.rot = rot;
                p.vr = vr;
                p.r = r;
                p.a = a;
                p.life = life;
            }
        }
        for i in 0..n {
            let p = &mut self.ps[i];
            if p.life <= 0.0 {
                continue;
            }
            p.life -= dt;
            p.vy += 1350.0 * dt;
            p.vx *= (1.0 - 0.5 * dt).max(0.0);
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.rot += p.vr * dt;
        }
    }

    /// Magnetic swarm: aimless drifters inside a 300 px radius get pulled into
    /// a vortex around a fast-moving cursor, then disperse.
    fn tick_swarm(&mut self, n: usize, dt: f32) {
        const RADIUS: f32 = 300.0;
        let (cx, cy) = self.cursor;
        let (px, py) = self.cursor_prev;
        let cvx = (cx - px) / dt.max(0.001);
        let cvy = (cy - py) / dt.max(0.001);
        let cspeed = (cvx * cvx + cvy * cvy).sqrt();
        // Only a moving cursor stirs the swarm; strength scales with speed.
        let strength = (cspeed / 620.0).clamp(0.0, 1.4);
        let damping = (1.0 - 4.2 * dt).max(0.0);
        for i in 0..n {
            let p = &mut self.ps[i];
            p.ph += dt * p.fq;
            // Ambient wander.
            p.vx += p.ph.sin() * 11.0 * dt;
            p.vy += (p.ph * 1.31).cos() * 11.0 * dt;
            if strength > 0.02 {
                let dx = cx - p.x;
                let dy = cy - p.y;
                let d2 = dx * dx + dy * dy;
                if d2 < RADIUS * RADIUS && d2 > 16.0 {
                    let d = d2.sqrt();
                    let f = (1.0 - d / RADIUS) * strength;
                    // Radial pull + perpendicular swirl = vortex.
                    let pull = 1250.0 * f;
                    let swirl = 1500.0 * f;
                    p.vx += (dx / d * pull - dy / d * swirl) * dt;
                    p.vy += (dy / d * pull + dx / d * swirl) * dt;
                }
            }
            p.vx *= damping;
            p.vy *= damping;
            let sp2 = p.vx * p.vx + p.vy * p.vy;
            if sp2 > 400.0 * 400.0 {
                let s = 400.0 / sp2.sqrt();
                p.vx *= s;
                p.vy *= s;
            }
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            if p.x < -12.0 {
                p.x = self.w + 12.0;
            } else if p.x > self.w + 12.0 {
                p.x = -12.0;
            }
            if p.y < -12.0 {
                p.y = self.h + 12.0;
            } else if p.y > self.h + 12.0 {
                p.y = -12.0;
            }
        }
    }

    /// Repulsion dust: motes rise gently; the cursor is a 150 px repulsive
    /// sphere that pushes them aside smoothly.
    fn tick_dust(&mut self, n: usize, dt: f32) {
        const RADIUS: f32 = 150.0;
        let (cx, cy) = self.cursor;
        let damping = (1.0 - 2.6 * dt).max(0.0);
        for i in 0..n {
            let p = &mut self.ps[i];
            p.ph += dt * p.fq;
            let dx = p.x - cx;
            let dy = p.y - cy;
            let d2 = dx * dx + dy * dy;
            if d2 < RADIUS * RADIUS && d2 > 4.0 {
                let d = d2.sqrt();
                let f = (1.0 - d / RADIUS).powi(2);
                p.vx += dx / d * f * 2600.0 * dt;
                p.vy += dy / d * f * 2600.0 * dt;
                p.k = f; // draw-time flare channel
            } else {
                p.k *= (1.0 - 6.0 * dt).max(0.0);
            }
            // Wander + buoyancy, then drag so the push decays smoothly.
            p.vx += p.ph.sin() * 5.0 * dt;
            p.vy -= 9.0 * dt;
            if p.vy < -30.0 {
                p.vy = -30.0;
            }
            p.vx *= damping;
            p.vy *= damping;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            let wrapped = p.y < -8.0;
            if wrapped {
                p.y = self.h + 8.0;
            } else if p.y > self.h + 8.0 {
                p.y = -8.0;
            }
            if p.x < -8.0 {
                p.x = self.w + 8.0;
            } else if p.x > self.w + 8.0 {
                p.x = -8.0;
            }
            if wrapped {
                // Re-enter from below at a fresh lane.
                let nx = self.range(0.0, self.w);
                self.ps[i].x = nx;
            }
        }
    }

    /// Draw the frame into the panel session (surface already clipped to size).
    pub fn draw(&self, s: &DrawingSession<'_>, cache: &mut FxCache) -> Result<()> {
        s.clear(cf(0.0, 0.0, 0.0, 0.0));
        match self.effect {
            Effect::Rain | Effect::Thunderstorm => self.draw_rain(s, cache),
            Effect::Snow => self.draw_snow(s, cache),
            Effect::Fireflies => self.draw_fireflies(s, cache),
            Effect::Embers => self.draw_embers(s, cache),
            Effect::Sakura => self.draw_sakura(s, cache),
            Effect::Mist => self.draw_mist(s, cache),
            Effect::Aurora => self.draw_aurora(s, cache),
            Effect::Starfield => self.draw_starfield(s, cache),
            Effect::Matrix => self.draw_matrix(s, cache),
            Effect::Ripples => self.draw_ripples(s, cache),
            Effect::Constellation => self.draw_constellation(s, cache),
            Effect::MagicTrail => self.draw_magic_trail(s, cache),
            Effect::ShatterClicks => self.draw_shatter(s, cache),
            Effect::QuantumSwarm => self.draw_swarm(s, cache),
            Effect::CyberGridReveal => self.draw_grid_reveal(s, cache),
            Effect::RepulsionDust => self.draw_dust(s, cache),
        }
        if self.flash > 0.0 {
            if let Some(b) = cache.pen(s) {
                b.set_color(cf(0.9, 0.95, 1.0, self.flash));
                s.fill_rect(&Rect::new(0.0, 0.0, self.w, self.h), b);
            }
        }
        Ok(())
    }

    fn draw_rain(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        for i in 0..self.ps.len() {
            let p = &self.ps[i];
            let splash = p.k >= 9.0;
            if !splash && i >= n {
                break; // dead tail holds only splashes
            }
            if splash && p.life <= 0.0 {
                continue;
            }
            let Some(b) = cache.pen(s) else { return };
            if splash {
                b.set_color(cf(0.75, 0.85, 1.0, (p.life * 1.8).min(0.6)));
                s.draw_line(
                    v2(p.x, p.y),
                    v2(p.x - p.vx * 0.03, p.y - p.vy * 0.03),
                    b,
                    1.1,
                );
            } else {
                let len = p.r;
                let norm = (1.0f32 + 0.21 * 0.21).sqrt();
                b.set_color(if i & 1 == 0 {
                    cf(0.75, 0.85, 1.0, p.a)
                } else {
                    cf(0.85, 0.92, 1.0, p.a)
                });
                s.draw_line(
                    v2(p.x, p.y),
                    v2(p.x - 0.21 * len / norm, p.y - len / norm),
                    b,
                    p.k,
                );
            }
        }
        if self.effect == Effect::Thunderstorm && self.bolt_n > 1 {
            let fade = (1.0 - self.bolt_age / 0.5).clamp(0.0, 1.0);
            let glow = s.create_solid_brush(cf(0.49, 0.83, 0.99, 0.30 * fade));
            let halo = s.create_solid_brush(cf(0.75, 0.92, 1.0, 0.55 * fade));
            let core = s.create_solid_brush(cf(1.0, 1.0, 1.0, 0.95 * fade));
            if let (Ok(glow), Ok(halo), Ok(core)) = (glow, halo, core) {
                let mut open = false;
                let mut prev = v2(0.0, 0.0);
                for pt in &self.bolt[..self.bolt_n] {
                    if pt.x.is_nan() {
                        open = false;
                        continue;
                    }
                    if open {
                        s.draw_line(prev, *pt, &glow, 10.0);
                        s.draw_line(prev, *pt, &halo, 4.0);
                        s.draw_line(prev, *pt, &core, 1.6);
                    }
                    prev = *pt;
                    open = true;
                }
            }
        }
    }

    fn draw_snow(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let Some(b) = cache.pen(s) else { return };
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let a = if p.life <= 0.5 { p.life * p.a } else { p.a };
            b.set_color(cf(1.0, 1.0, 1.0, a));
            s.fill_ellipse(&ellipse(p.x, p.y, p.r, p.r), b);
        }
    }

    fn draw_fireflies(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        if !FxCache::radial(
            s,
            &[
                stop(0.0, cf(0.99, 0.94, 0.54, 1.0)),
                stop(0.45, cf(0.96, 0.62, 0.04, 0.55)),
                stop(1.0, cf(0.96, 0.62, 0.04, 0.0)),
            ],
            &mut cache.glow_warm,
        ) {
            return;
        }
        // `with_transform` replaces the caller-visible transform — compose the
        // session's base (DIP scale + panel offset) in or particles draw in raw
        // pixels and crowd the top-left on scaled monitors.
        let base = s.transform();
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let pulse = (self.t * p.fq + p.ph).sin() * 0.5 + 0.5;
            let a = pulse * pulse;
            let r = p.r * (0.7 + 0.5 * pulse);
            if p.k > 0.5 {
                if let Some(c) = cache.pen(s) {
                    c.set_color(cf(0.53, 0.92, 0.99, a * 0.8));
                    s.fill_ellipse(&ellipse(p.x, p.y, r * 0.5, r * 0.5), c);
                }
                continue;
            }
            let g = &cache.glow_warm[bucket(a)];
            let m = Matrix3x2::scale(r, r) * Matrix3x2::translation(p.x, p.y) * base;
            s.with_transform(&m, || s.fill_ellipse(&ellipse(0.0, 0.0, 1.0, 1.0), g));
        }
    }

    fn draw_embers(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        if !FxCache::radial(
            s,
            &[
                stop(0.0, cf(1.0, 0.93, 0.84, 1.0)),
                stop(0.4, cf(0.97, 0.45, 0.09, 0.7)),
                stop(1.0, cf(0.86, 0.15, 0.15, 0.0)),
            ],
            &mut cache.glow_ember,
        ) {
            return;
        }
        let base = s.transform();
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let life_t = (p.life / 6.0).clamp(0.0, 1.0);
            let flick = 0.75 + 0.25 * p.ph.sin();
            let a = (life_t * flick).min(1.0);
            let r = p.r * (0.5 + life_t * 0.7);
            let g = &cache.glow_ember[bucket(a)];
            let m = Matrix3x2::scale(r, r) * Matrix3x2::translation(p.x, p.y) * base;
            s.with_transform(&m, || s.fill_ellipse(&ellipse(0.0, 0.0, 2.4, 2.4), g));
        }
    }

    fn draw_sakura(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        if !FxCache::radial(
            s,
            &[
                stop(0.0, cf(1.0, 0.85, 0.9, 0.95)),
                stop(1.0, cf(0.96, 0.62, 0.75, 0.75)),
            ],
            &mut cache.petal,
        ) {
            return;
        }
        let base = s.transform();
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let squash = p.ph.cos().abs().max(0.15);
            let m = Matrix3x2::rotation(p.rot)
                * Matrix3x2::scale(squash, 1.0)
                * Matrix3x2::translation(p.x, p.y)
                * base;
            let g = &cache.petal[bucket(0.9)];
            s.with_transform(&m, || {
                s.fill_ellipse(&ellipse(0.0, 0.0, p.r, p.r * 0.62), g)
            });
        }
    }

    fn draw_mist(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        if !FxCache::radial(
            s,
            &[
                stop(0.0, cf(0.85, 0.9, 1.0, 1.0)),
                stop(1.0, cf(0.85, 0.9, 1.0, 0.0)),
            ],
            &mut cache.glow_mist,
        ) {
            return;
        }
        let base = s.transform();
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let g = &cache.glow_mist[bucket(p.a)];
            let m = Matrix3x2::scale(p.r, p.f2) * Matrix3x2::translation(p.x, p.y) * base;
            s.with_transform(&m, || s.fill_ellipse(&ellipse(0.0, 0.0, 1.0, 1.0), g));
        }
    }

    fn draw_aurora(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        if !FxCache::linear(
            s,
            v2(0.0, 0.0),
            v2(0.0, -1.0),
            &[
                stop(0.0, cf(0.20, 0.83, 0.60, 0.55)),
                stop(0.55, cf(0.22, 0.74, 0.97, 0.32)),
                stop(1.0, cf(0.66, 0.33, 0.97, 0.0)),
            ],
            &mut cache.ribbon,
        ) {
            return;
        }
        let base = s.transform();
        let (w, h) = (self.w, self.h);
        let step = 26.0f32;
        for band in 0..3 {
            let yb = h * (0.10 + band as f32 * 0.12);
            let amp = h * (0.05 + band as f32 * 0.025);
            let a = 0.16 - band as f32 * 0.04;
            let g = &cache.ribbon[bucket(a)];
            let mut x = 0.0f32;
            while x < w {
                let t = self.t * (0.6 + band as f32 * 0.25);
                let y = yb + amp * (x * 0.003 + t).sin() + amp * 0.5 * (x * 0.007 - t * 1.3).sin();
                let m = Matrix3x2::scale(1.0, amp * 3.2) * Matrix3x2::translation(x, y) * base;
                s.with_transform(&m, || s.draw_line(v2(0.0, 0.0), v2(0.0, -1.0), g, step));
                x += step;
            }
        }
    }

    fn draw_starfield(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            let tw = (self.t * p.fq + p.ph).sin() * 0.5 + 0.5;
            let Some(b) = cache.pen(s) else { return };
            b.set_color(cf(0.9, 0.95, 1.0, 0.15 + tw * 0.7));
            s.fill_ellipse(&ellipse(p.x, p.y, p.r, p.r), b);
        }
        if self.met_live {
            if !FxCache::linear(
                s,
                v2(0.0, 0.0),
                v2(1.0, 0.0),
                &[
                    stop(0.0, cf(1.0, 1.0, 1.0, 0.9)),
                    stop(1.0, cf(0.6, 0.8, 1.0, 0.0)),
                ],
                &mut cache.tail,
            ) {
                return;
            }
            let m = &self.met;
            let sp = (m.vx * m.vx + m.vy * m.vy).sqrt().max(1.0);
            let (ux, uy) = (m.vx / sp, m.vy / sp);
            let tail = 130.0;
            let life = m.life.clamp(0.0, 1.0);
            let g = &cache.tail[bucket(life)];
            s.draw_line(v2(m.x, m.y), v2(m.x - ux * tail, m.y - uy * tail), g, 2.2);
            if let Some(b) = cache.pen(s) {
                b.set_color(cf(1.0, 1.0, 1.0, 0.95 * life));
                s.fill_ellipse(&ellipse(m.x, m.y, m.r, m.r), b);
            }
        }
    }

    fn draw_matrix(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        const CELL: f32 = 20.0;
        if cache.glyphs.is_none() {
            cache.glyphs = (|| -> Option<Bitmap> {
                let bmp = s.create_bitmap_target().ok()?;
                s.with_target(&bmp, || {
                    let fmt = TextFormat::new("Segoe UI", 13.0);
                    let brush = s.create_solid_brush(cf(0.06, 0.9, 0.5, 1.0));
                    if let (Ok(fmt), Ok(brush)) = (fmt, brush) {
                        const GLYPHS: [char; 24] = [
                            'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ',
                            'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', '0', '1', '7', 'A', 'Z', 'Ω',
                        ];
                        for (i, g) in GLYPHS.iter().enumerate() {
                            let mut buf = [0u8; 4];
                            let text = g.encode_utf8(&mut buf);
                            s.draw_text(
                                text,
                                &fmt,
                                &Rect::new(i as f32 * CELL, 0.0, i as f32 * CELL + CELL, CELL),
                                &brush,
                            );
                        }
                    }
                });
                Some(bmp)
            })();
        }
        let n = self.target_count().min(self.ps.len());
        // `pen` needs &mut cache → detach the strip handle first (a COM clone).
        if let Some(strip) = cache.glyphs.clone() {
            for i in 0..n {
                let p = &self.ps[i];
                let cells = p.f2 as i32;
                for c in 0..cells {
                    let gy = p.y - c as f32 * CELL;
                    if gy < -CELL {
                        break;
                    }
                    let fade = 1.0 - c as f32 / cells as f32;
                    let src = (c + i as i32).rem_euclid(24);
                    s.draw_bitmap_src(
                        &strip,
                        &Rect::new(src as f32 * CELL, 0.0, src as f32 * CELL + CELL, CELL),
                        &Rect::new(p.x - 10.0, gy - CELL, p.x + 10.0, gy),
                        fade * 0.75,
                    );
                }
                let src = (p.k as i32).rem_euclid(24);
                s.draw_bitmap_src(
                    &strip,
                    &Rect::new(src as f32 * CELL, 0.0, src as f32 * CELL + CELL, CELL),
                    &Rect::new(p.x - 10.0, p.y - CELL, p.x + 10.0, p.y),
                    0.95,
                );
                if let Some(hb) = cache.pen(s) {
                    hb.set_color(cf(0.88, 0.97, 1.0, 0.35));
                    s.draw_line(v2(p.x, p.y), v2(p.x, p.y - CELL), hb, CELL * 0.8);
                }
            }
            return;
        }
        // Glyph strip unavailable: dashed streaks fallback.
        let n2 = n;
        for i in 0..n2 {
            let p = &self.ps[i];
            let Some(b) = cache.pen(s) else { return };
            let tl = p.f2 * CELL;
            b.set_color(cf(0.05, 0.65, 0.9, 0.5));
            s.draw_line(v2(p.x, p.y), v2(p.x, p.y - tl), b, 9.0);
            b.set_color(cf(0.88, 0.95, 1.0, 0.9));
            s.draw_line(v2(p.x, p.y), v2(p.x, p.y - CELL), b, 9.0);
        }
    }

    fn draw_ripples(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        for i in 0..n {
            let p = &self.ps[i];
            if p.life <= 0.0 {
                continue;
            }
            let Some(b) = cache.pen(s) else { return };
            let t = 1.0 - p.life;
            b.set_color(cf(0.56, 0.79, 0.99, 0.45 * p.life));
            s.draw_ellipse(&ellipse(p.x, p.y, p.r, p.r), b, (2.5 - 2.0 * t).max(0.5));
        }
    }

    fn draw_constellation(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        let (cx, cy) = self.cursor;
        for i in 0..n {
            let a = &self.ps[i];
            for j in (i + 1)..n {
                let b2 = &self.ps[j];
                let dx = a.x - b2.x;
                let dy = a.y - b2.y;
                let d2 = dx * dx + dy * dy;
                if d2 < 16900.0 {
                    let d = d2.sqrt();
                    let Some(p) = cache.pen(s) else { return };
                    p.set_color(cf(0.5, 0.7, 0.9, (1.0 - d / 130.0) * 0.4));
                    s.draw_line(v2(a.x, a.y), v2(b2.x, b2.y), p, 0.8);
                }
            }
            let dx = cx - a.x;
            let dy = cy - a.y;
            let d2 = dx * dx + dy * dy;
            if d2 < 32400.0 {
                let d = d2.sqrt();
                let Some(p) = cache.pen(s) else { return };
                p.set_color(cf(0.22, 0.74, 0.97, (1.0 - d / 180.0) * 0.6));
                s.draw_line(v2(a.x, a.y), v2(cx, cy), p, 1.0);
            }
        }
        for i in 0..n {
            let p = &self.ps[i];
            let Some(b) = cache.pen(s) else { return };
            b.set_color(cf(0.8, 0.9, 1.0, 0.7));
            s.fill_ellipse(&ellipse(p.x, p.y, p.r, p.r), b);
        }
    }

    /// Neon sparks along the cursor path — hue slides cyan → violet per spark;
    /// alpha is a fast fade-in / smooth fade-out envelope, radius shrinks.
    fn draw_magic_trail(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        let Some(b) = cache.pen(s) else { return };
        for i in 0..n {
            let p = &self.ps[i];
            if p.life <= 0.0 {
                continue;
            }
            let age = 1.0 - p.life;
            let a = (age * 6.0).min(1.0) * p.life * 0.9;
            let r = p.r * (0.35 + p.life * 0.65);
            // cyan (0.3, 0.9, 1.0) → violet (0.72, 0.38, 1.0)
            let t = p.ph;
            b.set_color(cf(0.3 + (0.72 - 0.3) * t, 0.9 + (0.38 - 0.9) * t, 1.0, a));
            s.fill_ellipse(&ellipse(p.x, p.y, r, r), b);
        }
    }

    /// Glass shards: thin rotating triangles catching the light.
    fn draw_shatter(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        let base = s.transform();
        for i in 0..n {
            let p = &self.ps[i];
            if p.life <= 0.0 {
                continue;
            }
            let Some(b) = cache.pen(s) else { return };
            let fade = (p.life / 1.4).clamp(0.0, 1.0);
            b.set_color(cf(0.82, 0.94, 1.0, p.a * fade));
            let r = p.r;
            let m = Matrix3x2::rotation(p.rot) * Matrix3x2::translation(p.x, p.y) * base;
            s.with_transform(&m, || {
                s.draw_line(v2(-r, -r * 0.5), v2(r * 0.9, -r * 0.3), b, 1.1);
                s.draw_line(v2(r * 0.9, -r * 0.3), v2(0.0, r), b, 1.1);
                s.draw_line(v2(0.0, r), v2(-r, -r * 0.5), b, 1.1);
            });
        }
    }

    /// Magnetic swarm dots — they brighten as the vortex winds them inward.
    fn draw_swarm(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        let (cx, cy) = self.cursor;
        let Some(b) = cache.pen(s) else { return };
        for i in 0..n {
            let p = &self.ps[i];
            let dx = cx - p.x;
            let dy = cy - p.y;
            let d2 = dx * dx + dy * dy;
            // Dots near the cursor glow hotter (vortex heat channel in `k`).
            let near = if d2 < 300.0 * 300.0 {
                1.0 - d2.sqrt() / 300.0
            } else {
                0.0
            };
            let a = (p.a * (0.55 + near * 1.2)).min(1.0);
            b.set_color(cf(0.45 + near * 0.4, 0.75 + near * 0.2, 1.0, a));
            let r = p.r * (1.0 + near * 0.9);
            s.fill_ellipse(&ellipse(p.x, p.y, r, r), b);
        }
        // Faint capture-radius halo while the swarm is stirred — reads as the
        // field the cursor carries; invisible when the pointer rests far away.
        let speed2 = {
            let dx = cx - self.cursor_prev.0;
            let dy = cy - self.cursor_prev.1;
            dx * dx + dy * dy
        };
        if speed2 > 4.0 {
            b.set_color(cf(0.4, 0.75, 1.0, 0.10));
            s.draw_ellipse(&ellipse(cx, cy, 300.0, 300.0), b, 1.0);
            b.set_color(cf(0.5, 0.85, 1.0, 0.22));
            s.draw_ellipse(&ellipse(cx, cy, 26.0, 26.0), b, 1.4);
        }
    }

    /// Cyber grid reveal: a dot-matrix lattice exists everywhere but only the
    /// 250 px soft mask around the cursor is lit.
    fn draw_grid_reveal(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        const STEP: f32 = 44.0;
        const MASK: f32 = 250.0;
        let (cx, cy) = self.cursor;
        let Some(b) = cache.pen(s) else { return };
        // Scan only the mask's bounding box — the lattice is infinite but the
        // visible cells are the ones within ~1.15 masks of the cursor.
        let reach = MASK + STEP;
        let x0 = (((cx - reach) / STEP).floor() as i32).max(0);
        let x1 = (((cx + reach) / STEP).ceil() as i32).min((self.w / STEP) as i32 + 1);
        let y0 = (((cy - reach) / STEP).floor() as i32).max(0);
        let y1 = (((cy + reach) / STEP).ceil() as i32).min((self.h / STEP) as i32 + 1);
        for gy in y0..y1 {
            for gx in x0..x1 {
                // Hex offset: alternate rows shift half a cell.
                let x = gx as f32 * STEP + if gy & 1 == 1 { STEP * 0.5 } else { 0.0 };
                let y = gy as f32 * STEP;
                let dx = x - cx;
                let dy = y - cy;
                let d2 = dx * dx + dy * dy;
                if d2 > MASK * MASK {
                    continue;
                }
                let falloff = 1.0 - d2.sqrt() / MASK;
                let a = falloff.powi(2) * 0.55;
                if a < 0.015 {
                    continue;
                }
                b.set_color(cf(0.35, 0.85, 1.0, a));
                s.fill_ellipse(&ellipse(x, y, 1.7, 1.7), b);
            }
        }
        // Soft mask edge — a whisper of a ring so the reveal has a boundary.
        b.set_color(cf(0.35, 0.85, 1.0, 0.12));
        s.draw_ellipse(&ellipse(cx, cy, MASK, MASK), b, 1.0);
    }

    /// Dust motes: ultra-small, faint, drifting up; `p.k` flares them where
    /// the repulsion sphere just pushed through.
    fn draw_dust(&self, s: &DrawingSession<'_>, cache: &mut FxCache) {
        let n = self.target_count().min(self.ps.len());
        let Some(b) = cache.pen(s) else { return };
        for i in 0..n {
            let p = &self.ps[i];
            let a = (p.a + p.k * 0.5).min(0.85);
            b.set_color(cf(0.85, 0.9, 1.0, a));
            s.fill_ellipse(&ellipse(p.x, p.y, p.r, p.r), b);
        }
    }
}
