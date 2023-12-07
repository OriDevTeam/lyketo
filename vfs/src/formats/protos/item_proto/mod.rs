// Relative Modules
pub mod header;
pub mod stride_156;
pub mod stride_171;
pub mod stride_199;

// Standard Uses
use std::io::Cursor;

// Crate Uses
use crate::utils::four_cc;
use crate::utils::key::Key;
use crate::utils::four_cc::FourCC;
use crate::formats::{MCSP_FOURCC, MIPX_FOURCC, MMPT_FOURCC};
use crate::formats::encrypted_object::methods::basic::types::Type6;

// External Uses
use eyre::{Result, bail, WrapErr, eyre};
use once_cell::sync::Lazy;
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;
use bytemuck::Pod;


pub static DEFAULT_KEY: Lazy<Key> = Lazy::new(|| Key::from_segments_u32(
    vec![173217, 72619434, 408587239, 27973291]
));


pub const CHARACTER_NAME_LENGTH: usize = 24;


pub type Stride156 = (stride_156::ItemTable, MIPX_FOURCC);

#[allow(unused)]
pub struct ItemProto<Record> {
    header: header::Header,
    records: Vec<Record>
}

impl<Record> ItemProto<Record> where Record: Pod {
    pub fn from_bytes_with_properties(
        data: Vec<u8>, compression_magic: FourCC
    ) -> Result<Self> {
        let mut cursor = Cursor::new(data);
        let header = header::Header::from_bytes(&mut cursor)?;

        if header.magic != compression_magic {
            bail!(
                "Expected FourCC/Magic '{}', got '{}' instead",
                compression_magic, header.magic
            )
        }

        let expected_stride = std::mem::size_of::<Record>();
        if header.stride as usize != expected_stride {
            bail!(
                "Expected Stride(Table Size) to be {} but got {} instead",
                expected_stride, header.stride
            )
        }

        /*
        match header.stride {
            156 => {},
            171 => {},
            199 => {},
            size => bail!("Can't parse item proto table: stride '{size}' is not supported")
        }
        */

        let content_size = &cursor.remaining();
        if header.expected_content_size as usize != *content_size {
            bail!(
                "Expected {} after header, got {} instead",
                header.expected_content_size, content_size
            )
        }

        let handler = Type6::with_key(DEFAULT_KEY.clone());

        let pos = cursor.position() as usize;
        let inner = cursor.into_inner();

        let table_data = inner[pos..inner.len() - 4].to_vec();
        let deserialized = handler.deserialize(
            table_data, true
        )?;

        let records = bytemuck::try_cast_vec::<_, Record>(deserialized)
            .map_err(|e| eyre!("{:?}", e.0))
            .wrap_err_with(|| "Could not read item proto raw table data:")?;

        Ok(Self { header, records })
    }

    #[allow(unused)]
    pub fn from_file(data: Vec<u8>) -> Result<Self> {
        let content_size = (data.len() - 4 - 4 - 4) as u32;

        let mut cursor = Cursor::new(data);

        let magic: FourCC = cursor.read_u32::<LittleEndian>()?;

        if magic != *MMPT_FOURCC {
            bail!(
                "Expected Magic/FourCC {}({}), but got {}({}) instead",
                *MMPT_FOURCC, four_cc::to_string(*MMPT_FOURCC),
                magic, four_cc::to_string(magic)
            )
        }

        let element_count = cursor.read_u32::<LittleEndian>()?;

        let expected_content_size = cursor.read_u32::<LittleEndian>()?;

        if content_size != expected_content_size {
            bail!("Expected {expected_content_size} after metadata, got {content_size} instead", )
        }

        let handler = Type6::with_key(DEFAULT_KEY.clone());
        let object = cursor.into_inner()[4 + 4 + 4..content_size as usize + 8].to_vec();
        let data = handler.deserialize(object, true).unwrap();


        todo!()
    }
}

