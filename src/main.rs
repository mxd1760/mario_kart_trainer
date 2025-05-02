// Mario Kart Time Tracker - Minimal "Walking Skeleton"


use rusqlite::{Connection, Result};
use tracker_app_view::TrackerAppView;
use app_database::DataManager;

mod shared_error;
mod app_database;
mod tracker_app_view;
mod data_model;







fn main() -> Result<(),shared_error::SharedError> {
    let dm = DataManager::new();

    let app = TrackerAppView::new(dm);
    let options = eframe::NativeOptions::default();
    eframe::run_native("MK8DX Tracker", options, Box::new(|_| Ok(Box::new(app))))?;
    Ok(())
}

        
