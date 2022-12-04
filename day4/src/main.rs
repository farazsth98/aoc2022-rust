use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut not_overlapped = 0;
    let mut total_assignments = 0;

    for line in reader.lines() {
        let content = line.unwrap();

        // Split up the assignments
        let mut split_content = content.split(",").collect::<Vec<&str>>();

        // Split up the assignments into start and end sections
        let assignment1 = split_content[0].split("-").collect::<Vec<&str>>();
        let assignment2 = split_content[1].split("-").collect::<Vec<&str>>();

        let (start1, end1): (i32, i32) = (
            assignment1[0].parse().unwrap(),
            assignment1[1].parse().unwrap(),
        );

        let (start2, end2): (i32, i32) = (
            assignment2[0].parse().unwrap(),
            assignment2[1].parse().unwrap(),
        );

        // Now, determine whether one assignment does not overlap the other at
        // all
        if start1 > end2 {
            not_overlapped += 1;
        } else if end1 < start2 {
            not_overlapped += 1;
        }

        total_assignments += 1;
    }

    // The amount of overlapped assignments will be the total number of
    // assignments minus the amount of non-overlapped assignments
    let overlapped = total_assignments - not_overlapped;
    println!("Fully contained: {overlapped}");
}

fn part_one() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut fully_contained = 0;

    for line in reader.lines() {
        let content = line.unwrap();

        // Split up the assignments
        let split_content = content.split(",").collect::<Vec<&str>>();

        // Split up the assignments into start and end sections
        let assignment1 = split_content[0].split("-").collect::<Vec<&str>>();
        let assignment2 = split_content[1].split("-").collect::<Vec<&str>>();

        let (start1, end1): (i32, i32) = (
            assignment1[0].parse().unwrap(),
            assignment1[1].parse().unwrap(),
        );

        let (start2, end2): (i32, i32) = (
            assignment2[0].parse().unwrap(),
            assignment2[1].parse().unwrap(),
        );

        // Now, determine if the assignment that starts at the lowest section
        // fully encompasses the other one
        if start1 <= start2 && end1 >= end2 {
            fully_contained += 1;
        } else if start2 <= start1 && end2 >= end1 {
            fully_contained += 1;
        }
    }

    println!("Fully contained: {fully_contained}");
}
