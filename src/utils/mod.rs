use thiserror::Error;

pub mod error;
pub mod language;
pub mod modules;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },
}

// Source - https://stackoverflow.com/a/69812881
// Posted by yolenoyer, modified by community. See post 'Timeline' for change history
// Retrieved 2026-04-08, License - CC BY-SA 4.0

pub fn parse_dconf(mut str: String) -> String {
    // removes file protocol
    if str.starts_with("file://") {
        str = str[7..].into();
    }

    str
}
