#![feature(result_flattening)]

use cfg::Cfg;
use std::process::{exit, Command};
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use edit::edit;
use chrono::Local;
use utils::*;

mod cfg;
mod utils;

fn main() {
    let code = match run() {
        Ok(_) => 0,
        Err(e) => e.code()
    };
    exit(code)
}

fn run() -> Result<()> {
    let c = Cfg::new()?;
    dbg!(&c);

    let blog = edit("").map_err(UbError::Edit)?;
    let now = Local::now().format(&c.date_format_string);
    let header = c.header_template.replace("%t", &now.to_string());
    let entry = format!("{header}\n\n{blog}");
    for f in c.target_files {
        append_to_file(f, &entry)?
    }

    let script_exit = Command::new(c.script).status().map_err(UbError::ScriptStart)?;
    if let Some(status) = script_exit.code() {
        if status != 0 {
            return Err(UbError::ScriptNZExit(status))
        }
    }
    #[cfg(unix)]
    if let Some(_signal) = script_exit.signal() {
        return Err(UbError::ScriptTerminated)
    }


    Ok(())
}
