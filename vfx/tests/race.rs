// Standard USes
use std::{env, fs};
use std::path::Path;

// Crate Uses

// External Uses
use lyketo::formats::m2_script;


// TODO: Delete this file
const TEST_M2SCRIPT_PATH: &str = "lyketo/tests/data/test.msm";


#[test]
fn load_race_data_script() {
    let path = Path::new(TEST_M2SCRIPT_PATH);
    let contents = fs::read_to_string(path).unwrap();

    race::from_file(Path::new(TEST_M2SCRIPT_PATH)).unwrap();
}
