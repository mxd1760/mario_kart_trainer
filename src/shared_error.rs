use std::fmt;

#[derive(Debug)]
pub enum SharedError {
    Sqlite(rusqlite::Error),
    Eframe(eframe::Error),
}

// Implement fmt::Display for AppError to make it easier to print errors
impl fmt::Display for SharedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SharedError::Sqlite(err) => write!(f, "SQLite Error: {}", err),
            SharedError::Eframe(err) => write!(f, "eFrame Error: {}", err),
        }
    }
}

// Implement From trait to easily convert other error types into AppError
impl From<rusqlite::Error> for SharedError {
    fn from(err: rusqlite::Error) -> Self {
        SharedError::Sqlite(err)
    }
}

impl From<eframe::Error> for SharedError {
    fn from(err: eframe::Error) -> Self {
        SharedError::Eframe(err)
    }
}