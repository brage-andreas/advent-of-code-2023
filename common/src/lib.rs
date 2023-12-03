use std::fs;

fn generate_path_from_day(day: u8) -> String {
    format!("day-{:02}/src/day-{:02}-input.txt", day, day)
}

pub fn read_from_file(path_from_workspace_root: &str) -> String {
    match fs::read_to_string(&path_from_workspace_root) {
        Ok(result) => result,
        Err(error) => panic!(
            "Could not read from local file `{}`\n  Message: \"{:?}\"",
            &path_from_workspace_root, &error
        ),
    }
}

pub fn read_input(day: u8) -> String {
    let path_from_workspace_root = generate_path_from_day(day);

    read_from_file(&path_from_workspace_root)
}
