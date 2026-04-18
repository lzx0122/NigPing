use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to read hostname: {0}")]
    Hostname(#[from] std::io::Error),
    #[error("device name is not valid UTF-8")]
    InvalidHostnameEncoding,
    #[error("already monitoring a process")]
    AlreadyMonitoring,
    #[error("process '{0}' was not found; start the game first")]
    ProcessNotFound(String),
    #[error("no active monitoring session")]
    NoActiveMonitoring,
    #[error("{0}")]
    Msg(String),
}

pub fn to_cmd_err(e: AppError) -> String {
    e.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_to_cmd_err_msg() {
        let err = AppError::Msg("test error".to_string());
        assert_eq!(to_cmd_err(err), "test error");
    }

    #[test]
    fn test_to_cmd_err_already_monitoring() {
        let err = AppError::AlreadyMonitoring;
        assert_eq!(to_cmd_err(err), "already monitoring a process");
    }

    #[test]
    fn test_to_cmd_err_process_not_found() {
        let err = AppError::ProcessNotFound("TslGame.exe".to_string());
        assert_eq!(to_cmd_err(err), "process 'TslGame.exe' was not found; start the game first");
    }

    #[test]
    fn test_to_cmd_err_hostname() {
        let io_err = Error::new(ErrorKind::NotFound, "file not found");
        let err = AppError::Hostname(io_err);
        assert_eq!(to_cmd_err(err), "failed to read hostname: file not found");
    }

    #[test]
    fn test_to_cmd_err_invalid_hostname_encoding() {
        let err = AppError::InvalidHostnameEncoding;
        assert_eq!(to_cmd_err(err), "device name is not valid UTF-8");
    }

    #[test]
    fn test_to_cmd_err_no_active_monitoring() {
        let err = AppError::NoActiveMonitoring;
        assert_eq!(to_cmd_err(err), "no active monitoring session");
    }
}
