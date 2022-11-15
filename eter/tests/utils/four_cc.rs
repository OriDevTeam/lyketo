// Standard Uses

// Crate Uses

// External Uses
use lyketo_eter::utils::four_cc;


#[test]
fn make_four_ccs() {
    println!("{} | U32: {} | Bytes: {:#?}",
             four_cc::to_string(&four_cc::from_chars(['M', 'C', 'O', 'Z'])),
             four_cc::from_chars(['M', 'C', 'O', 'Z']),
             four_cc::to_bytes(four_cc::from_chars(['M', 'C', 'O', 'Z']))
    );

    println!("{} | U32: {} | Bytes: {:#?}",
             four_cc::to_string(&four_cc::from_chars(['M', 'C', 'P', 'Z'])),
             four_cc::from_chars(['M', 'C', 'P', 'Z']),
             four_cc::to_bytes(four_cc::from_chars(['M', 'C', 'P', 'Z']))
    );
}

