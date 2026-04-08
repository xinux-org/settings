use anyhow::{Result, anyhow};

pub mod error;
pub mod language;
pub mod modules;

// Source - https://stackoverflow.com/a/69812881
// Posted by yolenoyer, modified by community. See post 'Timeline' for change history
// Retrieved 2026-04-08, License - CC BY-SA 4.0

pub fn get_stdout(command: &str, args: &[&str]) -> Result<String> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        // Err(Error::CommandFailed {
        //     command: command.to_string(),
        //     code: output.status.code().unwrap_or(-1),
        // })
        Err(anyhow!("asjkldas;ldkfja;slkdjf"))
    }
}

pub fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
