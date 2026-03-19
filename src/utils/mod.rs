use thiserror::Error;

pub mod language;
pub mod error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },
}

fn get_stdout(command: &str, args: &[&str]) -> Result<String> {
    use std::process::Command;

    let output = Command::new(command).args(args).output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().into())
    } else {
        Err(Error::CommandFailed {
            command: command.to_string(),
            code: output.status.code().unwrap_or(-1),
        })
    }
}


fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
