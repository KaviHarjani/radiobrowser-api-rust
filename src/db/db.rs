use check::models::StationItem;
use check::models::StationCheckItemNew;
use std::error::Error;

pub trait DbConnection {
    fn delete_never_working(&mut self, hours: u32) -> Result<(), Box<dyn Error>>;
    fn delete_were_working(&mut self, hours: u32) -> Result<(), Box<dyn Error>>;
    fn delete_old_checks(&mut self, hours: u32) -> Result<(), Box<dyn Error>>;
    fn delete_old_clicks(&mut self, hours: u32) -> Result<(), Box<dyn Error>>;
    fn get_station_count_broken(&mut self) -> Result<u32, Box<dyn Error>>;
    fn get_station_count_working(&mut self) -> Result<u32, Box<dyn Error>>;
    fn get_station_count_todo(&mut self, hours: u32) -> Result<u32, Box<dyn Error>>;
    fn get_checks(&mut self, hours: u32, source: &str) -> Result<u32, Box<dyn Error>>;
    fn get_deletable_never_working(&mut self, hours: u32) -> Result<u32, Box<dyn Error>>;
    fn get_deletable_were_working(&mut self, hours: u32) -> Result<u32, Box<dyn Error>>;

    fn insert_check(&mut self, item: &StationCheckItemNew) -> Result<(), Box<dyn std::error::Error>>;
    fn update_station(&mut self, item: &StationCheckItemNew) -> Result<(), Box<dyn std::error::Error>>;
    fn get_stations_to_check(&mut self, hours: u32, itemcount: u32) -> Vec<StationItem>;
}