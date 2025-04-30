use chrono::TimeDelta;
use rusqlite::Row;

use super::{FormattedTime, Rules, Run, TTLapTime, TrackTime, Tracks};

pub trait DatabaseTableItem{
  fn create_database_query()->String;
  fn database_insert_query()->String;
  fn database_access_query()->String;
  fn database_delete_query()->String;
  fn database_replace_query()->String;
  fn instance_from_row(row:&Row)->Self;
}


impl DatabaseTableItem for TrackTime{
  fn create_database_query()->String {
    "CREATE TABLE IF NOT EXISTS track_times (
        id INTEGER PRIMARY KEY,
        run_id INTEGER FOREIGN KEY,
        track TEXT NOT NULL,
        rules TEXT NOT NULL,
        time TIME NOT NULL
    )".to_owned()
  }

  fn database_insert_query()->String {
    "INSERT INTO track_times".to_owned()
  }

  fn database_access_query()->String {
    todo!()
  }

  fn database_delete_query()->String {
      todo!()
  }

  fn database_replace_query()->String {
      todo!()
  }
  
  fn instance_from_row(row:&Row)->Self {
      TrackTime{
        id:1,
        run_id:1,
        track:Tracks::MarioKartStadium,
        rules:Rules{b_200cc:true,b_items:false},
        time:FormattedTime::new(0,2,0,0),
      }
    }
}

impl DatabaseTableItem for TTLapTime{
    fn create_database_query()->String {
        todo!()
    }

    fn database_insert_query()->String {
        todo!()
    }

    fn database_access_query()->String {
        todo!()
    }

    fn database_delete_query()->String {
        todo!()
    }

    fn database_replace_query()->String {
        todo!()
    }
    
    fn instance_from_row(row:&Row)->Self {
        todo!()
    }
}

impl DatabaseTableItem for Run{
    fn create_database_query()->String {
        todo!()
    }

    fn database_insert_query()->String {
        todo!()
    }

    fn database_access_query()->String {
        todo!()
    }

    fn database_delete_query()->String {
        todo!()
    }

    fn database_replace_query()->String {
        todo!()
    }
    
    fn instance_from_row(row:&Row)->Self {
        todo!()
    }
}




