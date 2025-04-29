// Mario Kart Time Tracker - Minimal "Walking Skeleton"


use rusqlite::{Connection, Result};
use tracker_app_view::TrackerAppView;

mod app_error;
mod database;
mod tracker_app_view;
mod data_model;




fn main() -> Result<(),app_error::AppError> {
    let conn = Connection::open("times.db")?;
    database::init_db(&conn)?;

    let app = TrackerAppView::new(conn);
    let options = eframe::NativeOptions::default();
    eframe::run_native("MK8DX Tracker", options, Box::new(|_| Ok(Box::new(app))))?;
    Ok(())
}

        
