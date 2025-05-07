use std::{env, fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

pub fn create_temp_dir() -> std::io::Result<TempDir> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_path = env::temp_dir().join(format!("glob-test-{}", timestamp));
    fs::create_dir_all(&temp_path)?;
    Ok(TempDir { path: temp_path })
}

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}