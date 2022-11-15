// Standard Uses

// Crate Uses

// External Uses
use bytemuck;
use itertools::*;


#[derive(Clone, Default)]
pub struct Key {
    segments: Vec<[u8; 4]>
}

impl Key {
    pub fn from_decimals_u32(decimals: Vec<u32>) -> Self {
        let segments: Vec<[u8; 4]> = decimals.iter().map(|s| {
            bytemuck::cast::<u32, [u8; 4]>(*s)
        }).collect();

        Self {
            segments
        }
    }

    /// Flips the byte order between Little and Big Endian
    pub fn flip(&mut self) {
        for segment in &mut self.segments {
            segment.reverse()
        }
    }

    pub fn flipped(&self) -> Vec<[u8; 4]>{
        let mut flipped = vec![];

        for segment in &self.segments {
            let mut seg_flip = segment.clone();
            seg_flip.reverse();
            flipped.push(seg_flip);
        }

        flipped
    }

    pub fn segments_le(&self) -> Vec<[u8; 4]> {
        self.segments.clone()
    }

    pub fn segments_be(&self) -> Vec<[u8; 4]> {
        self.flipped()
    }

    pub fn to_hex_le(&self) -> String {
        let mut repr = String::new();

        for segment in &self.segments_le() {
            for byte in segment {
                repr.push_str(&*format!("{:02X}", byte));
            }
        }

        repr
    }

    pub fn to_hex_be(&self) -> String {
        let mut repr = String::new();

        for segment in &self.segments_be() {
            for byte in segment {
                repr.push_str(&*format!("{:02X}", byte));
            }
        }

        repr
    }

    pub fn to_hex_le_dashed(&self) -> String {
        let dashed = self.to_hex_le()
            .chars()
            .map(Some)
            .interleave_shortest(
                [None, Some('-')].into_iter().cycle()
            )
            .flatten()
            .collect::<String>();

        dashed[..dashed.len() - 1].to_string()
    }

    pub fn to_hex_be_dashed(&self) -> String {
        let dashed = self.to_hex_be()
            .chars()
            .map(Some)
            .interleave_shortest(
                [None, Some('-')].into_iter().cycle()
            )
            .flatten()
            .collect::<String>();

        dashed[..dashed.len() - 1].to_string()
    }

    pub fn to_decimals_le(&self) -> Vec<u32> {
        let segments = self.segments_le().iter().map(|s| {
            bytemuck::cast::<[u8; 4], u32>(*s)
        }).collect();

        segments
    }

    pub fn to_decimals_be(&self) -> Vec<u32> {
        let segments = self.segments_be().iter().map(|s| {
            bytemuck::cast::<[u8; 4], u32>(*s)
        }).collect();

        segments
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut granular = vec![];

        for segment in &self.segments {
            granular.append(&mut segment.to_vec())
        }

        granular
    }
}


#[allow(unused)]
impl TryFrom<[u32; 4]> for Key {
    type Error = anyhow::Error;

    fn try_from(value: [u32; 4]) -> Result<Self, Self::Error> {
        todo!()
    }
}

