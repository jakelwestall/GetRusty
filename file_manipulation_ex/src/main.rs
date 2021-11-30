use std::path::Path;
use std::fs::File;

fn main() {
    make_file("test_file.txt");
}

fn make_file(fp: &str) {
    if Path::new(fp).exists() {
        println!("The file does exist.");
    } else {
        println!("The file does not exist, creating");
        File::create(fp);
    }
}