use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());
    let mut acc = 0;
    let (mut max, mut max2, mut max3) = (0, 0, 0);

    for line in reader.lines() {
        // The text file only has numbers
        let line_content = line.unwrap();

        // If its not a number, calculate the max and continue
        if line_content == "" {
            if acc > max {
                max3 = max2;
                max2 = max;
                max = acc;
            } else if acc > max2 {
                max3 = max2;
                max2 = acc;
            } else if acc > max3 {
                max3 = acc;
            }
            acc = 0;
            continue;
        }

        // Else, increment the acc
        acc += line_content.parse::<i32>().unwrap();
    }

    let total_max = max + max2 + max3;

    println!("Max amount of calories found: {max} + {max2} + {max3} = {total_max}");
}
