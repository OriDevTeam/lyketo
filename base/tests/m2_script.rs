// Standard USes
use std::{env, fs};
use std::path::Path;

// Crate Uses

// External Uses
use lyketo::formats::m2_script;


const TEST_M2SCRIPT_PATH: &str = "base/tests/data/m2_script/test.ms";


#[test]
fn load_parse_script() {
    let path = Path::new(TEST_M2SCRIPT_PATH);
    let contents = fs::read_to_string(path).unwrap();

    m2_script::parse_file(contents.as_str()).unnwrap();

    assert_eq!()
}
