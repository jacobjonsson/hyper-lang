mod ast;
mod codegen;

use std::env;
use std::fs::File;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub(crate) const SYNTAX_KINDS: &str = "crates/syntax/src/generated.rs";
pub(crate) const AST_TOKENS: &str = "crates/ast/src/generated/tokens.rs";
pub(crate) const AST_NODES: &str = "crates/ast/src/generated/nodes.rs";

fn main() -> Result<()> {
    let command = match env::args().nth(1) {
        Some(cmd) => cmd,
        None => panic!("no command given"),
    };

    if command == "codegen" {
        codegen::generate_syntax()?;
        return Ok(());
    }

    println!("{:?}", command);

    Ok(())
}

fn reformat(text: impl std::fmt::Display) -> std::io::Result<String> {
    let mut rustfmt = Command::new("rustfmt")
        .arg("--config-path")
        .arg(project_root().join("rustfmt.toml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text)?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let preamble = "Generated file, do not edit by hand";
    Ok(format!("//! {}\n\n{}", preamble, stdout))
}

pub(crate) fn project_root() -> PathBuf {
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn write_to_file(content: String, path: &Path) -> Result<()> {
    File::create(path)?.write_all(content.as_bytes())
}
