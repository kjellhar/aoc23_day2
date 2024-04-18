//use std::collections::HashMap;
use std::fs::read_to_string;
use regex::Regex;


const FILE_NAME: &str = "input.txt";
const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

fn main() {
    
    let find_game = Regex::new(r"Game ([0-9]{1,3})").unwrap();
    let find_red = regex::Regex::new(r"([0-9]{1,2}) red").unwrap();
    let find_blue = regex::Regex::new(r"([0-9]{1,2}) blue").unwrap();
    let find_green = regex::Regex::new(r"([0-9]{1,2}) green").unwrap();

    let mut res: u32 = 0;
    let mut sum_of_power: u32 = 0;

    for line in read_to_string(FILE_NAME).unwrap().lines() {
        
        let game_number: u32 = if let Some(nu) = 
            find_game.captures(line) {nu[1].parse::<u32>().unwrap()} else {break};
        
        let mut nred = 0;
        for red in find_red.captures_iter(line) {
            if red[1].parse::<u32>().unwrap()>nred {nred = red[1].parse::<u32>().unwrap();}          
        }
        
        let mut nblue = 0;
        for blue in find_blue.captures_iter(line) {
            if blue[1].parse::<u32>().unwrap()> nblue {nblue = blue[1].parse::<u32>().unwrap();}          
        }

        let mut ngreen = 0;
        for green in find_green.captures_iter(line) {
            if green[1].parse::<u32>().unwrap() > ngreen {ngreen = green[1].parse::<u32>().unwrap();}          
        }

        sum_of_power += nred*ngreen*nblue;

        if nred <= RED_LIMIT && ngreen <= GREEN_LIMIT && nblue <= BLUE_LIMIT {
            res += game_number;
        } else {
            res = res;
        }
    }

    println!("Sum:  {}", res);
    println!("Sum of power:  {}", sum_of_power);
}
