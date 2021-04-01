use std::collections::HashMap;
use std::env;
use std::process::Command;

use cargo::core::{maybe_allow_nightly_features, Workspace};
use cargo::core::compiler::{CompileMode, Metadata};
use cargo::ops::{self, CompileOptions};
use cargo::util::config::Config;

fn compile() -> (String, Option<String>) {
    let current_dir = env::current_dir().unwrap();
    let manifest_path = format!("{}/Cargo.toml", current_dir.to_str().unwrap());

    let mut config = Config::default().unwrap();
    config.configure(0, false, Some("always"), false, false, false, &None, &[], &[]).unwrap();

    let ws = Workspace::new(manifest_path.as_ref(), &config).unwrap();

    let compile_options = CompileOptions::new(&config, CompileMode::Build).unwrap();
    let result = ops::compile(&ws, &compile_options).unwrap();

    let kind = compile_options.build_config.requested_kinds.first().unwrap();
    let rustc_output = result.root_output
        .get(kind)
        .map(|k| k.to_str())
        .flatten()
        .unwrap();
    let build_script_output = extract_out_dir(&result.extra_env);

    (rustc_output.into(), build_script_output.map(String::from))
}

fn link(rustc_output: &str, build_script_output: Option<&str>) -> String {
    let libkernel = format!("{}/libkernel.a", rustc_output);
    let asm_output = build_script_output.map(|s| format!("{}/assembly", s));
    let asm_object_files = enumerate_asm_object_files(asm_output);

    let kernel_image = format!("{}/kernel.bin", rustc_output);
    let ld_executable = if cfg!(windows) { "ld.lld.exe" } else { "ld.lld" };
    let result = Command::new(ld_executable)
        .args(&["-o", &kernel_image])
        .arg("-Tlinker.ld")
        .arg(&libkernel)
        .args(asm_object_files)
        .output()
        .unwrap();

    let success_output = String::from_utf8(result.stdout).unwrap();
    let error_output = String::from_utf8(result.stderr).unwrap();
    match result.status {
        s if s.success() => print!("{}", success_output),
        _ => print!("{}", error_output)
    }

    kernel_image
}

fn extract_out_dir(env: &HashMap<Metadata, Vec<(String, String)>>) -> Option<&str> {
    env
        .values()
        .flatten()
        .filter(|e| e.0 == "OUT_DIR")
        .map(|e| e.1.as_str())
        .next()
}

fn enumerate_asm_object_files(asm_path: Option<String>) -> Vec<String> {
    let mut object_files = vec![];

    if let Some(asm_path) = asm_path {
        let object_file = format!("{}/boot.o", asm_path);
        object_files.push(object_file);
    }

    object_files
}

fn run(kernel_image: &str, debugging: bool) {
    let mut command = Command::new("qemu-system-i386");
    let mut command = command.args(&["-kernel", kernel_image]);

    if debugging {
        command = command.arg("-s").arg("-S");
    }

    if cfg!(windows) {
        command = command.args(&["-L", "C:/Program Files/qemu"]);
    }

    let _ = command.spawn().unwrap().wait();
}

fn main() {
    maybe_allow_nightly_features();

    let (rustc_output, build_script_output) = compile();
    let kernel_image = link(&rustc_output, build_script_output.as_ref().map(|s| s.as_str()));

    match env::args().collect::<Vec<String>>() {
        args if args.contains(&"--debug".into()) => run(&kernel_image, true),
        args if args.contains(&"--run".into()) => run(&kernel_image, false),
        _ => {}
    }
}
