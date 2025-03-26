use std::io;

const SHIFT: u8 = 3;

fn main() {
    let mut input = String::new();
    let mut cipher = String::new();
    println!("Please enter your plaintext: ");

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    println!("Select your cipher. c for Caeser, v for Vigenere: ");

    io::stdin()
        .read_line(&mut cipher)
        .expect("error: unable to read user input");

    let cipher = cipher.trim();

    if cipher == "c" {
        println!("Output: {}", caeser(&input));
    } else if cipher == "v" {
        println!("Output: {}", vigenere(&input, "oculorhinolaryngology"));
    } else {
        println!("Invalid selection");
    }
}

fn caeser(input: &str) -> String {
    let mut output = String::from("");
    for chara in input.chars() {
        let mut ascii_val = chara as u8;
        ascii_val += SHIFT;
        output.push(ascii_val as char)
    }
    return output;
}
fn vigenere(input: &str, key: &str) -> String {
    let mut output = String::from("");
    let key = key.to_lowercase();
    let mut values = Vec::new();
    //generate the key shift values
    for k in key.chars() {
        let mut ascii_val = k as u8;
        ascii_val -= 97;
        values.push(ascii_val);
    }
    //set counter
    let mut counter = 0;
    for chara in input.chars() {
        let mut ascii_val: u8 = chara as u8;
        if chara.is_ascii_alphabetic() {
            ascii_val += values[counter];
            //wrap around logic
            if ascii_val > 122 {
                ascii_val -= 26;
            }

            output.push(ascii_val as char);
            //handel the key wrap around
        } else {
            output.push(chara);
        }
        counter += 1;
        if counter >= values.len() {
            counter = 0
        }
    }
    return output;
}
