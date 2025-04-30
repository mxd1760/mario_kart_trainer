use std::borrow::Cow;

use strum_macros::{Display, EnumIter, EnumString};

use chrono::{TimeDelta,NaiveDate};


pub mod database_queries;


#[derive(Debug,Clone)]
pub struct FormattedTime{
  pub hours:i8,
  pub minutes:i8,
  pub seconds:i8,
  pub millis:i16,
}

impl FormattedTime{
  pub fn new(hours:i8,minutes:i8,seconds:i8,millis:i16)->Self{
    FormattedTime { hours, minutes, seconds, millis }
  }
  pub fn not_zero(&self)->bool{
    return !(self.hours==0 && self.minutes==0 && self.seconds==0 && self.millis==0)
  }
  pub fn to_display_str(&self)->String{
    match self.hours{
      0=>format!("{:02}:{:02}:{:03}", self.minutes, self.seconds, self.millis),
      _=>format!("{:02}:{:02}:{:02}:{:03}", self.hours, self.minutes, self.seconds, self.millis)
    }
  }
  pub fn to_db_str(&self)->String{
    format!("{:02}:{:02}:{:02}:{:03}", self.hours, self.minutes, self.seconds, self.millis)
  }
  pub fn from_db_str(val:&str)->Self{
    let items:Vec<&str> = val.split(":").collect();
    FormattedTime { 
      hours: items[0].parse().unwrap(), 
      minutes: items[1].parse().unwrap(), 
      seconds: items[2].parse().unwrap(), 
      millis: items[3].parse().unwrap() }
  }
}


#[derive(Debug)]
pub struct TrackTime {
  id: i64,
  run_id: i32,
  track: Tracks,
  rules: Rules,
  time: FormattedTime,
}

impl TrackTime{
  pub fn get_display_str(&self) -> String {
    //format!("{} - ", self.track, self.)
    "".to_owned()
  }
  pub fn new(id:i64, run_id:i32, track:Tracks,rules:Rules,time:FormattedTime)->TrackTime{
    TrackTime { id, run_id, track, rules,time }
  }
}

#[derive(Debug)]
pub struct TTLapTime{
  id:i64,
  runid: i32,
  track:Tracks,
  b_200cc:bool,
  lap:i8,
  time:FormattedTime,
}

impl TTLapTime{

}

#[derive(Debug)]
pub struct Run{
  id:i32,
  category:Category,
  final_time:FormattedTime,
  user_id:i16,
  date:NaiveDate,
}

impl Run{
  pub fn get_display_str(&self)->String{
    format!("{} in {} on {}",self.final_time.to_display_str(),self.category.to_display_str(),self.date)
  }
}


pub struct User{
  pub id:i16,
  name:Cow<'static,str>,
}

impl User{
  pub const NO_USER:User = User{id:-1,name:Cow::Borrowed("No one")};
}


#[derive(Debug,PartialEq,Clone)]
pub struct Rules{
  b_200cc:bool,
  b_items:bool,
}

impl Default for Rules{
  fn default() -> Self {
      Self { b_200cc: Default::default(), b_items: Default::default() }
  }
}

impl Rules{
  fn to_string(&self)->String{
    format!("{} - {}",if self.b_200cc {"200cc"} else {"150cc"}, if self.b_items {"Items"} else {"No Items"})
  }
  fn from_str(val:&str)->Self{
    let items:Vec<&str> = val.split("-").collect();
    let cc = match items[0].trim(){
      "200cc" => true,
      _=>false
    };
    let it = match items[1].trim(){
      "Items" => true,
      _=>false
    };
    Rules{b_200cc:cc,b_items:it}
  }
}

