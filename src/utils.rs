use std::path::PathBuf;
use std::fs;

pub fn templates_names(source_path: PathBuf) -> Vec<String> {
    fs::read_dir(&source_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

pub fn file_exists_in_home(filename: &str) -> bool {
    dirs::home_dir()
        .map(|home| home.join(filename))
        .is_some_and(|path| path.is_file())
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~") {
        let home = std::env::var("HOME").unwrap();
        // On gère à la fois "~" seul et "~/..."
        let stripped = stripped.strip_prefix('/').unwrap_or(stripped);
        PathBuf::from(home).join(stripped)
    } else {
        PathBuf::from(path)
    }
}
