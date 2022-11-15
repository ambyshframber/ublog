use lazy_static::lazy_static;
use std::collections::HashMap;
use toml::Value;
use std::fs::read_to_string;
use std::path::PathBuf;
use expanduser::expanduser;

use crate::utils::*;

const CFG_FILE: &str = "~/.ublogrc";

const RFC_2822: &str = "%a, %e %b %Y %T %z";

lazy_static! {
    static ref DATE_FMTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ISO-8601", "%Y-%m-%dT%H:%M:%S%:z");
        m.insert("RFC-2822", RFC_2822);
        m.insert("unix", "%s");
        m
    };
}

#[derive(Debug)]
pub struct Cfg {
    pub target_files: Vec<PathBuf>,
    pub date_format_string: String,
    pub header_template: String,
    pub script: PathBuf,
}
impl Cfg {
    pub fn new() -> Result<Cfg> {
        let cfg_path = expanduser(CFG_FILE).expect("failed to expand tilde!");
        let cfg_str = read_to_string(cfg_path).map_err(UbError::CfgFile)?;

        let cfg_v = cfg_str.parse::<Value>().map_err(UbError::ParseFail)?;

        let t = cfg_v.as_table().ok_or(UbError::Invalid)?;

        let targets = t.get("target").ok_or(UbError::NoTarget)?;
        let target_files = match targets {
            Value::String(s) => vec![PathBuf::from(expanduser(s).map_err(UbError::Tilde)?)],
            Value::Array(a) => {
                a.iter().map(|v| v.as_str().ok_or(UbError::BadTarget).map(|s| expanduser(s).map_err(UbError::Tilde))).map(Result::flatten).collect::<Result<Vec<PathBuf>>>()?
            }

            _ => return Err(UbError::BadTarget)
        };

        let date_fmt = if let Some(v) = t.get("time_format") {
            v.as_str().ok_or(UbError::BadTimeFormat)?
        }
        else {
            RFC_2822
        };
        let date_format_string = String::from(*DATE_FMTS.get(date_fmt).unwrap_or(&date_fmt));

        let header_template = if let Some(header_template) = t.get("header_template") {
            header_template.as_str().ok_or(UbError::BadTemplate)?
        }
        else {
            "%t"
        }.into();

        let script = expanduser(t.get("script").ok_or(UbError::NoScript)?.as_str().ok_or(UbError::BadScript)?).map_err(UbError::Tilde)?;

        Ok(Cfg {
            target_files, date_format_string, script, header_template
        })
    }
}
