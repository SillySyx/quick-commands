pub struct AppError {
    pub message: &'static str,
}

impl AppError {
    pub fn new(message: &'static str) -> Self {
        Self {
            message,
        }
    }
}