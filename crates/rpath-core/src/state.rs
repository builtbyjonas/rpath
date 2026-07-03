use crate::model::{EnvironmentPlan, EnvironmentSnapshot, RpathError, RpathResult};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn state_dir() -> RpathResult<PathBuf> {
    let base = dirs::data_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| RpathError::State("could not resolve a user data directory".to_string()))?;
    Ok(base.join("rpath"))
}

pub fn snapshot_from_plan(
    plan: &EnvironmentPlan,
    reason: impl Into<String>,
) -> EnvironmentSnapshot {
    let now = unix_now();
    EnvironmentSnapshot {
        id: format!("{now}-{}", std::process::id()),
        created_at_unix: now,
        shell: plan.shell,
        platform: plan.platform.clone(),
        variables: plan.variables.clone(),
        path: plan.path.clone(),
        path_entries: plan.path_entries.clone(),
        reason: reason.into(),
    }
}

pub fn save_snapshot(snapshot: &EnvironmentSnapshot) -> RpathResult<PathBuf> {
    let dir = state_dir()?.join("snapshots");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", snapshot.id));
    fs::write(&path, serde_json::to_string_pretty(snapshot)?)?;
    Ok(path)
}

pub fn load_snapshot(id: &str) -> RpathResult<EnvironmentSnapshot> {
    let path = state_dir()?.join("snapshots").join(format!("{id}.json"));
    let raw = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn delete_snapshot(id: &str) -> RpathResult<()> {
    let path = state_dir()?.join("snapshots").join(format!("{id}.json"));
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn list_snapshots() -> RpathResult<Vec<EnvironmentSnapshot>> {
    list_from_dir(state_dir()?.join("snapshots"))
}

pub fn save_version(plan: &EnvironmentPlan) -> RpathResult<PathBuf> {
    let snapshot = snapshot_from_plan(plan, "version");
    let dir = state_dir()?.join("versions");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", snapshot.id));
    fs::write(&path, serde_json::to_string_pretty(&snapshot)?)?;
    Ok(path)
}

pub fn list_versions() -> RpathResult<Vec<EnvironmentSnapshot>> {
    list_from_dir(state_dir()?.join("versions"))
}

fn list_from_dir(dir: PathBuf) -> RpathResult<Vec<EnvironmentSnapshot>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut snapshots: Vec<EnvironmentSnapshot> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(entry.path())?;
        snapshots.push(serde_json::from_str(&raw)?);
    }
    snapshots.sort_by_key(|snapshot| snapshot.created_at_unix);
    Ok(snapshots)
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
