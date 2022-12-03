use std::fs::File;
use std::io::{BufRead, BufReader};

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut score = 0;

    for line in reader.lines() {
        let content = line.unwrap();
        let split_content = content.split(" ").collect::<Vec<&str>>();
        let (enemy, me) = (split_content[0], split_content[1]);

        if enemy == "A" {
            if me == "X" {
                score += LOSS + SCISSORS;
            } else if me == "Y" {
                score += DRAW + ROCK;
            } else {
                score += WIN + PAPER;
            }
        } else if enemy == "B" {
            if me == "X" {
                score += LOSS + ROCK;
            } else if me == "Y" {
                score += DRAW + PAPER;
            } else {
                score += WIN + SCISSORS;
            }
        } else {
            if me == "X" {
                score += LOSS + PAPER;
            } else if me == "Y" {
                score += DRAW + SCISSORS;
            } else {
                score += WIN + ROCK;
            }
        }
    }

    println!("Final score: {score}");
}

fn part_one() {
    let filepath = "./challenge_input.txt";

    let reader = BufReader::new(File::open(filepath).unwrap());

    let mut score = 0;

    for line in reader.lines() {
        let content = line.unwrap();
        let split_content = content.split(" ").collect::<Vec<&str>>();
        let (enemy, me) = (split_content[0], split_content[1]);

        if enemy == "A" {
            if me == "X" {
                score += DRAW + ROCK;
            } else if me == "Y" {
                score += WIN + PAPER;
            } else {
                score += LOSS + SCISSORS;
            }
        } else if enemy == "B" {
            if me == "X" {
                score += LOSS + ROCK;
            } else if me == "Y" {
                score += DRAW + PAPER;
            } else {
                score += WIN + SCISSORS;
            }
        } else {
            if me == "X" {
                score += WIN + ROCK;
            } else if me == "Y" {
                score += LOSS + PAPER;
            } else {
                score += DRAW + SCISSORS;
            }
        }
    }

    println!("Final score: {score}");
}
