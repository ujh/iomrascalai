use std::env;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()
        .expect("failed to execute git rev-parse");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("REVISION");
    let mut f = File::create(&dest_path).unwrap();
    let data = String::from_utf8(output.stdout).unwrap();
    let without_newline = data.lines().next().unwrap();
    f.write_all(without_newline.as_bytes()).unwrap();
}
