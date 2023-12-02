use std::fs;

pub fn read_from_file(path_from_workspace_root: &str) -> String {
    match fs::read_to_string(&path_from_workspace_root) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `{}`\n  Message: \"{:?}\"",
            &path_from_workspace_root, &error
        ),
    }
}
