use std::collections::{BTreeSet, HashMap};

use crate::utils::utils::get_input_file;

pub fn get_central_element(slice: &[u32])->u32{
    slice[slice.len()/2]
}

pub fn day5_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d5.txt");
    let input_parts:Vec<_> = input_file_str.split("\r\n\r\n").collect(); // Windows :(

    let raw_constraints: Vec<Vec<u32>> = input_parts[0].lines().map(|l|l.split('|').map(|v|v.parse().expect("Should be num")).collect()).collect();
    let sequences:Vec<Vec<u32>> = input_parts[1].lines().map(|l|l.split(',').map(|v|v.parse().expect("Should be num")).collect()).collect();
    let mut constraints:HashMap<u32, Vec<u32>> = HashMap::new();

    for c in raw_constraints{
        constraints.entry(c[0]).and_modify(|v|(*v).push(c[1])).or_insert(vec![c[1]]);
    } 

    // Part 1
    let mut total:u32 = 0;
    let mut bad_sequences = Vec::new();

    for s in sequences.iter(){
        let mut seen_vals:BTreeSet<u32> = BTreeSet::new();
        let mut valid = true;
        for v in s.iter(){
            seen_vals.insert(*v);
            let con = if let Some(c) = constraints.get(v){c}else{continue;}; // Unconstrained Val
            for c in con.iter(){
                if seen_vals.contains(c){valid = false}
            }
        }
        if valid {
            total+=get_central_element(&s)
        }else{
            bad_sequences.push(s.clone());
        }
    }
    println!("Answer to part 1: {}", total);

    // Part 2
    // Basically just bubble sort
    let mut total_2:u32 = 0;
    for mut seq in bad_sequences.into_iter(){
        let mut i = 0;
        'outer: while i<seq.len(){
            let val_constraints = if let Some(c) = constraints.get(&seq[i]){c}else{i+= 1; continue;}; // Unconstrained Val
            for c in val_constraints{
                for j in 0..i{
                    if *c == seq[j]{
                        (seq[i], seq[j]) = (seq[j], seq[i]); // Swap
                        i = j; // Go back
                        continue 'outer;
                    }
                }
            }
            i+=1;
        }
        total_2 += get_central_element(&seq);
    }

    println!("Answer to part 2: {}", total_2);
}