use std::collections::HashMap;

use crate::utils::utils::get_input_file;

pub fn day19_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d19.txt");
    let parts:Vec<&str> = input_string.split("\r\n\r\n").collect();
    let base_pattens: Vec<&str> = parts[0].split(',').map(|s|{s.trim()}).collect();
    let target_towels: Vec<&str> = parts[1].lines().collect();


    let mut i = 0;
    for t in target_towels.iter(){
        let mut visited: HashMap<&str, bool> = HashMap::new();
        if part1_can_be_made(t, &base_pattens, & mut visited){
            i += 1;
        }
    }
    println!("Answer to part 1: {}", i);

    let mut j = 0;
    for t in target_towels.iter(){
        let mut visited: HashMap<&str, usize> = HashMap::new();
        j += part2_can_be_made(t, &base_pattens, & mut visited)
    }
    println!("Answer to part 2: {}", j);
    
}

fn part1_can_be_made<'a>(pattern: &'a str, base_pattens: &Vec<&str>, visited: & mut HashMap<&'a str, bool>)->bool{
    if pattern.len() == 0 {return true;}
    if let Some(v) = visited.get(pattern) {return *v;};
    
    let mut possible = false;
    for p in base_pattens {
        if pattern.starts_with(p){
            let sub_pattern = &pattern[p.len()..];
            if part1_can_be_made(sub_pattern, base_pattens, visited){
                possible = true;
                break;
            }
        }
    }

    visited.insert(pattern, possible);
    possible
}


fn part2_can_be_made<'a>(pattern: &'a str, base_pattens: &Vec<&str>, visited: & mut HashMap<&'a str, usize>)->usize{
    if pattern.len() == 0 {return 1;}
    if let Some(v) = visited.get(pattern) {return *v;};
    
    let mut combos = 0;
    for p in base_pattens {
        if pattern.starts_with(p){
            let sub_pattern = &pattern[p.len()..];
            combos += part2_can_be_made(sub_pattern, base_pattens, visited);
        }
    }

    visited.insert(pattern, combos);
    combos
}