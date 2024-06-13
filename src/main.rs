use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, process};

use clap::Parser;
use eyre::bail;
use nix::sys::memfd::{memfd_create, MemFdCreateFlag};

#[derive(Debug, Parser)]
struct Cli {
    /// Program used to interpret the script in the comments
    #[arg(short, long, default_value = "sh")]
    program: OsString,

    /// Path to the script, automatically passed by the OS
    file: PathBuf,
}

const BS_MARKER: &str = "IN_BANGSCRIPT";

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    if let Some(_) = env::var_os(BS_MARKER) {
        let mut cmd = Command::new(&cli.program);
        cmd.arg(&cli.file);
        bail!(cmd.exec());
    }

    env::set_var("FILE", &cli.file.canonicalize()?);
    env::set_var(BS_MARKER, "");

    let mut bangscript_content: Vec<u8> = Vec::new();

    let file = File::options().read(true).open(&cli.file)?;

    let bytes = file.bytes();

    let mut eol = false;
    for b in bytes {
        let b = b?;
        if b == b'\n' {
            eol = true;
        } else if eol == true {
            if b == b'#' {
                eol = false;
                continue;
            } else {
                break; // we are done
            }
        }
        bangscript_content.push(b);
    }

    let fd = memfd_create(c"script", MemFdCreateFlag::empty())?;

    let mut f = unsafe { File::from_raw_fd(fd.as_raw_fd()) };
    f.write_all(&bangscript_content)?;

    File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open("result-conent")?
        .write_all(&bangscript_content)?;

    let mut cmd = std::process::Command::new(&cli.program);
    cmd.arg(format!("/proc/self/fd/{}", fd.as_raw_fd()));
    let error = cmd.exec();
    bail!(error);
}
