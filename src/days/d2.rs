use crate::utils::utils::get_input_file;

fn is_seq_safe(seq: &[i32])->bool{
    let first_diff = seq[1] - seq[0];
    let accending:bool = first_diff>0;
    let mut safe = true;
    for i in 1..seq.len(){
        let diff = seq[i] - seq[i-1];
        if diff==0{safe = false; break;}
        if diff.abs() > 3 {safe = false; break;}
        let diff_accending = diff>0;
        // Xor - true if one is different from the other
        if diff_accending ^ accending {safe=false; break;}
    }

    return safe;
}

pub fn day2_main(_second_part: bool, _extra_args: &Vec<String>){
     let input_file_str = get_input_file("d2.txt");

    let input_data: Vec<Vec<i32>> = input_file_str.lines().map(|line: &str|{
        line.split_ascii_whitespace().map(|s|s.parse().expect("Should be a number")).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // Part 1
    let mut safe_count = 0;
    for line in input_data.iter(){
        let first_diff = line[1] - line[0];
        let accending:bool = first_diff>0;
        let mut safe = true;
        for i in 1..line.len(){
            let diff = line[i] - line[i-1];
            if diff==0{safe = false; break;}
            if diff.abs() > 3 {safe = false; break;}
            let diff_accending = diff>0;
            // Xor - true if one is different from the other
            if diff_accending ^ accending {safe=false; break;}
        }
        if safe {safe_count+=1;}
    }

    println!("Answer part 1: {}", safe_count);
    
    // Part 2
    // Brute force - optimising my runtime ;)
    let mut safe_count_b = 0;
    for line in input_data.iter(){
        let mut safe = is_seq_safe(&line);
        for i in 0..line.len(){
            let mut line_clone = line.clone();
            line_clone.remove(i);
            safe |= is_seq_safe(&line_clone);
        }
        if safe {safe_count_b+=1;}
    }

    println!("Answer part 2: {}", safe_count_b);

}