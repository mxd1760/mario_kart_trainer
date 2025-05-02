use chrono::NaiveDate;
use directories::ProjectDirs;
use rusqlite::{params,Connection,Result};
use serde::{Deserialize, Serialize};
use crate::{data_model::{
  database_queries::DatabaseTableItem, Category, FormattedTime, Run, TrackTime, User
}, shared_error::SharedError};


const DB_NAME: &str = "times.db";
const CFG_NAME: &str = "config.json";

#[derive(Serialize,Deserialize,Default)]
struct AppConfig{
  last_user:Option<String>,
}

impl AppConfig{
  fn load_config(path: &std::path::PathBuf)->AppConfig{
    if let Ok(data) = std::fs::read_to_string(path.join(CFG_NAME)){
      serde_json::from_str(&data).unwrap_or_default()
    } else {
      AppConfig::default()
    }
  }
  fn save_config(&self,path:&std::path::PathBuf){
    let json = serde_json::to_string_pretty(self).unwrap();
    std::fs::write(path.join(CFG_NAME),json).unwrap();
  }
}

pub struct DataManager{
  app_config:AppConfig,
  app_dir:std::path::PathBuf,
  pub conn:Connection,
  pub runs:Vec<Run>,
  pub users:Vec<User>,
}

impl DataManager{

  pub fn new()->DataManager{
    let dm = Self::init_db();
    dm
  }

  fn init_db()->DataManager{
    let app_dir = Self::get_app_dir().expect("Could not find valid local directory");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");
    let conn = Connection::open(app_dir.join(DB_NAME)).unwrap();
    let app_config = AppConfig::load_config(&app_dir);

    let _ = Run::create_database(&conn);
    let _ = User::create_database(&conn);

    DataManager { 
      app_config,
      app_dir,
      conn, 
      runs: vec![], 
      users: vec![] 
    }
  }

  pub fn prefetch(&mut self){
    self.runs = Run::database_get_all(&self.conn).unwrap();
    self.users = User::database_get_all(&self.conn).unwrap();
  }

  pub fn update_runs(&mut self){
    self.runs = Run::database_get_all(&self.conn).unwrap();
  }

  fn get_app_dir() -> Option<std::path::PathBuf>{
    ProjectDirs::from("com","marcus_doucette","mk_trainer")
      .map(|proj_dirs| proj_dirs.data_local_dir().to_path_buf())
  }

  fn set_config_user(&mut self,user:&String){
    self.app_config.last_user = Some(user.clone());
    self.app_config.save_config(&self.app_dir);
  }

  pub fn get_saved_username(&self)->String{
    self.app_config.last_user.clone().unwrap_or("".to_owned())
  }

  pub fn fetch_user(&mut self, username:&String) -> Result<User,SharedError>{
    self.set_config_user(username);
    if let Some(user) = User::check_if_exists(&self.conn, username.clone())?{
      Ok(user)
    }else{
      let new_user = User::database_insert(&self.conn, username.clone())?;
      Ok(new_user)
    }
  }
  pub fn fetch_user_by_id(&self, id:i64)->Result<User,SharedError>{
    if id==-1 {
      return Ok(User::NO_USER);
    }
    User::database_get_one(&self.conn, id)
  }

  pub fn insert_run(&mut self,category:Category,final_time:FormattedTime,user_id:i16,date:NaiveDate)->Result<(),SharedError> {
    Run::database_insert(&self.conn, category,final_time,user_id,date)?;
    self.update_runs();
    Ok(())
  }
}


