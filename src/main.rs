use std::fs;
use std::fs::ReadDir;

fn main() {
    let input = fs::read_dir("./input").unwrap();
    convert(input)
}

fn convert(directory: ReadDir) {
    for file in directory {
        let a = file.unwrap();
        let s = a.path().display().to_string();
        if a.file_name().to_str().unwrap().ends_with(".lclass") {
            fs::rename(&s, &s.replace(".lclass", ".class"));
            println!("Done")
        } else if a.file_type().unwrap().is_dir() {
            convert(fs::read_dir(&s).unwrap())
        }
    }
}
