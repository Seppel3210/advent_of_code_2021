use std::path::Path;

pub fn input(path: impl AsRef<Path>) -> String {
    let dir: &Path = "input".as_ref();
    std::fs::read_to_string(dir.join(path)).unwrap()
}
