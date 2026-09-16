pub fn file_exists_in_home(filename: &str) -> bool {
    dirs::home_dir()
        .map(|home| home.join(filename))
        .is_some_and(|path| path.is_file())
}
