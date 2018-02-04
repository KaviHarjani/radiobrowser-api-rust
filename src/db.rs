extern crate mysql;
extern crate xml_writer;
extern crate chrono;

use db::mysql::Value;
use std;

pub struct Connection {
    pool: mysql::Pool
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Station {
    station_id: i32,
    changeuuid: String,
    stationuuid: String,
    name: String,
    url: String,
    homepage: String,
    favicon: String,
    tags: String,
    country: String,
    state: String,
    language: String,
    votes: i32,
    negativevotes: i32,
    lastchangetime: chrono::NaiveDateTime,
    ip: String
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Result1n {
    name: String,
    value: String,
    stationcount: u32,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    name: String,
    value: String,
    country: String,
    stationcount: u32,
}

pub fn serialize_result1n_list(type_str: &str, entries: Vec<Result1n>) -> std::io::Result<String> {
    let mut xml = xml_writer::XmlWriter::new(Vec::new());
    xml.begin_elem("result")?;
    for entry in entries{
        xml.begin_elem(type_str)?;
            xml.attr_esc("name", &entry.name)?;
            xml.attr_esc("value", &entry.value)?;
            let count_str = format!("{}", entry.stationcount);
            xml.attr_esc("stationcount", &count_str)?;
        xml.end_elem()?;
    }
    xml.end_elem()?;
    xml.close()?;
    xml.flush()?;
    Ok(String::from_utf8(xml.into_inner()).unwrap())
}

pub fn serialize_state_list(entries: Vec<State>) -> std::io::Result<String> {
    let mut xml = xml_writer::XmlWriter::new(Vec::new());
    xml.begin_elem("result")?;
    for entry in entries{
        xml.begin_elem("state")?;
            xml.attr_esc("name", &entry.name)?;
            xml.attr_esc("value", &entry.value)?;
            xml.attr_esc("country", &entry.country)?;
            let count_str = format!("{}", entry.stationcount);
            xml.attr_esc("stationcount", &count_str)?;
        xml.end_elem()?;
    }
    xml.end_elem()?;
    xml.close()?;
    xml.flush()?;
    Ok(String::from_utf8(xml.into_inner()).unwrap())
}

pub fn serialize_station_list(entries: Vec<Station>) -> std::io::Result<String> {
    let mut xml = xml_writer::XmlWriter::new(Vec::new());
    xml.begin_elem("result")?;
    for entry in entries{
        xml.begin_elem("station")?;
            let station_id_str = format!("{}", entry.station_id);
            xml.attr_esc("id", &station_id_str)?;
            xml.attr_esc("changeuuid", &entry.changeuuid)?;
            xml.attr_esc("stationuuid", &entry.stationuuid)?;
            xml.attr_esc("name", &entry.name)?;
            xml.attr_esc("url", &entry.url)?;
            xml.attr_esc("homepage", &entry.homepage)?;
            xml.attr_esc("favicon", &entry.favicon)?;
            xml.attr_esc("tags", &entry.tags)?;
            xml.attr_esc("country", &entry.country)?;
            xml.attr_esc("state", &entry.state)?;
            xml.attr_esc("language", &entry.language)?;
            let station_votes_str = format!("{}", entry.votes);
            xml.attr_esc("votes", &station_votes_str)?;
            let station_negativevotes_str = format!("{}", entry.negativevotes);
            xml.attr_esc("negativevotes", &station_negativevotes_str)?;
            let station_lastchangetime_str = format!("{}", entry.lastchangetime);
            xml.attr_esc("lastchangetime", &station_lastchangetime_str)?;
            xml.attr_esc("ip", &entry.ip)?;
        xml.end_elem()?;
    }
    xml.end_elem()?;
    xml.close()?;
    xml.flush()?;
    Ok(String::from_utf8(xml.into_inner()).unwrap())
}


impl Connection {
    pub fn get_stations_by_all(&self) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station ORDER BY Name");
        self.get_stations(query)
    }

    pub fn get_stations_by_name(&self, search: String) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station WHERE Name LIKE '%{search}%' ORDER BY Name", search = search);
        self.get_stations(query)
    }

