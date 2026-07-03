use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, str::FromStr};

pub type RpathResult<T> = Result<T, RpathError>;

#[derive(Debug, thiserror::Error)]
pub enum RpathError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported shell: {0}")]
    UnsupportedShell(String),
    #[error("invalid shell: {0}")]
    InvalidShell(String),
    #[error("environment collection failed: {0}")]
    Collection(String),
    #[error("state error: {0}")]
    State(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShellKind {
    Cmd,
    PowerShell,
    Pwsh,
    Bash,
    Zsh,
    Fish,
}

impl ShellKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cmd => "cmd",
            Self::PowerShell => "powershell",
            Self::Pwsh => "pwsh",
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
        }
    }

    pub fn is_windows_shell(self) -> bool {
        matches!(self, Self::Cmd | Self::PowerShell | Self::Pwsh)
    }

    pub fn is_posix_shell(self) -> bool {
        matches!(self, Self::Bash | Self::Zsh | Self::Fish)
    }
}

impl fmt::Display for ShellKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ShellKind {
    type Err = RpathError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "cmd" | "cmd.exe" => Ok(Self::Cmd),
            "powershell" | "powershell.exe" | "windows-powershell" => Ok(Self::PowerShell),
            "pwsh" | "pwsh.exe" | "powershell-core" => Ok(Self::Pwsh),
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "fish" => Ok(Self::Fish),
            other => Err(RpathError::InvalidShell(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PathSource {
    Current,
    System,
    User,
    RegistryMachine,
    RegistryUser,
    File(String),
    ShellProfile(String),
    PackageManager(String),
    Integration(String),
    Snapshot(String),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathEntry {
    pub raw: String,
    pub expanded: String,
    pub source: PathSource,
    pub exists: bool,
    pub valid: bool,
    pub critical: bool,
    pub duplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStats {
    pub original_path_entries: usize,
    pub computed_path_entries: usize,
    pub duplicates_removed: usize,
    pub invalid_entries: usize,
    pub added_entries: usize,
    pub removed_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub path: Option<String>,
}

impl Diagnostic {
    pub fn info(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Info,
            code: code.into(),
            message: message.into(),
            path: None,
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            code: code.into(),
            message: message.into(),
            path: None,
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            code: code.into(),
            message: message.into(),
            path: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPlan {
    pub shell: ShellKind,
    pub platform: String,
    pub variables: BTreeMap<String, String>,
    pub path: String,
    pub path_entries: Vec<PathEntry>,
    pub diagnostics: Vec<Diagnostic>,
    pub stats: PlanStats,
    pub generated_at_unix: u64,
}

impl EnvironmentPlan {
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSnapshot {
    pub id: String,
    pub created_at_unix: u64,
    pub shell: ShellKind,
    pub platform: String,
    pub variables: BTreeMap<String, String>,
    pub path: String,
    pub path_entries: Vec<PathEntry>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffReport {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub reordered: Vec<String>,
    pub unchanged_count: usize,
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub shell: Option<ShellKind>,
    pub no_dedupe: bool,
    pub strict: bool,
    pub validate_paths: bool,
    pub remove_invalid: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            shell: None,
            no_dedupe: false,
            strict: false,
            validate_paths: true,
            remove_invalid: false,
        }
    }
}
