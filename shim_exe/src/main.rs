use std::path::Path;
use std::process::Command;

fn invoke_current_bat() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 1 || args.is_empty() {
        panic!("No cmd driver exe");
    }
    let driver_exe = args[0].clone();
    let shim_name = Path::new(&driver_exe)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let args = args
        .iter()
        .skip(1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();

    let state = Command::new("mise")
        .arg("x")
        .arg("--")
        .arg(shim_name)
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .unwrap();

    std::process::exit(state.code().unwrap_or(1));
}

fn main() {
    invoke_current_bat();
}
