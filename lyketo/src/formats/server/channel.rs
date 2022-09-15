// Standard Uses

// Crate Uses

// External Uses
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Channel {
    hostname: String,
    channel: u8,
    auth_server: u32,
    port: u32,
    p2p_port: u32,
    db_port: u32,
    dn_addr: String,
    table_postfix: String,
    item_id_range: String,
    passes_per_sec: u32,
    safe_event_second_cycle: u32,
    ping_event_second_cycle: u32,
    player_sql: String,
    common_sql: String,
    log_sql: String,
    locale_service: String,
    max_level: u32,
    bind_ip: String,
    busy: u32,
    full: u32
}
