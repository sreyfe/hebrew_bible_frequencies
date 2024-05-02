use std::fs;
use serde_json;

const ALEPHBET: [(char, &str); 27] = [
('א', "253 253 253 "),
('ב', "29 29 29 "),
('ג', "235 206 43 "),
('ד', "112 44 140 "),
('ה', "219 105 23 "),
('ו', "150 205 230 "),
('ז', "186 28 48 "),
('ח', "192 189 127 "),
('ט', "127 126 128 "),
('י', "95 166 65 "),
('כ', "212 133 178 "),
('ך', "212 133 178 "),
('ל', "66 119 182 "),
('מ', "223 132 97 "),
('ם', "223 132 97 "),
('נ', "70 51 151 "),
('ן', "70 51 151 "),
('ס', "225 161 26 "),
('ע', "145 33 140 "),
('פ', "232 233 72 "),
('ף', "232 233 72 "),
('צ', "126 21 16 "),
('ץ', "126 21 16 "),
('ק', "146 174 49 "),
('ר', "111 52 13 "),
('ש', "211 43 30 "),
('ת', "43 53 20 "),
];

fn find_dimension(length: usize) -> f32{
    let length_f = length as f32;
    length_f.sqrt().ceil()
}

fn trailing_black(dim: f32, length: usize) -> String{
    let dim_f = dim as usize;
    let leftover = dim_f*dim_f-length;

    "0 0 0 ".repeat(leftover)
}

fn main() {
    let data = fs::read_to_string("./src/txt.json").expect("Unable to read file");
    let json: serde_json::Value =
    serde_json::from_str(data.as_str()).expect("JSON was not well-formatted");
    let mut chapter_number = 1;
    

    for chapter in json["text"].as_array().unwrap(){
        let mut chapter_string = String::new();
        let mut file_string = String::new();
        let mut chapter_length: usize = 0;
        let mut verse_number = 1;

        for verse in chapter.as_array().unwrap(){
            let mut verse_string = String::new();
            let mut verse_length: usize = 0;
            let mut verse_file_string = String::new();

            for c in verse.to_string().chars(){
                for pair in ALEPHBET{
                    if c == pair.0{
                        chapter_length = chapter_length + 1;
                        verse_length = verse_length + 1;

                        verse_string.push_str(pair.1);
                        chapter_string.push_str(pair.1);
                    }
                }
            }
            verse_file_string.push_str("P3\n");
            verse_file_string.push_str(&find_dimension(verse_length).to_string());
            verse_file_string.push_str(" ");
            verse_file_string.push_str(&find_dimension(verse_length).to_string());
            verse_file_string.push_str(" 255");
            verse_file_string.push_str("\n");
            verse_file_string.push_str(&verse_string);
            //adds trailing black in order to round out the ppm
            verse_file_string.push_str(trailing_black(find_dimension(verse_length), verse_length).as_str());

            let filename = "verses/".to_string() + &chapter_number.to_string() + "-" + &verse_number.to_string() + ".ppm";
            fs::write(filename, verse_file_string).expect("Unable to write file");

            verse_number += 1;
        }

        file_string.push_str("P3\n");
        file_string.push_str(&find_dimension(chapter_length).to_string());
        file_string.push_str(" ");
        file_string.push_str(&find_dimension(chapter_length).to_string());
        file_string.push_str(" 255");
        file_string.push_str("\n");
        file_string.push_str(&chapter_string);
        //adds trailing black in order to round out the ppm
        file_string.push_str(trailing_black(find_dimension(chapter_length), chapter_length).as_str());

        let filename = "chapters/".to_string() + &chapter_number.to_string() + ".ppm";

        fs::write(filename, file_string).expect("Unable to write file");

        chapter_number += 1;
    }
}
