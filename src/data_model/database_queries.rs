use chrono::{NaiveDate, TimeDelta};
use rusqlite::{params, Connection, Row};

use crate::shared_error::SharedError;

use super::{Category, FormattedTime, Rules, Run, TTLapTime, TrackTime, Tracks};

pub trait DatabaseTableItem{
  fn create_database(conn:&Connection)->Result<(),SharedError>;
  //fn database_insert()->bool;
  fn database_get_one()->Result<Self,SharedError> where Self: Sized;
  fn database_get_all(conn:&Connection)->Result<Vec<Self>,SharedError> where Self: Sized;
  fn database_delete()->Result<(),SharedError>;
  fn database_replace()->Result<(),SharedError>;
  //fn instance_from_row(row:&Row)->Self;
}


impl DatabaseTableItem for TrackTime{
    fn create_database(conn:&Connection)->Result<(),SharedError> {
        todo!()
    }

    fn database_get_one()->Result<Self,SharedError> where Self: Sized {
        todo!()
    }

    fn database_get_all(conn:&Connection)->Result<Vec<Self>,SharedError> where Self: Sized {
        todo!()
    }

    fn database_delete()->Result<(),SharedError> {
        todo!()
    }

    fn database_replace()->Result<(),SharedError> {
        todo!()
    }
}

impl DatabaseTableItem for TTLapTime{
    fn create_database(conn:&Connection)->Result<(),SharedError> {
        todo!()
    }

    fn database_get_one()->Result<Self,SharedError> where Self: Sized {
        todo!()
    }

    fn database_get_all(conn:&Connection)->Result<Vec<Self>,SharedError> where Self: Sized {
        todo!()
    }

    fn database_delete()->Result<(),SharedError> {
        todo!()
    }

    fn database_replace()->Result<(),SharedError> {
        todo!()
    }
}

impl DatabaseTableItem for Run{
    fn create_database(conn:&Connection) -> Result<(),SharedError> {
      conn.execute(
        "CREATE TABLE IF NOT EXISTS runs (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          category TEXT NOT NULL,
          final_time TEXT NOT NULL,
          user_id INTEGER NOT NULL,
          date TEXT);",())?;
      Ok(())
    }
    
    fn database_get_one() -> Result<Run, SharedError> {
        todo!()
    }
    
    fn database_get_all(conn:&Connection) -> Result<std::vec::Vec<Run>, SharedError> {
      //println!("select");
      let mut stmt = conn.prepare("SELECT id, category, final_time, user_id, date FROM runs;").unwrap_or_else(|e|{
        panic!("Failed select query: {}",e)
      });
      let runs = stmt.query_map([],|row|{
        let date_str:String = row.get(4)?;
        let cat_str:String = row.get(1)?;
        let time_str:String = row.get(2)?;
        Ok(Run{
          id:row.get(0)?,
          category:Category::from_db_str(cat_str.as_str()),
          final_time:FormattedTime::from_db_str(time_str.as_str()),
          user_id:row.get(3)?,
          date:NaiveDate::parse_from_str(&date_str,"%Y-%m-%d").map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            size_of_val(&e), 
            rusqlite::types::Type::Text, 
            Box::new(e)))?
        })
      })?;
      let mut out = vec![];
      for run in runs{
        let r = run.unwrap();
        //println!("   {:?}",r);
        out.push(r);
      }
      Ok(out)
    }
    
    fn database_delete() -> Result<(), SharedError> {
        todo!()
    }
    
    fn database_replace() -> Result<(), SharedError> {
        todo!()
    }
}
impl Run{
  pub fn database_insert(conn:&Connection, 
    category:Category,
    final_time:FormattedTime,
    user_id:i16,
    date:NaiveDate
  )->Result<(),SharedError>{
    //println!("insert");
    match conn.execute(
      "INSERT INTO runs (category, final_time, user_id, date) VALUES (?1, ?2, ?3, ?4);",
      params![category.to_db_str(),final_time.to_db_str(),user_id,date.format("%Y-%m-%d").to_string()]
    ){
        Ok(rows) => (),//println!("inserted {} row(s)",rows),
        Err(err) => println!("insert failed: {}",err),
    }
    Ok(())
  }
}




