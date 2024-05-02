use std::fs;
use serde_json;

const ALEPHBET: [(char, &str); 27] = [
('א', "00000"),
('ב', "00001"),
('ג', "00010"),
('ד', "00011"),
('ה', "00100"),
('ו', "00101"),
('ז', "00110"),
('ח', "00111"),
('ט', "01000"),
('י', "01001"),
('כ', "01010"),
('ך', "01010"),
('ל', "01011"),
('מ', "01100"),
('ם', "01100"),
('נ', "01101"),
('ן', "01101"),
('ס', "01110"),
('ע', "01111"),
('פ', "10000"),
('ף', "10000"),
('צ', "10001"),
('ץ', "10001"),
('ק', "10010"),
('ר', "10011"),
('ש', "10100"),
('ת', "10101"),
];

fn find_dimension(length: usize) -> f32{
    let length_f = length as f32;
    length_f.sqrt().ceil()
}

fn main() {
    let data = fs::read_to_string("./src/txt.json").expect("Unable to read file");
    let json: serde_json::Value =
    serde_json::from_str(data.as_str()).expect("JSON was not well-formatted");
    let mut chapter_number = 1;

    for chapter in json["text"].as_array().unwrap(){
        println!("s");
        let mut chapter_string = String::new();
        let mut file_string = String::new();

        for verse in chapter.as_array().unwrap(){
            for c in verse.to_string().chars(){
                for pair in ALEPHBET{
                    if c == pair.0{
                        chapter_string.push_str(pair.1);
                    }
                }
            }
        }
        file_string.push_str("P1\n");
        file_string.push_str(&find_dimension(chapter_string.chars().count()).to_string());
        file_string.push_str(" ");
        file_string.push_str(&find_dimension(chapter_string.chars().count()).to_string());
        file_string.push_str("\n");
        file_string.push_str(&chapter_string);
        print!("{chapter_number}");

        let filename = chapter_number.to_string() + ".pbm";

        fs::write(filename, file_string).expect("Unable to write file");

        chapter_number += 1;
    }
}
