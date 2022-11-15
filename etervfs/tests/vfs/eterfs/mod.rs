// Relative Modules
mod index;

// Standard Uses

// Crate Uses

// External Uses



// const PACK_PATH: &str = "tests/data/eterfs/";


/*
#[test]
pub fn create_vfs() {
    println!("{:?}", env::current_dir().unwrap());

    // Firstly lets create the file type handlers, these are responsible for knowing how to
    // do decompression, decryption, sectioning, integral checking, unwrapping and etc...
    let mut mock_handler = Box::new(Mock::new());
    mock_handler.set_key("Test".to_string()).unwrap();

    let mut mock_handler_dyn: Rc<Box<dyn Handler>> = Rc::new(mock_handler);

    // Now we create the actual Virtual File System, this is what manages how to deal with
    // files in the virtual domain like path mapping, linking files in the memory and work
    // with the handlers
    // NOTE: You can instantiate the type handlers after creating the VFS, as long as when
    // you are going to add/request pack files the handlers are added before-hand
    let mut vfs = Eter::new();
    vfs.add_handler(mock_handler_dyn);

    // Load available packs from a directory, this does not decompress/decrypt them yet,
    // it only indexes the files that are inside them
    vfs.load_disk_dir(PACK_PATH).expect("Could load from disk");

    let file_path = "test/hello.txt";

    // Since we now have packs in the virtual space, lets check if a virtual file exists
    // and we can get it by name or CRC32
    if !vfs.virtual_map.has_file_by_name(file_path) { panic!("Could not find file in the virtual space") };

    // Since it exists, lets try getting it, the function deals with decompressing,
    // or decrypting, sectioning and unwrapping
    let mem_file = vfs.virtual_map.get_file_by_name(file_path).unwrap();

    // Lets check if the file we got is what we expected
    assert_eq!(mem_file, b"Hello World!")
}
*/


/*
#[test]
pub fn create_vfs_content_manually() {
    // TODO: Add cryptographic and compression capabilities to this instance
    // let encrypted_object_handler = EncryptedObjectHandler();
    // vfs.add_type_handler(encrypted_object_handler);

    // Lets prepare the virtual file system normally
    let mut mock_handler: Rc<Box<Mock>> = Rc::new(Box::new(Mock::new()));
    mock_handler.set_key("Test".to_string()).unwrap();

    let mut mock_handler_dyn: Rc<Box<dyn Handler>> = Rc::clone(&mock_handler);

    let mut vfs = Eter::new();
    vfs.add_handler(mock_handler_dyn);

    // Now instead of detecting the index we can register an Index of our choice manually
    let index = Index::load_from_disk(&format!("{}/Index", PACK_PATH)).unwrap();
    vfs.drive_map.register_index(&index).unwrap();

    // Additionally we can add a single pack manually
    // we call let it try to detect what types it is based on its file name, extensions
    // and intrinsic file data
    let mock_pack = format!("{}/test", PACK_PATH);

    // By specifying the unit path to be searched by patterns
    vfs.load_unit_pattern(&*mock_pack).unwrap();

    // Or by specifying which are the unit files directly
    let mock_files_paths = vec![format!("{}.mock", mock_pack)];
    vfs.load_unit_files(mock_files_paths).expect("Cannot load pack");

    // Or we can add it trough the handler directly if we need something like being sure
    // we are only accepting a file with a specific type
    mock_handler.add_mock(&mock_pack).unwrap();
}
*/
