#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}
#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}
