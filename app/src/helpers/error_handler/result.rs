use crate::helpers::error_handler::AppError;

pub type Result<T> = std::result::Result<T, AppError>;

pub trait ResultCtx<T> {
    fn ctx(self, label: &'static str, msg: impl Into<String>) -> Result<T>;
}

impl<T> ResultCtx<T> for Result<T> {
    fn ctx(self, label: &'static str, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| e.with_ctx(label, msg))
    }
}
