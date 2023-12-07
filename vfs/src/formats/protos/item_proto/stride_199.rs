// Standard Uses
use std::ffi::c_char;
use bytemuck::{Pod, Zeroable};

// Crate Uses

// External Uses


const ITEM_NAME_LENGTH: usize = 24;
const ITEM_LIMITS_MAXIMUM: usize = 2;
const ITEM_APPLIES_MAXIMUM: usize = 4;
const ITEM_VALUES_MAXIMUM: usize = 6;
const ITEM_SOCKETS_MAXIMUM: usize = 6;


#[derive(Copy, Clone, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct Record {
    vnum: u32,
    vnum_range: u32,
    name: [c_char; ITEM_NAME_LENGTH + 1],
    locale_name: [c_char; ITEM_NAME_LENGTH + 1],
    r#type: u8,
    sub_type: u8,
    weight: u8,
    size: u8,
    anti_flags: u32,
    flags: u32,
    wear_flags: u32,
    immune_flags: u32,
    buy_item_price: u32,
    sell_item_price: u32,
    limits: [ItemLimit; ITEM_LIMITS_MAXIMUM],
    applies: [ItemApply; ITEM_APPLIES_MAXIMUM],
    values: [i32; ITEM_VALUES_MAXIMUM],
    sockets: [i32; ITEM_SOCKETS_MAXIMUM],
    refine_element_apply_type: u32,
    refine_element_grade: u32,
    refine_element_value: u32,
    refine_element_bonus: u32,
    refined_vnum: u32,
    refine_set: u16,
    material: u32,
    alter_to_magic_item_percentage: u8,
    specular: u8,
    gain_socket_percentage: u8,
    mask_type: u8,
    mask_sub_type: u8
}

#[derive(Copy, Clone, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct ItemLimit {
    r#type: u8,
    value: i32
}

#[derive(Copy, Clone, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C, packed(1))]
pub struct ItemApply {
    r#type: u16,
    value: i32
}
