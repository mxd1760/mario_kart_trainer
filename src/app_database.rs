use rusqlite::{params,Connection,Result};
use crate::data_model::{
  database_queries::DatabaseTableItem, Run, TrackTime
};

pub fn init_db(conn: &Connection) -> Result<()> {
  let _ = Run::create_database(conn);
  Ok(())
}

// pub fn insert_time(conn: &Connection, track: &str, time_seconds: f32, date: &str) -> Result<()> {
//   conn.execute(
//       "INSERT INTO track_times (track, time_seconds, date) VALUES (?1, ?2, ?3)",
//       params![track, time_seconds, date],
//   )?;
//   Ok(())
// }

// pub fn fetch_times<T:DatabaseTableItem>(conn: &Connection) -> Result<Vec<T>> {
//   let mut stmt = conn.prepare("SELECT id, track, time_seconds, date FROM track_times")?;
//   let times = stmt.query_map([], |row| {
//     Ok(T::instance_from_row(row))
//   })?;
//   Ok(times.filter_map(Result::ok).collect())
// }