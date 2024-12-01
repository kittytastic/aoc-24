use std::path::Path;
use std::fs;

// Read an input file from the input data directory
pub fn get_input_file(file_name: &str) -> String{
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input_data").join(file_name);
    if !path.is_file(){
        println!("ERROR: file doesn't exist {:?}", path);
        std::process::exit(1);
    }

    return fs::read_to_string(path).expect("Should be able to read file");
}