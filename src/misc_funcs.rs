use std::{collections::HashMap, fs, io::Write};

use toml::Table;

use crate::data_types::{
    data_info_types::DataInfo,
    tank_data_types::{Tank, TankData},
};

fn parse_config() -> toml::map::Map<std::string::String, toml::Value> {
    let path = std::path::Path::new("./config.toml");
    if path.exists() {
        let str_data = fs::read_to_string(path).expect("Couldn't read the data file.");
        str_data.parse::<Table>().unwrap()
    } else {
        panic!("Can't find the config file!")
    }
}

#[allow(dead_code)]
pub fn update_data() -> (DataInfo, HashMap<String, Tank>) {
    let tank_data_path = std::path::Path::new("./tank_data.json");
    let data_info_path = std::path::Path::new("./data_info.json");

    if tank_data_path.exists() && data_info_path.exists() {
        let _ = trash::delete(tank_data_path);
        let _ = trash::delete(data_info_path);
        read_data()
    } else {
        read_data()
    }
}

pub fn read_data() -> (DataInfo, HashMap<String, Tank>) {
    let tank_data_path = std::path::Path::new("./tank_data.json");
    let data_info_path = std::path::Path::new("./data_info.json");
    if tank_data_path.exists() && data_info_path.exists() {
        let str_tank_data =
            fs::read_to_string(tank_data_path).expect("Error reading the tank data");
        let str_data_info =
            fs::read_to_string(data_info_path).expect("Error reading the data info");

        let tank_data: TankData =
            serde_json::from_str(&str_tank_data).expect("Error reading tank data.");
        let data_info: DataInfo =
            serde_json::from_str(&str_data_info).expect("Error reading data info");

        (data_info, tank_data.tanks)
    } else {
        let config = parse_config();

        let client = reqwest::blocking::Client::new();
        let data_info_target = format!(
            "https://api.wotblitz.eu/wotb/encyclopedia/info/?application_id={}",
            config["AppId"].as_str().unwrap()
        );
        let tank_data_target = format!(
            "https://api.wotblitz.eu/wotb/encyclopedia/vehicles/?application_id={}",
            config["AppId"].as_str().unwrap()
        );

        let data_info = client
            .get(data_info_target)
            .send()
            .expect("Error getting data info")
            .json::<DataInfo>()
            .expect("Error parsing data info");

        let tank_data = client
            .get(tank_data_target)
            .send()
            .expect("Error getting tank data")
            .json::<TankData>()
            .expect("Error parsing tank data");

        let mut data_info_file =
            fs::File::create(data_info_path).expect("Error creating data info file");
        let mut tank_data_file =
            fs::File::create(tank_data_path).expect("Error creating tank data file");

        let _ = data_info_file.write_all(
            serde_json::to_string(&data_info)
                .expect("Cannot write data info")
                .as_bytes(),
        );
        let _ = tank_data_file.write_all(
            serde_json::to_string(&tank_data)
                .expect("Cannot write tank data")
                .as_bytes(),
        );

        (data_info, tank_data.tanks)
    }
}
