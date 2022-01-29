/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    #[cfg(feature = "add-a-variant")]
    Debug,
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let level_msg = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warning => "WARNING",
        LogLevel::Info => "INFO",
        #[cfg(feature = "add-a-variant")]
        LogLevel::Debug => "DEBUG"
    };
    format!("[{}]: {}", level_msg, message)
}
#[cfg(feature = "add-a-variant")]
pub fn debug(message: &str) -> String {
    log(LogLevel::Debug, message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
