use eframe::egui;
use rusqlite::Connection;
use crate::data_model;
use crate::app_database;

pub struct TrackerAppView {
  conn: Connection,
  track_input: String,
  time_input: String,
  date_input: String,
  records: Vec<data_model::TrackTime>,
}

impl TrackerAppView {
  pub fn new(conn: Connection) -> Self {
      let records = app_database::fetch_times(&conn).unwrap_or_default();
      Self {
          conn,
          track_input: String::new(),
          time_input: String::new(),
          date_input: String::new(),
          records,
      }
  }
}

impl eframe::App for TrackerAppView {
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
                  if app_database::insert_time(&self.conn, &self.track_input, time, &self.date_input).is_ok() {
                      self.records = app_database::fetch_times(&self.conn).unwrap_or_default();
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
                  ui.label(record.get_display_str());
              }
          });
      });
  }
}