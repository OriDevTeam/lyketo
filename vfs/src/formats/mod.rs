// Relative Modules
pub mod encrypted_object;
pub mod panama;
pub mod protos;

// Crate Uses
use crate::utils::{four_cc, four_cc::FourCC};

// External Uses
use lazy_static::lazy_static;


lazy_static!(
    // TODO: These are here for convenience, we know roughly what they mean but not exactly
    //       where they should be, most likely not here and not always pertaining to versions.
    //       A hunch is that they might relate directly just to Type versions
    pub static ref MMPT_FOURCC: FourCC = four_cc::from_chars(['M', 'M', 'P', 'T']);
    pub static ref MIPX_FOURCC: FourCC = four_cc::from_chars(['M', 'I', 'P', 'X']);
    pub static ref MCOZ_FOURCC: FourCC = four_cc::from_chars(['M', 'C', 'O', 'Z']);
    pub static ref MCSP_FOURCC: FourCC = four_cc::from_chars(['M', 'C', 'S', 'P']);
);
