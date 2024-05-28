#![allow(dead_code, unused)]

use std::collections::HashMap;

use crate::data_types::tank_data_types::{Nation, Tank, TankType};

// TODO: Make this shit work.
// ! The main feature.
pub fn calculate_xp() {}

pub fn tank_selection(tanks: (&mut Option<Tank>, &mut Option<Tank>), selection: Tank) {
    let (first_tank, second_tank) = tanks;

    if first_tank.is_none() {
        *first_tank = Some(selection);
    } else if second_tank.is_none() {
        *second_tank = Some(selection);
    } else {
        *first_tank = Some(selection);
        *second_tank = None;
    }
}

pub fn search_database(
    filters: (String, Option<Nation>, Option<TankType>, bool),
    database: HashMap<String, Tank>,
    search_result: &mut Vec<Tank>,
) {
    let (search_term, nation, tank_type, ignore_case) = filters;
    for (id, tank) in database {
        let tank_name = if ignore_case {
            tank.name.to_lowercase()
        } else {
            tank.name.clone()
        };
        let search_term = if ignore_case {
            search_term.to_lowercase()
        } else {
            search_term.clone()
        };

        if nation.is_some() && tank_type.is_some() {
            let tank_type = tank_type.unwrap();
            let nation = nation.unwrap();
            if tank_name.contains(&search_term)
                && (tank.tank_type == tank_type && tank.nation == nation)
            {
                search_result.push(tank)
            }
        } else if nation.is_some() {
            let nation = nation.unwrap();
            if tank.nation == nation && tank_name.contains(&search_term) {
                search_result.push(tank)
            }
        } else if tank_type.is_some() {
            let tank_type = tank_type.unwrap();
            if tank.tank_type == tank_type && tank_name.contains(&search_term) {
                search_result.push(tank)
            }
        } else if tank_name.contains(&search_term) {
            search_result.push(tank)
        }
    }
}
