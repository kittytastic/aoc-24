use regex::Regex;
use crate::utils::utils::get_input_file;

pub fn day3_main(second_part: bool, _extra_args: &Vec<String>){
    if second_part{
        part2();
    }else{
        part1();
    }
}


fn part1(){
    let input_file_str = get_input_file("d3.txt");
    let sum = sum_mul_in_slice(&input_file_str);
    println!("Answer to part 1: {}", sum);
}


fn sum_mul_in_slice(slice: &str) -> i32{
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(&slice).map(|c| c.extract()) {
        results.push((a.parse::<i32>().expect("Should be a num"), b.parse::<i32>().expect("Should be a num")));
    }

    let sum:i32 = results.iter().map(|(a,b)|a*b).sum();
    sum
}


fn part2(){
    let input_file_str = get_input_file("d3.txt");

    let mut enabled_slices: Vec<&str> = Vec::new();
    let mut current_slice_start:usize = 0;
    let mut enabled = true;
    const DO_TXT: &str = "do()";
    const DONT_TXT: &str = "don't()";
    
    for i in 0..input_file_str.len(){
        let curr_slice = &input_file_str[i..];
        if curr_slice.starts_with(DO_TXT){
            if enabled {continue;}
            enabled = true;
            current_slice_start = i;
        }

        if curr_slice.starts_with(DONT_TXT){
            if enabled {enabled_slices.push(&input_file_str[current_slice_start..i])};
            enabled = false;
        }
    }    

    if enabled {enabled_slices.push(&input_file_str[current_slice_start..input_file_str.len()])};

    //println!("slices: {:?}", enabled_slices);
    let sum:i32 = enabled_slices.iter().map(|s|sum_mul_in_slice(*s)).sum();
    print!("Answer part 2: {}", sum);
}