use crate::utils::utils::get_input_file;
use num_integer::lcm;

pub fn day13_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d13.txt");
    let input:Vec<((i64, i64), (i64, i64), (i64, i64))> = input_str.split("\r\n\r\n").map(|game_str|{
        let mut g_iter = game_str.lines();
        let l1 = g_iter.next().expect("has 3 lines");
        let l2 = g_iter.next().expect("has 3 lines");
        let l3 = g_iter.next().expect("has 3 lines");
        (extract_deltas(l1), extract_deltas(l2), extract_targets(l3))
    }).collect();   

    // Part 1
    let mut count = 0;
    for i in input.iter(){
        if let Some(t) = part1_calc(i){
            count+=t;
        }
    }
    println!("Answer to part 1: {}", count);

    // Part 2
    let mut count = 0;
    for i in input.iter(){
        if let Some(t) = part2_calc(i){
            count+=t;
        }
    }
    println!("Answer to part 2: {}", count);
}

fn extract_deltas(line: &str)->(i64, i64){
    let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
    let ignore = &[',', 'X', 'Y', '='];
    let x = parts[2].trim_matches(ignore);
    let y = parts[3].trim_matches(ignore);
    (x.parse().expect("should be int"), y.parse().expect("should be int"))
}

fn extract_targets(line: &str)->(i64, i64){
    let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
    let ignore = &[',', 'X', 'Y', '='];
    let x = parts[1].trim_matches(ignore);
    let y = parts[2].trim_matches(ignore);
    (x.parse().expect("should be int"), y.parse().expect("should be int"))
}


fn part1_calc(input:&((i64, i64), (i64, i64), (i64, i64)))->Option<i64>{
    let ((ax, ay), (bx, by), (tx, ty)) = input.clone();

    let mut b_count = tx/bx +1;
    let mut a_count = 0;

    let mut x = b_count*bx;
    let mut y = b_count*by;
    while (x!=tx || y!=ty) && b_count>=0{
        if x>tx || y>ty{
            b_count -= 1;
            x-=bx;
            y-=by;
        }else{
            a_count += 1;
            x+=ax;
            y+=ay
        }
    }
    return if b_count>=0{Some(a_count*3 + b_count)}else{None}
}


fn part2_calc(input:&((i64, i64), (i64, i64), (i64, i64)))->Option<i64>{
    let ((ax, ay), (bx, by), (tx, ty)) = input.clone();
    let tx = tx + 10000000000000;
    let ty = ty+ 10000000000000;
    
    // Solve the simultaneous equations:
    // C*ax + K*bx = tx
    // C*ay + K*by = ty

    // Solve for K

    let lcm_a = lcm(ax, ay);
    let mul_x_eq_by = lcm_a/ax;
    let mul_y_eq_by = lcm_a/ay;

    let first_c = bx*mul_x_eq_by;
    let second_c = by*mul_y_eq_by;
    let new_tx = tx*mul_x_eq_by;
    let new_ty = ty*mul_y_eq_by;

    let b_count_const = first_c-second_c;
    let equal_to_const = new_tx-new_ty;

    if equal_to_const%b_count_const!=0{return None}
    let b_count = equal_to_const/b_count_const;
    
    let a_count = (tx - b_count*bx)/ax;

    Some(3*a_count + b_count)
}