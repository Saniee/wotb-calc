use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::EnumIter;

#[derive(Serialize, Deserialize)]
pub struct TankData {
    pub status: String,
    pub meta: Meta,
    #[serde(rename = "data")]
    pub tanks: HashMap<String, Tank>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tank {
    pub suspensions: Vec<i64>,
    pub description: Option<String>,
    pub engines: Vec<i64>,
    pub prices_xp: Option<HashMap<String, i64>>,
    pub next_tanks: Option<HashMap<String, i64>>,
    pub modules_tree: HashMap<String, ModulesTree>,
    pub nation: Nation,
    pub is_premium: bool,
    pub images: Images,
    pub cost: Option<Cost>,
    pub default_profile: DefaultProfile,
    pub tier: i64,
    pub tank_id: i64,
    #[serde(rename = "type")]
    pub tank_type: TankType,
    pub guns: Vec<i64>,
    pub turrets: Vec<i64>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cost {
    pub price_credit: i64,
    pub price_gold: i64,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TankType {
    #[serde(rename = "AT-SPG")]
    AtSpg,
    #[serde(rename = "heavyTank")]
    HeavyTank,
    #[serde(rename = "lightTank")]
    LightTank,
    #[serde(rename = "mediumTank")]
    MediumTank,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultProfile {
    pub weight: i64,
    pub profile_id: String,
    pub firepower: i64,
    pub shot_efficiency: i64,
    pub gun_id: i64,
    pub signal_range: Option<serde_json::Value>,
    pub shells: Vec<Shell>,
    pub armor: Armor,
    pub speed_forward: i64,
    pub battle_level_range_min: i64,
    pub speed_backward: i64,
    pub engine: Engine,
    pub max_ammo: i64,
    pub battle_level_range_max: i64,
    pub engine_id: i64,
    pub hp: i64,
    pub is_default: bool,
    pub protection: i64,
    pub suspension: Suspension,
    pub suspension_id: i64,
    pub max_weight: i64,
    pub gun: Gun,
    pub turret_id: Option<i64>,
    pub turret: Turret,
    pub maneuverability: i64,
    pub hull_weight: i64,
    pub hull_hp: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Armor {
    pub turret: Hull,
    pub hull: Hull,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hull {
    pub front: i64,
    pub sides: i64,
    pub rear: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Engine {
    pub tier: i64,
    pub fire_chance: f64,
    pub power: i64,
    pub name: String,
    pub weight: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gun {
    pub move_down_arc: i64,
    pub caliber: i64,
    pub name: String,
    pub weight: i64,
    pub move_up_arc: i64,
    pub fire_rate: f64,
    pub clip_reload_time: f64,
    pub dispersion: f64,
    pub clip_capacity: i64,
    pub traverse_speed: f64,
    pub reload_time: f64,
    pub tier: i64,
    pub aim_time: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shell {
    #[serde(rename = "type")]
    pub shell_type: ShellType,
    pub penetration: i64,
    pub damage: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShellType {
    #[serde(rename = "ARMOR_PIERCING")]
    ArmorPiercing,
    #[serde(rename = "ARMOR_PIERCING_CR")]
    ArmorPiercingCr,
    #[serde(rename = "HIGH_EXPLOSIVE")]
    HighExplosive,
    #[serde(rename = "HOLLOW_CHARGE")]
    HollowCharge,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Suspension {
    pub tier: i64,
    pub load_limit: i64,
    pub traverse_speed: i64,
    pub name: String,
    pub weight: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Turret {
    pub name: String,
    pub weight: i64,
    pub view_range: i64,
    pub traverse_left_arc: i64,
    pub hp: i64,
    pub traverse_speed: i64,
    pub tier: i64,
    pub traverse_right_arc: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Images {
    pub preview: Option<String>,
    pub normal: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModulesTree {
    pub name: String,
    pub next_modules: Option<Vec<i64>>,
    pub next_tanks: Option<Vec<i64>>,
    pub is_default: bool,
    pub price_xp: i64,
    pub price_credit: i64,
    pub module_id: i64,
    #[serde(rename = "type")]
    pub modules_tree_type: ModulesTreeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)]
pub enum ModulesTreeType {
    #[serde(rename = "vehicleChassis")]
    VehicleChassis,
    #[serde(rename = "vehicleEngine")]
    VehicleEngine,
    #[serde(rename = "vehicleGun")]
    VehicleGun,
    #[serde(rename = "vehicleTurret")]
    VehicleTurret,
}

#[derive(Serialize, Deserialize, Debug, EnumIter, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Nation {
    China,
    European,
    France,
    Germany,
    Japan,
    Other,
    Uk,
    Usa,
    Ussr,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub count: i64,
}
