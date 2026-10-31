//! Platform-independent core: data model, layout normalization, rule engine, persistence.
//! This crate must never depend on Windows APIs so it can be unit-tested anywhere.

pub mod adaptive;
pub mod base64;
pub mod brand;
pub mod color_tint;
pub mod config_store;
pub mod date_group;
pub mod file_explorer_themes;
pub mod geometry;
pub mod hotkey;
pub mod i18n;
pub mod model;
pub mod notification_center_themes;
pub mod peco_theme;
pub mod rules;
pub mod start_menu_themes;
pub mod taskbar_themes;
pub mod updates;

pub use config_store::{ConfigStore, Lint, LintLevel, LoadOutcome};
pub use date_group::{CivilDate, DateBucket, date_bucket};
pub use model::*;
pub use rules::{Cond, Decision, ItemFacts, Rule, RuleSet, StrOp, Target, TypeCategory};
