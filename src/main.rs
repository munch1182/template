use std::{fs, path::Path, process::Command};

pub fn main() {
    let curr_dir = std::env::current_dir().unwrap();
    write_json(&curr_dir.join("./dist/{{project-name}}.json")).unwrap();
    start_dev_server();
}

fn start_dev_server() {
    let result = Command::new("cmd").args(["/c pnpm run dev"]).output();
    println!("dev server result: {}", result.is_ok());
}

fn write_json(path: &Path) -> std::io::Result<()> {
    let s = r#"
{
    "name": "{{project-name}}",
    "version": "0.1.0",
    "libfile": "../target/debug/{{project-name}}.dll",
    "uiurl": "http://localhost:3001/{{project-name}}/"
}
"#;

    fs::write(path, s)?;
    Ok(())
}
