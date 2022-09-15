// Standard Uses

// Crate Uses

// External Uses
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Database {
    welcome_message: String,
    sql_account: String,
    sql_common: String,
    sql_hotbackup: String,
    table_postfix: String,
    bind_port: String,
    db_sleep_msec: String,
    client_heart_fps: String,
    hash_player_life_sec: u32,
    backup_limit_sec: u32,
    player_id_start: u32,
    player_delete_level_limit: u32,
    player_delete_check_simple: u32,
    item_id_range: String,
    min_length_of_social_id: u32,
    // name_column: u32,
    // locale: String,
    simple_socialid: u32,
    // block: String
}
