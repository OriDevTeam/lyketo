// Standard Uses

// Crate Uses

// External Uses
use lyketo_vfs::utils::four_cc;


#[test]
fn make_four_ccs() {
    let mcoz = four_cc::from_chars(['M', 'C', 'O', 'Z']);
    println!("{} | U32: {} | Bytes: {:#?}",
             four_cc::to_string(&mcoz),
             &mcoz,
             four_cc::to_bytes(mcoz)
    );

    let mcpz = four_cc::from_chars(['M', 'C', 'P', 'Z']);
    println!("{} | U32: {} | Bytes: {:#?}",
             four_cc::to_string(&mcpz),
             &mcpz,
             four_cc::to_bytes(mcpz)
    );
}

