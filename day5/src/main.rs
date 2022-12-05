use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    //part_one()
    part_two();
}

fn part_two() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut parsing_move_commands = false;

    let mut stacks: Vec<Vec<&str>> = vec![];

    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    // Initialize the stacks vector first
    for line in &lines {
        let split_content = line.split(' ').collect::<Vec<&str>>();

        // Find the line with the labels for each stack
        if split_content[0].is_empty() && split_content[1] == "1" {
            // The number of stacks will be the length of this split up line
            // divided by 3
            let num_of_stacks = split_content.len() / 3;

            // Initialize the stacks vector
            for _ in 0..num_of_stacks {
                stacks.push(vec![]);
            }

            break;
        }
    }

    // Now we parse the stacks and move commands
    for line in &lines {
        let split_content = line.split(' ').collect::<Vec<&str>>();

        // If the vector has a length of 1 and only contains an empty string,
        // then its the separating line between the stacks and the move commands
        if split_content.len() == 1 && split_content[0].is_empty() {
            parsing_move_commands = true;
            continue;
        }
        // We also ignore the line with the labels (see above)
        if split_content[0].is_empty() && split_content[1] == "1" {
            continue;
        }

        if !parsing_move_commands {
            // Number of blanks we come across in the split content
            let mut num_of_blanks = 0;

            // Increment number of crates by 1 every time we come across 3
            // blanks, or every time we come across a crate
            let mut num_of_crates = 0;

            // Every 3 blanks, we need to ignore the next blank afterwards. Use
            // this boolean as a flag for that
            let mut ignore_next_blank = false;

            // Now, parse the layer of this stack
            for c in split_content {
                if c.is_empty() {
                    if !ignore_next_blank {
                        num_of_blanks += 1;
                    } else {
                        ignore_next_blank = false;
                    }

                    // Every 3 blanks, we increment num_of_crates, set
                    // num_of_blanks to 0, and ignore the next blank
                    if num_of_blanks == 3 {
                        num_of_crates += 1;
                        num_of_blanks = 0;
                        ignore_next_blank = true;
                    }
                } else {
                    num_of_crates += 1;
                    stacks[num_of_crates - 1].push(&c[1..2]);
                }
            }
        } else {
            let num_of_crates: i32 = FromStr::from_str(split_content[1]).unwrap();
            let stack_from_idx: i32 = FromStr::from_str(split_content[3]).unwrap();
            let stack_to_idx: i32 = FromStr::from_str(split_content[5]).unwrap();

            // For part two, create a new vector to push all the removed crates
            // into
            let mut stack_of_moved_crates: Vec<&str> = vec![];

            for _ in 0..num_of_crates {
                let char = stacks[(stack_from_idx - 1) as usize].remove(0);
                stack_of_moved_crates.push(char);
            }

            // Now, push them into the stack to move to in reverse order (pop)
            for _ in 0..num_of_crates {
                //let char = stacks[(stack_from_idx - 1) as usize].remove(0);
                let char = stack_of_moved_crates.pop().unwrap();
                stacks[(stack_to_idx - 1) as usize].insert(0, char);
            }
        }
    }

    // Finally, print out the first crate in each stack in a row
    for stack in stacks {
        let c = stack[0];
        print!("{c}")
    }
    println!();
}

fn part_one() {
    let filepath = "./test.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut parsing_move_commands = false;

    let mut stacks: Vec<Vec<&str>> = vec![];

    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    // Initialize the stacks vector first
    for line in &lines {
        let split_content = line.split(' ').collect::<Vec<&str>>();

        // Find the line with the labels for each stack
        if split_content[0].is_empty() && split_content[1] == "1" {
            // The number of stacks will be the length of this split up line
            // divided by 3
            let num_of_stacks = split_content.len() / 3;

            // Initialize the stacks vector
            for _ in 0..num_of_stacks {
                stacks.push(vec![]);
            }

            break;
        }
    }

    // Now we parse the stacks and move commands
    for line in &lines {
        let split_content = line.split(' ').collect::<Vec<&str>>();

        // If the vector has a length of 1 and only contains an empty string,
        // then its the separating line between the stacks and the move commands
        if split_content.len() == 1 && split_content[0].is_empty() {
            parsing_move_commands = true;
            continue;
        }
        // We also ignore the line with the labels (see above)
        if split_content[0].is_empty() && split_content[1] == "1" {
            continue;
        }

        if !parsing_move_commands {
            // Number of blanks we come across in the split content
            let mut num_of_blanks = 0;

            // Increment number of crates by 1 every time we come across 3
            // blanks, or every time we come across a crate
            let mut num_of_crates = 0;

            // Every 3 blanks, we need to ignore the next blank afterwards. Use
            // this boolean as a flag for that
            let mut ignore_next_blank = false;

            // Now, parse the layer of this stack
            for c in split_content {
                if c.is_empty() {
                    if !ignore_next_blank {
                        num_of_blanks += 1;
                    } else {
                        ignore_next_blank = false;
                    }

                    // Every 3 blanks, we increment num_of_crates, set
                    // num_of_blanks to 0, and ignore the next blank
                    if num_of_blanks == 3 {
                        num_of_crates += 1;
                        num_of_blanks = 0;
                        ignore_next_blank = true;
                    }
                } else {
                    num_of_crates += 1;
                    stacks[num_of_crates - 1].push(&c[1..2]);
                }
            }
        } else {
            let num_of_crates: i32 = FromStr::from_str(split_content[1]).unwrap();
            let stack_from_idx: i32 = FromStr::from_str(split_content[3]).unwrap();
            let stack_to_idx: i32 = FromStr::from_str(split_content[5]).unwrap();

            // Remove num_of_crates amount of crates from the beginning of
            // stack_from, and insert them into the beginning of stack_to
            for _ in 0..num_of_crates {
                let char = stacks[(stack_from_idx - 1) as usize].remove(0);
                stacks[(stack_to_idx - 1) as usize].insert(0, char);
            }
        }
    }

    // Finally, print out the first crate in each stack in a row
    for stack in stacks {
        let c = stack[0];
        print!("{c}")
    }
    println!();
}
