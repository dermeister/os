use std::process::Command;
use std::fs;
use std::env;

fn assembly() {
    println!("cargo:rerun-if-changed=src/boot.asm");

    let out_dir = env::var("OUT_DIR").unwrap();
    let asm_dir = format!("{}/assembly", out_dir);
    fs::create_dir_all(&asm_dir).expect("Error creating assembly folder");

    Command::new("nasm")
        .arg("-f elf32")
        .arg(format!("-o {}/boot.o", &asm_dir))
        .arg("src/boot.asm")
        .output()
        .expect("Error compiling assembly");
}

fn main() {
    assembly();
}
