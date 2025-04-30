use chrono::Local;
use eframe::egui;
use eframe::egui::Ui;
use rusqlite::Connection;
use strum::IntoEnumIterator;
use crate::data_model;
use crate::app_database;
use crate::data_model::database_queries::DatabaseTableItem;
use crate::data_model::Category;
use crate::data_model::Cups;
use crate::data_model::Run;
use crate::data_model::Tracks;
use crate::data_model::FormattedTime;
use crate::data_model::User;

pub struct TrackerAppView {
  conn: Connection,
  cc:bool,
  items:bool,
  category:Category,
  track:Tracks,
  cup:Cups,
  user:String,

  prev_cc:bool,
  prev_items:bool,
  prev_category:Category,
  prev_track:Tracks,
  prev_cup:Cups,
  prev_user:String,

  time:FormattedTime,
  records: Vec<data_model::Run>,
}

impl TrackerAppView {
  pub fn new(conn: Connection) -> Self {
      let records = Run::database_get_all(&conn).unwrap();
      Self {
          conn,
          cc:true,
          items:false,
          category:Category::TimeTrial(true,Tracks::MarioKartStadium),
          track:Tracks::MarioKartStadium,
          cup:Cups::MushroomCup,
          user:"".to_owned(),

          prev_cc:true,
          prev_items:false,
          prev_category:Category::TimeTrial(true,Tracks::MarioKartStadium),
          prev_track:Tracks::MarioKartStadium,
          prev_cup:Cups::MushroomCup,
          prev_user:"".to_owned(),

          time:FormattedTime::new(0,0,0,0),
          records,
      }
  }
}

impl eframe::App for TrackerAppView {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("Add Time Trial Record");

          ui.horizontal(|ui|{
            ui.label("Speed:");
            ui.selectable_value(&mut self.cc,true,"200cc");
            ui.selectable_value(&mut self.cc,false,"150cc");
          });

          ui.horizontal(|ui|{
            ui.label("Items: ");
            ui.selectable_value(&mut self.items,false,"Only Coins");
            ui.selectable_value(&mut self.items,true,"Default Items");
          });

          ui.horizontal(|ui|{
            ui.label("Run Category");
            egui::ComboBox::from_id_salt("category")
              .height(1000.0)
              .selected_text(format!("{}",self.category))
              .show_ui(ui,|ui|{
                for category in Category::iter(){
                  if ui.selectable_value(&mut self.category,category.clone(),category.to_string()).clicked() {
                    self.track = self.cup.get_tracks()[0].clone()
                  }
                }
            });
          });
          
          match &self.category{
            Category::TimeTrial(_, _) => {
              ui.horizontal(|ui| {
                ui.label("Cup:");
                egui::ComboBox::from_id_salt("track cup")
                  .height(1000.0)
                  .selected_text(format!("{}",self.cup))
                  .show_ui(ui,|ui|{
                    for cup in Cups::iter(){
                      if ui.selectable_value(&mut self.cup, cup.clone(), cup.to_string()).clicked() {
                        self.track = cup.get_tracks()[0].clone()
                      }
                    } 
                });
              });
              ui.horizontal(|ui|{
                ui.label("Track:");
                egui::ComboBox::from_id_salt("track")
                  .height(1000.0)
                  .selected_text(self.track.to_string())
                  .show_ui(ui,|ui|{
                    for t in self.cup.get_tracks(){
                      ui.selectable_value(&mut self.track, t.clone(), t.to_string());
                    }
                });
              });
              small_time(ui,&mut self.time)
            },
            Category::SingleCup(_, _) => {
              ui.horizontal(|ui| {
                ui.label("Track:");
                egui::ComboBox::from_id_salt("track")
                  .height(1000.0)
                  .selected_text(format!("{}",self.cup))
                  .show_ui(ui,|ui|{
                    for cup in Cups::iter(){
                      ui.selectable_value(&mut self.cup, cup.clone(), cup.to_string());
                    } 
                });
              });
              med_time(ui, &mut self.time);
            },
            Category::All96(_) | Category::Bcp48(_) | Category::Og48(_) => {
              big_time(ui,&mut self.time);
            },
            _=>{
              med_time(ui,&mut self.time);
            }
          }

          let mut reset_time = false;

          if self.cc != self.prev_cc{
            self.prev_cc = self.cc;
            reset_time = true;
          }

          if self.items != self.prev_items{
            self.prev_items = self.items;
            reset_time = true;
          }
          if self.category != self.prev_category{
            self.prev_category = self.category.clone();
            reset_time = true;
          }
          if self.track != self.prev_track{
            self.prev_track = self.track.clone();
            reset_time = true;
          }
          if self.cup != self.prev_cup{
            self.prev_cup = self.cup.clone();
            reset_time = true;
          }

          if ui.button("Add Record").clicked() {
            if self.time.not_zero() {
              println!("TODO Input time {:?}",self.time);
              if Run::database_insert(&self.conn,self.category.clone(),self.time.clone(),User::NO_USER.id,Local::now().date_naive()).is_ok() {
                self.records = Run::database_get_all(&self.conn).unwrap();
                reset_time = true;
              }
            }
          }

          if reset_time{
            self.time = FormattedTime::new(0,0,0,0);
          }


          ui.separator();
          ui.heading("Recorded Times");

          egui::ScrollArea::vertical().show(ui, |ui| {
              for record in &self.records {
                  ui.label(record.get_display_str());
              }
          });
      });
  }
}


fn big_time(ui: &mut Ui,time:&mut FormattedTime){
  ui.horizontal(|ui|{
    ui.label("Time Input: ");
    ui.add(egui::DragValue::new(&mut time.hours).range(0..=10)
      .speed(0.1)
    );
    ui.label("h");

    ui.add(egui::DragValue::new(&mut time.minutes).range(0..=59)
      .speed(0.2)
    );
    ui.label("m");

    ui.add(egui::DragValue::new(&mut time.seconds).range(0..=59)
      .speed(0.2)
    );
    ui.label("s");

    ui.add(egui::DragValue::new(&mut time.millis).range(0..=999)
      .speed(1.0)
    );
    ui.label("ms");
  });

  
  ui.label(format!("Time: {:01}:{:02}:{:02}:{:03}", time.hours, time.minutes, time.seconds, time.millis));
}

fn med_time(ui:&mut Ui, time:&mut FormattedTime){ 
  ui.horizontal(|ui|{
    ui.label("Time Input: ");
    ui.add(egui::DragValue::new(&mut time.minutes).range(0..=59)
      .speed(0.2)
    );
    ui.label("m");

    ui.add(egui::DragValue::new(&mut time.seconds).range(0..=59)
      .speed(0.2)
    );
    ui.label("s");

    ui.add(egui::DragValue::new(&mut time.millis).range(0..=999)
      .speed(1.0)
    );
    ui.label("ms");
  });
  ui.label(format!("Time: {:02}:{:02}:{:03}", time.minutes, time.seconds, time.millis));
}

fn small_time(ui: &mut Ui,time:&mut FormattedTime ){
  ui.horizontal(|ui|{
    ui.label("Time Input: ");
    ui.add(egui::DragValue::new(&mut time.minutes).range(0..=10)
      .speed(0.1)
    );
    ui.label("m");

    ui.add(egui::DragValue::new(&mut time.seconds).range(0..=59)
      .speed(0.2)
    );
    ui.label("s");

    ui.add(egui::DragValue::new(&mut time.millis).range(0..=999)
      .speed(1.0)
    );
    ui.label("ms");
  });
  ui.label(format!("Time: {:02}:{:02}:{:03}", time.minutes, time.seconds, time.millis));
}