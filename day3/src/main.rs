use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// Is there a better way to implement this enum?
enum Chars {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    AUp,
    BUp,
    CUp,
    DUp,
    EUp,
    FUp,
    GUp,
    HUp,
    IUp,
    JUp,
    KUp,
    LUp,
    MUp,
    NUp,
    OUp,
    PUp,
    QUp,
    RUp,
    SUp,
    TUp,
    UUp,
    VUp,
    WUp,
    XUp,
    YUp,
    ZUp,
}

impl FromStr for Chars {
    type Err = ();

    fn from_str(input: &str) -> Result<Chars, Self::Err> {
        match input {
            "a" => Ok(Chars::A),
            "b" => Ok(Chars::B),
            "c" => Ok(Chars::C),
            "d" => Ok(Chars::D),
            "e" => Ok(Chars::E),
            "f" => Ok(Chars::F),
            "g" => Ok(Chars::G),
            "h" => Ok(Chars::H),
            "i" => Ok(Chars::I),
            "j" => Ok(Chars::J),
            "k" => Ok(Chars::K),
            "l" => Ok(Chars::L),
            "m" => Ok(Chars::M),
            "n" => Ok(Chars::N),
            "o" => Ok(Chars::O),
            "p" => Ok(Chars::P),
            "q" => Ok(Chars::Q),
            "r" => Ok(Chars::R),
            "s" => Ok(Chars::S),
            "t" => Ok(Chars::T),
            "u" => Ok(Chars::U),
            "v" => Ok(Chars::V),
            "w" => Ok(Chars::W),
            "x" => Ok(Chars::X),
            "y" => Ok(Chars::Y),
            "z" => Ok(Chars::Z),
            "A" => Ok(Chars::AUp),
            "B" => Ok(Chars::BUp),
            "C" => Ok(Chars::CUp),
            "D" => Ok(Chars::DUp),
            "E" => Ok(Chars::EUp),
            "F" => Ok(Chars::FUp),
            "G" => Ok(Chars::GUp),
            "H" => Ok(Chars::HUp),
            "I" => Ok(Chars::IUp),
            "J" => Ok(Chars::JUp),
            "K" => Ok(Chars::KUp),
            "L" => Ok(Chars::LUp),
            "M" => Ok(Chars::MUp),
            "N" => Ok(Chars::NUp),
            "O" => Ok(Chars::OUp),
            "P" => Ok(Chars::PUp),
            "Q" => Ok(Chars::QUp),
            "R" => Ok(Chars::RUp),
            "S" => Ok(Chars::SUp),
            "T" => Ok(Chars::TUp),
            "U" => Ok(Chars::UUp),
            "V" => Ok(Chars::VUp),
            "W" => Ok(Chars::WUp),
            "X" => Ok(Chars::XUp),
            "Y" => Ok(Chars::YUp),
            "Z" => Ok(Chars::ZUp),
            _ => Err(()),
        }
    }
}

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut total_priority = 0;

    // We'll read three lines at a time into line1, line2, and line3
    let (mut line1, mut line2, mut line3): (String, String, String) =
        ("".to_string(), "".to_string(), "".to_string());

    for (idx, line) in reader.lines().enumerate() {
        let content = line.unwrap();

        // We store the content in each line variable
        if idx % 3 == 0 {
            line1 = content;
        } else if idx % 3 == 1 {
            line2 = content;
        } else if idx % 3 == 2 {
            line3 = content;

            // For line3 specifically, we have special handling
            let mut already_found: Vec<char> = vec![];

            // I'll collect line2 and line3 into a vector so I can use the
            // .contains() function
            let vec2: Vec<char> = line2.chars().collect();
            let vec3: Vec<char> = line3.chars().collect();

            for c in line1.chars() {
                // If this character is in line2, line3, and hasn't already been seen
                if vec2.contains(&c) && vec3.contains(&c) && !already_found.contains(&c) {
                    // Mark it as seen, and increase the total priority
                    already_found.push(c);
                    total_priority += Chars::from_str(&c.to_string()).unwrap() as i32;
                }
            }
        }
    }

    println!("Total priority: {total_priority}");
}

fn part_one() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut total_priority = 0;

    for line in reader.lines() {
        let content = line.unwrap();

        // Split the line in half and store each half
        let split_content = content.split_at(content.len() / 2);
        let (first, second) = (split_content.0, split_content.1);

        // Used to track a character if it's already been seen in both halves
        let mut already_found: Vec<char> = vec![];

        for first_c in first.chars() {
            for second_c in second.chars() {
                if first_c == second_c {
                    // If the character matches, and it hasn't already been seen
                    if !already_found.contains(&first_c) {
                        // Mark it as seen, and increase the total priority
                        already_found.push(first_c);
                        total_priority += Chars::from_str(&first_c.to_string()).unwrap() as i32;
                    }
                }
            }
        }
    }

    println!("Total priority: {total_priority}");
}
