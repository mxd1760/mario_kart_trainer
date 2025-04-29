use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Sqlite(rusqlite::Error),
    Eframe(eframe::Error),
}

// Implement fmt::Display for AppError to make it easier to print errors
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Sqlite(err) => write!(f, "SQLite Error: {}", err),
            AppError::Eframe(err) => write!(f, "eFrame Error: {}", err),
        }
    }
}

// Implement From trait to easily convert other error types into AppError
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Sqlite(err)
    }
}

impl From<eframe::Error> for AppError {
    fn from(err: eframe::Error) -> Self {
        AppError::Eframe(err)
    }
}