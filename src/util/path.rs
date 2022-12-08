use std::process::Command;

pub fn project_root() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("failed to execute get `project_root`");

    String::from_utf8(output.stdout).unwrap().trim().to_string()
    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn test_project_root() {
    //
    let current_project_root = "_";

    let root = project_root();

    assert_eq!(current_project_root, root);
}
