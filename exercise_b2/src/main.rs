use std::io;
use std::io::Read;
use std::fs;
fn main() {
    let mut file = fs::File::open("files/1-s2.0-S0960982203005347-mmc6.txt").unwrap();
    let mut contents = String::new();
    // read file and convert to vec
    file.read_to_string(&mut contents).unwrap();
    let contents_lower = contents.to_lowercase();
    let contents_vec: Vec<char> = contents_lower.chars().collect();

    loop {
        let mut word_input = String::new();
        println!("If you want exit, press only enter");
        println!("Enter a word:");
        io::stdin().read_line(&mut word_input).unwrap();
        // delete crlf
        word_input = word_input.trim().to_string();
        if word_input.len() == 0 {
            break;
        }
        // c1 using split
        let result_vec:Vec<&str>= contents_lower.split(word_input.to_lowercase().as_str()).collect();
        println!("c1 số lần xuất hiện trong file cua \"{}\" (khong ke hoa thuong) la: {}", word_input.as_str(), result_vec.len() -1);
        // c2 chuyen ve ver sau cout
        let word_input_vec: Vec<char> = word_input.to_lowercase().chars().collect();
        // count
        let mut count_sub_string = 0;
        for i in 0..contents_vec.len() {
            if word_input_vec[0] == contents_vec[i] && (i + word_input_vec.len()) < contents_vec.len(){
                let mut count = 0;
                for j in 0..word_input_vec.len() {
                    if word_input_vec[j] == contents_vec[i + j] {
                        count += 1;
                    }
                }
                if count == word_input_vec.len() {
                    count_sub_string += 1;
                }
            }
        }
        println!("số lần xuất hiện trong file cua \"{}\" (khong ke hoa thuong) la: {}", word_input.as_str(), count_sub_string);
    }
    println!("end program");
}