    pub fn get_stations_by_id(&self, id: i32) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station WHERE StationID='{id}' ORDER BY Name", id = id);
        self.get_stations(query)
    }

    pub fn get_stations_topvote(&self) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station ORDER BY Votes DESC");
        self.get_stations(query)
    }

    pub fn get_stations_topclick(&self) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station ORDER BY clickcount DESC");
        self.get_stations(query)
    }

    pub fn get_stations_lastclick(&self) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station ORDER BY ClickTimestamp DESC");
        self.get_stations(query)
    }

    pub fn get_stations_lastchange(&self) -> Vec<Station> {
        let query : String;
        query = format!("SELECT StationID,ChangeUuid,StationUuid,Name,Url,Homepage,Favicon,Tags,Country,Subcountry,Language,Votes,NegativeVotes,Creation,Ip from Station ORDER BY Creation DESC");
        self.get_stations(query)
    }

    fn get_stations(&self, query: String) -> Vec<Station> {
        let stations: Vec<Station> =
        self.pool.prep_exec(query, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|mut row| {
                Station {
                    station_id: row.take("StationID").unwrap(),
                    changeuuid: row.take("ChangeUuid").unwrap(),
                    stationuuid: row.take("StationUuid").unwrap(),
                    name: row.take("Name").unwrap(),
                    url: row.take("Url").unwrap(),
                    homepage: row.take("Homepage").unwrap(),
                    favicon: row.take("Favicon").unwrap(),
                    tags: row.take("Tags").unwrap(),
                    country: row.take("Country").unwrap(),
                    state: row.take("Subcountry").unwrap(),
                    language: row.take("Language").unwrap(),
                    votes: row.take("Votes").unwrap(),
                    negativevotes: row.take("NegativeVotes").unwrap(),
                    lastchangetime: row.take("Creation").unwrap(),
                    ip: row.take("Ip").unwrap()
                }
            }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        }).unwrap(); // Unwrap `Vec<Payment>`

        stations
    }

    pub fn get_1_n(&self, column: &str, search: Option<String>, order : String, reverse : bool, hidebroken : bool) -> Vec<Result1n>{
        let query : String;
        let reverse_string = if reverse { "DESC" } else { "ASC" };
        let hidebroken_string = if hidebroken { " AND LastCheckOK=TRUE" } else { "" };
        match search{
            Some(value) => {
                query = format!("SELECT {column} AS value,{column},COUNT(*) AS stationcount FROM Station WHERE {column} LIKE '%{search}%' AND {column}<>'' {hidebroken} GROUP BY {column} ORDER BY {order} {reverse}", column = column, search = value, order = order, reverse = reverse_string, hidebroken = hidebroken_string);
            },
            None => {
                query = format!("SELECT {column} AS value,{column},COUNT(*) AS stationcount FROM Station WHERE {column}<>'' {hidebroken} GROUP BY {column} ORDER BY {order} {reverse}", column = column, order = order, reverse = reverse_string, hidebroken = hidebroken_string);
            }
        }

        let stations: Vec<Result1n> =
        self.pool.prep_exec(query, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (name, value, stationcount) = mysql::from_row(row);
                Result1n {
                    name: name,
                    value: value,
                    stationcount: stationcount,
                }
            }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        }).unwrap(); // Unwrap `Vec<Payment>`
        stations
    }

    pub fn get_states(&self, country: Option<String>, search: Option<String>, order : String, reverse : bool, hidebroken : bool) -> Vec<State>{
        let mut params: Vec<Value> = Vec::with_capacity(1);
        let reverse_string = if reverse { "DESC" } else { "ASC" };
        let hidebroken_string = if hidebroken { " AND LastCheckOK=TRUE" } else { "" };
        let country_string = match country {
            Some(c) => {
                params.push(c.into());
                format!(" AND Country=?")
            },
            None => "".to_string()
        };
        let search_string = match search {
            Some(c) => {
                params.push((format!("%{}%",c)).into());
                format!(" AND Subcountry LIKE ?")
            },
            None => "".to_string()
        };
        
        let mut my_stmt = self.pool.prepare(format!(r"SELECT Subcountry AS value,Subcountry,Country,COUNT(*) AS stationcount FROM Station WHERE Subcountry <> '' {country} {search} {hidebroken} GROUP BY Subcountry, Country ORDER BY {order} {reverse}",hidebroken = hidebroken_string, order = order, country = country_string, reverse = reverse_string, search = search_string)).unwrap();
        let my_results = my_stmt.execute(params);
        let mut states: Vec<State> = vec![];

        for my_result in my_results {
            for my_row in my_result {
                let mut row_unwrapped = my_row.unwrap();
                states.push(State{
                    name: row_unwrapped.take(0).unwrap_or("".into()),
                    value: row_unwrapped.take(1).unwrap_or("".into()),
                    country: row_unwrapped.take(2).unwrap_or("".into()),
                    stationcount: row_unwrapped.take(3).unwrap_or(0)
                });
            }
        };
        states
    }
}
pub enum DBError{
    ConnectionError (String)
}

impl std::fmt::Display for DBError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self{
            DBError::ConnectionError(ref v) => write!(f, "{}", v)
        }
    }
}

pub fn new(host: &String,port : i32, name: &String, user: &String, password: &String) -> Result<Connection, DBError> {
    let connection_string = format!("mysql://{}:{}@{}:{}/{}",user,password,host,port,name);
    println!("Connection string: {}", connection_string);
    let pool = mysql::Pool::new(connection_string);
    match pool {
        Ok(p) => Ok(Connection{pool: p}),
        Err(e) => Err(DBError::ConnectionError(e.to_string()))
    }
}