#[derive(Debug,EnumIter,Display,PartialEq,Clone)]
pub enum Category{
  #[strum(to_string="Time Trial")]
  TimeTrial(bool,Tracks), // is 200cc,track
  #[strum(to_string="Single Cup")]
  SingleCup(Rules,Cups),
  #[strum(to_string="96 Tracks")]
  All96(Rules),
  #[strum(to_string="48 Base Game")]
  Og48(Rules),
  #[strum(to_string="48 BCP")]
  Bcp48(Rules),
  #[strum(to_string="Nitro Cups")]
  Nitro(Rules),
  #[strum(to_string="Retro Cups")]
  Retro(Rules),
  #[strum(to_string="Bonus Cups")]
  Bonus(Rules),
  #[strum(to_string="DLC Waves 1-2")]
  Dlc1_2(Rules),
  #[strum(to_string="DLC Waves 3-4")]
  Dlc3_4(Rules),
  #[strum(to_string="DLC Waves 5-6")]
  Dlc5_6(Rules),
}

impl Category{
  pub fn to_db_str(&self)->String{
    match self{
      Category::TimeTrial(_200cc, track) => format!("TT - {} - {}",track, if *_200cc {"200cc"} else {"150cc"}),
      Category::SingleCup(rules, cup) =>format!("SC - {} - {}",cup,rules.to_string()),
      Category::All96(rules) => format!("96 - {}",rules.to_string()),
      Category::Og48(rules) => format!("OG - {}",rules.to_string()),
      Category::Bcp48(rules) => format!("BP - {}",rules.to_string()),
      Category::Nitro(rules) => format!("NI - {}",rules.to_string()),
      Category::Retro(rules) => format!("RE - {}",rules.to_string()),
      Category::Bonus(rules) => format!("BO - {}",rules.to_string()),
      Category::Dlc1_2(rules) => format!("D1 - {}",rules.to_string()),
      Category::Dlc3_4(rules) => format!("D2 - {}",rules.to_string()),
      Category::Dlc5_6(rules) => format!("D3 - {}",rules.to_string()),
    }
  }
  pub fn to_display_str(&self)->String{
    match self{
      Category::TimeTrial(_200cc, track) => format!("{} - {} - {}",self,track, if *_200cc {"200cc"} else {"150cc"}),
      Category::SingleCup(rules, cup) =>format!("{} - {} - {}",self,cup,rules.to_string()),
      Category::All96(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Og48(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Bcp48(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Nitro(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Retro(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Bonus(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Dlc1_2(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Dlc3_4(rules) => format!("{} - {}",self,rules.to_string()),
      Category::Dlc5_6(rules) => format!("{} - {}",self,rules.to_string()),
    }
  }
  pub fn from_db_str(val: &str)->Self{
    let id = &val[..2];
    match id{
      "TT"=>{
        let items:Vec<&str> = val[5..].split("-").collect();
        let mb_track = items[0].trim();
        println!("mb track: {}",mb_track);
        let track:Tracks = mb_track.parse::<Tracks>().unwrap();
        let cc = match items[1].trim(){
          "200cc" => true,
          _=>false
        };

        Category::TimeTrial(cc, track)
      },
      "SC"=>{
        let items:Vec<&str> = val[5..].split("-").collect();
        let cup:Cups = items[0].trim().parse::<Cups>().unwrap();
        let cc = match items[1].trim(){
          "200cc" => true,
          _=>false
        };
        let it = match items[2].trim(){
          "Items" => true,
          _=>false
        };
        Category::SingleCup(Rules{b_200cc:cc,b_items:it},cup)
      },
      "96"=>Category::All96(Rules::from_str(&val[5..])),
      "OG"=>Category::Og48(Rules::from_str(&val[5..])),
      "BP"=>Category::Bcp48(Rules::from_str(&val[5..])),
      "NI"=>Category::Nitro(Rules::from_str(&val[5..])),
      "RE"=>Category::Retro(Rules::from_str(&val[5..])),
      "BO"=>Category::Bonus(Rules::from_str(&val[5..])),
      "D1"=>Category::Dlc1_2(Rules::from_str(&val[5..])),
      "D2"=>Category::Dlc3_4(Rules::from_str(&val[5..])),
      "D3"=>Category::Dlc5_6(Rules::from_str(&val[5..])),
      _=>panic!("invalid category in db")
    }
  }
}

#[derive(Debug,EnumIter,Display,EnumString,PartialEq,Clone)]
#[strum(serialize_all="title_case")]
pub enum Cups{
  MushroomCup,    FlowerCup,    StarCup,      SpecialCup,
  ShellCup,       BananaCup,    LeafCup,      LightningCup,
  EggCup,         TriforceCup,  CrossingCup,  BellCup,
  GoldenDashCup,  LuckyCatCup,  TurnipCup,    PropellerCup,
  RockCup,        MoonCup,      FruitCup,     BoomerangCup,
  FeatherCup,     CherryCup,    AcornCup,     SpinyCup
}

impl Cups{
  pub fn get_tracks(&self) -> Vec<Tracks>{
    match self{
        Cups::MushroomCup =>    vec![Tracks::MarioKartStadium, Tracks::WaterPark,        Tracks::SweetSweetCanyon, Tracks::ThwompRuins],
        Cups::FlowerCup =>      vec![Tracks::MarioCircuit,     Tracks::ToadHarbor,       Tracks::TwistedMansion,   Tracks::ShyGuyFalls,],
        Cups::StarCup =>        vec![Tracks::SunshineAirport,  Tracks::DolphinShoals,    Tracks::Electrodrome,     Tracks::MountWario,],
        Cups::SpecialCup =>     vec![Tracks::CloudtopCruise,   Tracks::BoneDryDunes,     Tracks::BowsersCastle,    Tracks::RainbowRoad,],
        Cups::ShellCup =>       vec![Tracks::MooMooMeadows,    Tracks::BGAMarioCircuit,  Tracks::CheepCheepBeach,  Tracks::ToadsTurnpike,],
        Cups::BananaCup =>      vec![Tracks::DryDryDesert,     Tracks::DonutPlains3,     Tracks::RoyalRaceway,     Tracks::DKJungle,],
        Cups::LeafCup =>        vec![Tracks::WarioStadium,     Tracks::SherbetLand,      Tracks::MusicPark,        Tracks::YoshiValley,],
        Cups::LightningCup =>   vec![Tracks::TickTockClock,    Tracks::PiranhaPlantSlide,Tracks::GrumbleVolcano,   Tracks::N64RainbowRoad,],
        Cups::EggCup =>         vec![Tracks::YoshiCircuit,     Tracks::ExcitebikeArena,  Tracks::DragonDriftway,   Tracks::MuteCity,],
        Cups::TriforceCup =>    vec![Tracks::WariosGoldmine,   Tracks::SNESRainbowRoad,  Tracks::IceIceOutpost,    Tracks::HyruleCircuit,],
        Cups::CrossingCup =>    vec![Tracks::BabyPark,         Tracks::CheeseLand,       Tracks::WildWoods,        Tracks::AnimalCrossing,],
        Cups::BellCup =>        vec![Tracks::NeoBowserCity,    Tracks::RibbonRoad,       Tracks::SuperBellSubway,  Tracks::BigBlue,],
        Cups::GoldenDashCup =>  vec![Tracks::ParisPromenade,   Tracks::ToadCircuit,      Tracks::ChocoMountain,    Tracks::CoconutMall,],
        Cups::LuckyCatCup =>    vec![Tracks::TokyoBlur,        Tracks::ShroomRidge,      Tracks::SkyGarden,        Tracks::NinjaHideaway,],
        Cups::TurnipCup =>      vec![Tracks::NewYorkMinute,    Tracks::MarioCircuit3,    Tracks::KalimariDesert,   Tracks::WaluigiPinball,],
        Cups::PropellerCup =>   vec![Tracks::SydneySprint,     Tracks::SnowLand,         Tracks::MushroomGorge,    Tracks::SkyHighSundae,],
        Cups::RockCup =>        vec![Tracks::LondonLoop,       Tracks::BooLake,          Tracks::RockRockMountain, Tracks::MapleTreeway,],
        Cups::MoonCup =>        vec![Tracks::BerlinByways,     Tracks::PeachGardens,     Tracks::MerryMountain,    Tracks::_3DSRainbowRoad,],
        Cups::FruitCup =>       vec![Tracks::AmsterdamDrift,   Tracks::RiversidePark,    Tracks::DKSummit,         Tracks::YoshisIsland,],
        Cups::BoomerangCup =>   vec![Tracks::BangkokRush,      Tracks::DSMarioCircuit,   Tracks::WaluigiStadium,   Tracks::SingaporeSpeedway,],
        Cups::FeatherCup =>     vec![Tracks::AthensDash,       Tracks::DaisyCruiser,     Tracks::MoonviewHighway,  Tracks::SqueakyCleanSprint,],
        Cups::CherryCup =>      vec![Tracks::LosAngelesLaps,   Tracks::SunsetWilds,      Tracks::KoopaCape,        Tracks::VancouverVelocity,],
        Cups::AcornCup =>       vec![Tracks::RomeAvanti,       Tracks::DKMountain,       Tracks::DaisyCircuit,     Tracks::PiranhaPlantCove,],
        Cups::SpinyCup =>       vec![Tracks::MadridDrive,      Tracks::RosalinasIceWorld,Tracks::BowsersCastle3,   Tracks::WiiRainbowRoad,],
    }
      
  }
}

impl Default for Cups {
    fn default() -> Self {
      Cups::MushroomCup
    }
}

#[derive(Debug,EnumIter,Display,EnumString,PartialEq,Clone)]
#[strum(serialize_all="title_case")]
pub enum Tracks{
  MarioKartStadium, WaterPark,        SweetSweetCanyon, ThwompRuins,
  MarioCircuit,     ToadHarbor,       TwistedMansion,   ShyGuyFalls,
  SunshineAirport,  DolphinShoals,    Electrodrome,     MountWario,
  CloudtopCruise,   BoneDryDunes,     BowsersCastle,    RainbowRoad,
  MooMooMeadows,    BGAMarioCircuit,  CheepCheepBeach,  ToadsTurnpike,
  DryDryDesert,     DonutPlains3,     RoyalRaceway,     DKJungle,
  WarioStadium,     SherbetLand,      MusicPark,        YoshiValley,
  TickTockClock,    PiranhaPlantSlide,GrumbleVolcano,   N64RainbowRoad,
  YoshiCircuit,     ExcitebikeArena,  DragonDriftway,   MuteCity,
  WariosGoldmine,   SNESRainbowRoad,  IceIceOutpost,    HyruleCircuit,
  BabyPark,         CheeseLand,       WildWoods,        AnimalCrossing,
  NeoBowserCity,    RibbonRoad,       SuperBellSubway,  BigBlue,

  ParisPromenade,   ToadCircuit,      ChocoMountain,    CoconutMall,
  TokyoBlur,        ShroomRidge,      SkyGarden,        NinjaHideaway,
  NewYorkMinute,    MarioCircuit3,    KalimariDesert,   WaluigiPinball,
  SydneySprint,     SnowLand,         MushroomGorge,    SkyHighSundae,
  LondonLoop,       BooLake,          RockRockMountain, MapleTreeway,
  BerlinByways,     PeachGardens,     MerryMountain,    _3DSRainbowRoad,
  AmsterdamDrift,   RiversidePark,    DKSummit,         YoshisIsland,
  BangkokRush,      DSMarioCircuit,   WaluigiStadium,   SingaporeSpeedway,
  AthensDash,       DaisyCruiser,     MoonviewHighway,  SqueakyCleanSprint,
  LosAngelesLaps,   SunsetWilds,      KoopaCape,        VancouverVelocity,
  RomeAvanti,       DKMountain,       DaisyCircuit,     PiranhaPlantCove,
  MadridDrive,      RosalinasIceWorld,BowsersCastle3,   WiiRainbowRoad,  
}

impl Default for Tracks{
  fn default() -> Self {
     Tracks::MarioKartStadium 
  }
}