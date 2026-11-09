pub mod formats;

use std::io::Write as _;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn rustfmt(root: &Path, source: &str) -> Vec<u8> {
    let mut child = Command::new("rustfmt")
        .arg("+nightly")
        .arg("--edition")
        .arg("2024")
        .arg("--config-path")
        .arg(root.join("rustfmt.toml"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot run rustfmt");
    child
        .stdin
        .take()
        .expect("rustfmt has no stdin")
        .write_all(source.as_bytes())
        .expect("cannot write to rustfmt");
    let output = child.wait_with_output().expect("cannot wait for rustfmt");
    assert!(output.status.success(), "rustfmt failed");
    output.stdout
}
