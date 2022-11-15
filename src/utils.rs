use std::io::{self, Write};
use toml;
use std::fs::File;
use thiserror::Error;

#[derive(Debug, Error)]
#[repr(u8)]
pub enum UbError {
    // file errors
    #[error("could not read config file: {0}")]
    CfgFile(io::Error),
    #[error("could not write config file: {0}")]
    TargetFile(io::Error),
    #[error("could not expand path: {0}")]
    Tilde(io::Error),
    #[error("editor failure: {0}")]
    Edit(io::Error),

    // script errors
    #[error("could not start script: {0}")]
    ScriptStart(io::Error),
    #[error("script exited with nonzero status")]
    ScriptNZExit(i32),
    #[error("script terminated by signal")]
    ScriptTerminated,

    // config errors
    #[error("could not parse config file: {0}")]
    ParseFail(toml::de::Error),
    #[error("config file is invalid")]
    Invalid,
    #[error("date format is invalid")]
    BadTimeFormat,
    #[error("no target files")]
    NoTarget,
    #[error("target file(s) were invalid")]
    BadTarget,
    #[error("no script")]
    NoScript,
    #[error("invalid script")]
    BadScript,
    #[error("bad template")]
    BadTemplate,
}
pub type Result<T> = std::result::Result<T, UbError>;

impl UbError {
    pub fn code(&self) -> i32 {
        match self {
            Self::CfgFile(_) | Self::TargetFile(_) => 128,
            Self::Tilde(_) => 129,
            Self::Edit(_) => 130,
            Self::ScriptStart(_) => 131,
            Self::ScriptNZExit(e) => *e,
            Self::ScriptTerminated => 132,
            _ => 133
        }
    }
}

/// appends 3 newlines and then the given text to a file
pub fn append_to_file<P: AsRef<std::path::Path>>(path: P, text: &str) -> Result<()> {
    let mut f = File::options().create(true).append(true).open(path).map_err(UbError::TargetFile)?;
    f.write(b"\n\n\n").map_err(UbError::TargetFile)?;
    f.write(text.as_bytes()).map_err(UbError::TargetFile)?;

    Ok(())
}

