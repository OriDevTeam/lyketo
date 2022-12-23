// Standard Uses

// Crate Uses

// External Uses
use lyketo_vfs::utils::key::Key;



#[test]
fn create_key() {
    let key = Key::from_decimals_u32(vec![45129401_u32, 92367215, 681285731, 1710201]);

    println!("{}", key.to_hex_le());
    println!("{}", key.to_hex_be());

    println!("{}", key.to_hex_le_dashed());
    println!("{}", key.to_hex_be_dashed());

    println!("Decimals (LE): {:#?}", key.to_decimals_le());
    println!("Decimals (BE): {:#?}", key.to_decimals_be());
}

