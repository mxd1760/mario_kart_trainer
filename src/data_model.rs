#[derive(Debug)]
pub struct TrackTime {
    id: i32,
    track: String,
    time_seconds: f32,
    date: String,
}

impl TrackTime{
  pub fn get_display_str(&self) -> String {
    format!("{} - {:.2}s on {}", self.track, self.time_seconds, self.date)
  }
  pub fn new(id:i32,track:String,time_seconds:f32,date:String)->TrackTime{
    TrackTime { id, track, time_seconds, date }
  }
}