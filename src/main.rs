// Mario Kart Time Tracker - Minimal "Walking Skeleton"

use eframe::egui;
use rusqlite::{params, Connection, Result};
use std::fmt;

#[derive(Debug)]
enum AppError {
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


#[derive(Debug)]
struct TrackTime {
    id: i32,
    track: String,
    time_seconds: f32,
    date: String,
}

fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS track_times (
            id INTEGER PRIMARY KEY,
            track TEXT NOT NULL,
            time_seconds REAL NOT NULL,
            date TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_time(conn: &Connection, track: &str, time_seconds: f32, date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO track_times (track, time_seconds, date) VALUES (?1, ?2, ?3)",
        params![track, time_seconds, date],
    )?;
    Ok(())
}

fn fetch_times(conn: &Connection) -> Result<Vec<TrackTime>> {
    let mut stmt = conn.prepare("SELECT id, track, time_seconds, date FROM track_times")?;
    let times = stmt.query_map([], |row| {
        Ok(TrackTime {
            id: row.get(0)?,
            track: row.get(1)?,
            time_seconds: row.get(2)?,
            date: row.get(3)?,
        })
    })?;

    Ok(times.filter_map(Result::ok).collect())
}

fn main() -> Result<(),AppError> {
    let conn = Connection::open("times.db")?;
    init_db(&conn)?;

    let app = TrackerApp::new(conn);
    let options = eframe::NativeOptions::default();
    eframe::run_native("MK8DX Tracker", options, Box::new(|_| Ok(Box::new(app))))?;
    Ok(())
}

struct TrackerApp {
    conn: Connection,
    track_input: String,
    time_input: String,
    date_input: String,
    records: Vec<TrackTime>,
}

impl TrackerApp {
    fn new(conn: Connection) -> Self {
        let records = fetch_times(&conn).unwrap_or_default();
        Self {
            conn,
            track_input: String::new(),
            time_input: String::new(),
            date_input: String::new(),
            records,
        }
    }
}

impl eframe::App for TrackerApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Add Time Trial Record");

            ui.horizontal(|ui| {
                ui.label("Track:");
                ui.text_edit_singleline(&mut self.track_input);
            });

            ui.horizontal(|ui| {
                ui.label("Time (seconds):");
                ui.text_edit_singleline(&mut self.time_input);
            });

            ui.horizontal(|ui| {
                ui.label("Date (YYYY-MM-DD):");
                ui.text_edit_singleline(&mut self.date_input);
            });

            if ui.button("Add Record").clicked() {
                if let (Ok(time), true) = (
                    self.time_input.parse::<f32>(),
                    !self.track_input.is_empty() && !self.date_input.is_empty(),
                ) {
                    if insert_time(&self.conn, &self.track_input, time, &self.date_input).is_ok() {
                        self.records = fetch_times(&self.conn).unwrap_or_default();
                        self.track_input.clear();
                        self.time_input.clear();
                        self.date_input.clear();
                    }
                }
            }

            ui.separator();
            ui.heading("Recorded Times");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for record in &self.records {
                    ui.label(format!("{} - {:.2}s on {}", record.track, record.time_seconds, record.date));
                }
            });
        });
    }
}        
