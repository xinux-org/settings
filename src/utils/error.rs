use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },
}
