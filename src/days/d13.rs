use crate::utils::utils::get_input_file;
pub fn day13_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d13.txt");
    let input:Vec<((i64, i64), (i64, i64), (i64, i64))> = input_str.split("\r\n\r\n").map(|game_str|{
        let mut g_iter = game_str.lines();
        let l1 = g_iter.next().expect("has 3 lines");
        let l2 = g_iter.next().expect("has 3 lines");
        let l3 = g_iter.next().expect("has 3 lines");
        (extract_deltas(l1), extract_deltas(l2), extract_targets(l3))
    }).collect();   

    let mut count = 0;
    for i in input.iter(){
        if let Some(t) = part1_calc(i){
            count+=t;
        }
    }
    println!("Answer to part 1: {}", count);
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