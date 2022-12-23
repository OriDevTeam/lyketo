// Standard Uses

// Crate Uses

// External Uses
use lyketo_vfs::vfs::eterfs::Eter;
use lyketo_vfs::vfs::handler::mock::Mock;


const PATH: &str = "tests/data/eterfs/test.mock";


#[test]
pub fn create_handler () {
    let vfs = Box::new(Eter::new());
        // Lets create a mock handler for testing purposes, its simple and straightforward
    // to mess with
    let mut handler = Mock::new(vfs);

    // And set the private key, this is just a fictitious security key to mimic the setup
    // of what a cryptographic handler would do at higher level
    handler.set_key("Test".to_string()).unwrap();

    // Add the mock unit of where the virtual file we want is in
    handler.add_mock(PATH).unwrap();

    // Attempt to get the virtual file since the key is set and the unit is added
    // so everything should be fine
    let mem_file = handler.get_file("test/virtual/space/hello.txt").unwrap();

    assert_eq!(mem_file, b"Hello World");
}

