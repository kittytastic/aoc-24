use std::collections::HashMap;

use crate::utils::utils::get_input_file;
pub fn day11_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d11.txt");
    let input_nums:Vec<usize> = input_str.split_ascii_whitespace().map(|s|{s.parse().expect("is num")}).collect();
    println!("{:?}", input_nums);
    println!("{} {}", usize::ilog10(3), usize::ilog10(13));
    println!("{} {}", has_even_digits(3), has_even_digits(13));
    println!("{:?} {:?}", split_in_half(13), split_in_half(1234));

    let total_blink = 75;
    let mut total_count = 0;
    for v in input_nums{
        total_count += blink(total_blink, v, &mut HashMap::new());
    }

    println!("Answer to part 1: {}", total_count)

}

fn has_even_digits(val: usize)->bool{
    val.ilog10() %2 == 1
}

fn split_in_half(val: usize)->(usize, usize){
    let digits = val.ilog10() + 1;
    let high = val / usize::pow(10, digits/2);
    let shift_high = high * usize::pow(10, digits/2);
    let low = val - shift_high;
    (high,low)
}

fn blink(remaining_blink: u32, store_val: usize, memo: &mut HashMap<(usize, u32), usize>)->usize{
    if remaining_blink==0{return 1;}
    if let Some(c) = memo.get(&(store_val, remaining_blink)){
        //println!("[MEMO] r: {} v: {}", remaining_blink, store_val);
        return *c;
    }

    let count = if store_val == 0{
        //println!("[ZERO] r: {} v: {}", remaining_blink, store_val);
        blink(remaining_blink-1, 1, memo)
    } else if has_even_digits(store_val){
        //println!("[SPLIT] r: {} v: {}", remaining_blink, store_val);
        let (a, b) = split_in_half(store_val);
        blink(remaining_blink-1, a, memo) + blink(remaining_blink-1, b, memo)
    }else{
        //println!("[ELSE] r: {} v: {}", remaining_blink, store_val);
        blink(remaining_blink-1, store_val*2024, memo)
    };

    memo.insert((store_val, remaining_blink), count);
    count
}