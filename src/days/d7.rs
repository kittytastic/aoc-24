use crate::utils::utils::get_input_file;

pub fn day7_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_file_string = get_input_file("d7_example.txt");
    let input: Vec<(u64, Vec<u64>)> = input_file_string.lines().map(|line|{
        let parts = line.split(':').collect::<Vec<&str>>();
        let target = parts[0].parse::<u64>().expect("is num");
        let vals = parts[1].split_ascii_whitespace().map(|v|{v.parse::<u64>().expect("is num")}).collect::<Vec<u64>>();
        (target, vals)
    }).collect();

    let part_one:u64 = input.iter().map(|(target, vals)|{if part1_step(0, *target, &vals){*target}else{0}}).sum();
    println!("Answer part 1: {}", part_one); 

    println!("{}", (6 as u64).ilog10());
    println!("{}", (9 as u64).ilog10());
    println!("{}", (99 as u64).ilog10());
    println!("{}", (100 as u64).ilog10());

    println!("{:?}", concat_base_10(6, 15));
    
    let part_two:u64 = input.iter().map(|(target, vals)|{if part2_step(0, *target, &vals){*target}else{0}}).sum();
    //println!("ASDASD: {}", part2_step(0, 156, &vec![15, 6]))
    println!("Answer part 2: {}", part_two); 

}

fn part1_step(val: u64, target: u64, remaining: &[u64])->bool{
    if remaining.len() == 0 {return val==target;}
    if val>target {return false;}
    let next_val = remaining[0];
    let using_plus = part1_step(val+next_val, target, &remaining[1..]);
    let using_mul = part1_step(val*next_val, target, &remaining[1..]);
    return using_plus | using_mul;
}

fn concat_base_10(low: u64, high: u64)->Option<u64>{
    low.checked_add(u64::checked_pow(10, high.checked_ilog10()?)?.checked_mul(high)?)
}


fn part2_step(val: u64, target: u64, remaining: &[u64])->bool{
    //println!("Step: {} {} {:?}", val, target, remaining);
    if remaining.len() == 0 {return val==target;}
    if val>target {return false;}
    let next_val = remaining[0];
    let using_plus = part2_step(val+next_val, target, &remaining[1..]);
    let using_mul = if let Some(v) = val.checked_mul(next_val){part2_step(v, target, &remaining[1..])}else{false};
    let using_concat = if remaining.len()>1{
        let mut local_remaining = (&remaining[1..]).to_vec();
        //println!("{:?}", local_remaining);
        if let Some(concat) = concat_base_10(local_remaining[0], next_val){
            //println!("{}", concat);
            local_remaining[0] = concat;
            part2_step(val, target, &local_remaining)
        }else{false}
    }else{false};
    return using_plus | using_mul | using_concat;
}