use std::fs;

pub fn read_from_file(path_from_crate_root: &str) -> String {
    match fs::read_to_string(&path_from_crate_root) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `path_from_crate_root`={}\n  Message: \"{:?}\"",
            &path_from_crate_root, &error
        ),
    }
}
