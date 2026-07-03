//! Core environment planning for rpath.
//!
//! This crate is intentionally shell-agnostic. It discovers environment sources,
//! builds a safe PATH plan, tracks diagnostics, and stores snapshots/versions.

pub mod collect;
pub mod diff;
pub mod model;
pub mod state;

pub use collect::{build_environment_plan, detect_shell};
pub use diff::{diff_path_entries, diff_plan_against_current};
pub use model::{
    BuildOptions, Diagnostic, DiagnosticSeverity, DiffReport, EnvironmentPlan, EnvironmentSnapshot,
    PathEntry, PathSource, PlanStats, RpathError, RpathResult, ShellKind,
};
pub use state::{
    delete_snapshot, list_snapshots, list_versions, load_snapshot, save_snapshot, save_version,
    snapshot_from_plan, state_dir,
};
