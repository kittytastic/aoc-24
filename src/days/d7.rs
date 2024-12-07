use crate::utils::utils::get_input_file;

pub fn day7_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_file_string = get_input_file("d7.txt");
    let input: Vec<(u64, Vec<u64>)> = input_file_string.lines().map(|line|{
        let parts = line.split(':').collect::<Vec<&str>>();
        let target = parts[0].parse::<u64>().expect("is num");
        let vals = parts[1].split_ascii_whitespace().map(|v|{v.parse::<u64>().expect("is num")}).collect::<Vec<u64>>();
        (target, vals)
    }).collect();

    let part_one:u64 = input.iter().map(|(target, vals)|{if part1_step(0, *target, &vals){*target}else{0}}).sum();
    println!("Answer part 1: {}", part_one); 

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
    low.checked_add(u64::checked_pow(10, low.checked_ilog10()?+1)?.checked_mul(high)?)
}


fn part2_step(val: u64, target: u64, remaining: &[u64])->bool{
    if remaining.len() == 0 {return val==target;}
    if val>target {return false;}
    let next_val = remaining[0];
    let using_plus = part2_step(val+next_val, target, &remaining[1..]);
    let using_mul = part2_step(val*next_val, target, &remaining[1..]);
    let using_concat = if let Some(c) = concat_base_10(next_val, val) {part2_step(c, target, &remaining[1..])}else{false};

    return using_plus | using_mul | using_concat;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        println!("{}",u64::ilog10(6));
        assert_eq!(concat_base_10(3, 4), Some(43));
        assert_eq!(concat_base_10(10, 19), Some(1910));
        assert_eq!(concat_base_10(6, 15), Some(156));
    }
}