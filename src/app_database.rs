use rusqlite::{params,Connection,Result};
use crate::data_model::TrackTime;

pub fn init_db(conn: &Connection) -> Result<()> {
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

pub fn insert_time(conn: &Connection, track: &str, time_seconds: f32, date: &str) -> Result<()> {
  conn.execute(
      "INSERT INTO track_times (track, time_seconds, date) VALUES (?1, ?2, ?3)",
      params![track, time_seconds, date],
  )?;
  Ok(())
}

pub fn fetch_times(conn: &Connection) -> Result<Vec<TrackTime>> {
  let mut stmt = conn.prepare("SELECT id, track, time_seconds, date FROM track_times")?;
  let times = stmt.query_map([], |row| {
    Ok(TrackTime::new( 
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
    ))
  })?;
  Ok(times.filter_map(Result::ok).collect())
}