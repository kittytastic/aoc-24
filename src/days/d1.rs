use crate::utils::utils::get_input_file;
use std::collections::{BinaryHeap, HashMap};

pub fn day1_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d1.txt");

    let split_input: Vec<Vec<&str>> = input_file_str.lines().map(|line: &str|{
        line.split_ascii_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut col1_heap:BinaryHeap<i32> = BinaryHeap::new();
    let mut col2_heap:BinaryHeap<i32> = BinaryHeap::new();

    for line in split_input.iter(){
        col1_heap.push(line[0].parse().expect("Should be num"));
        col2_heap.push(line[1].parse().expect("Should be num"));
    }

    assert_eq!(col1_heap.len(), col2_heap.len());

    let mut distance = 0;
    while col1_heap.len() > 0 {
        let c1_val = col1_heap.pop().expect("Should have val");
        let c2_val = col2_heap.pop().expect("Should have val");
        distance += (c1_val - c2_val).abs()
    }
    println!("Part 1 answer: {}", distance);

    // Part 2
    let mut col2_lookup:HashMap<i32, i32> = HashMap::new();
    let mut col1_values:Vec<i32> = Vec::new();
    for line in split_input{
        col1_values.push(line[0].parse().expect("Should be num"));
        let c2_val:i32 = line[1].parse().expect("Should be num");
        col2_lookup.entry(c2_val).and_modify(|v|*v+=1).or_insert(1);
    }

    let sim_score:i32 = col1_values.iter().map(|v| v*col2_lookup.get(v).cloned().unwrap_or_default()).sum();

    println!("Part 2 answer: {}", sim_score);
}