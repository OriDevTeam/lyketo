// Standard Uses
use std::ffi::c_char;

// Crate Uses
use crate::utils::four_cc;
use crate::utils::four_cc::FourCC;
use crate::formats::protos::mob_proto::CHARACTER_NAME_LENGTH;

// External Uses
use eyre::Result;
use lazy_static::lazy_static;
use bytemuck::AnyBitPattern;


lazy_static!(
    pub static ref MAGIC_VERSION_2_FCC: FourCC = four_cc::from_chars(['M', 'M', 'P', 'T']);
);


const MOB_ENCHANTS_MAX: usize = 8;

#[allow(unused)]
const MOB_SKILL_MAX: usize = 5;


pub struct Table {
    pub records: Vec<Record>
}

#[derive(Copy, Clone, Debug)]
#[derive(AnyBitPattern)]
#[repr(C)]
pub struct Record {
    pub vnum: u32,
    pub name: [c_char; CHARACTER_NAME_LENGTH + 1],
    pub locale_name: [c_char; CHARACTER_NAME_LENGTH + 1],

    pub r#type: u8,
    pub rank: u8,
    pub battle_type: u8,
    pub level: u8,
    pub size: u8,

    pub min_gold: u32,
    pub max_gold: u32,
    pub exp: u32,
    pub max_health: u32,
    pub regen_cycle: u8,
    pub regen_percent: u8,
    pub def: u16,

    pub ai_flag: u32,
    pub race_flag: u32,
    pub immune_flag: u32,

    pub str: u8, pub dex: u8,
    pub con: u8, pub int: u8,

    pub damage_range: [u32; 2],
    pub attack_speed: i16,
    pub movement_speed: i16,
    pub aggressive_health_percentage: u8,
    pub aggressive_sight: u16,
    pub attack_range: u16,

    pub enchants: [u8; MOB_ENCHANTS_MAX],
    pub resists: [u8; MOB_ENCHANTS_MAX],

    pub resurrection_vnum: u32,
    pub drop_item_vnum: u32,

    pub mount_capacity: u8,
    pub on_click_type: u8,

    pub empire: u8,
    // pub folder: [c_char; 64 + 1],

    pub damage_multiplier: f32,
    pub summon_vnum: u32,
    pub drain_sp: u32,
    pub color: u32,
    pub polyform_item_vnum: u32,

    // pub skills: [SkillLevel; MOB_SKILL_MAX],

    pub berserk_point: u32,
    pub stone_skin_point: u32,
    pub god_speed_point: u32,
    pub death_blow_point: u32,
    pub revive_point: u32
}

#[allow(unused)]
impl Table {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        todo!()

        /*
        let table = bytemuck::from_bytes(&data.as_slice());

        Ok(table)
         */
    }
}


#[derive(Copy, Clone, Debug)]
#[derive(AnyBitPattern)]
#[repr(C)]
pub struct SkillLevel {
    pub vnum: u32,
    pub level: u8
}

