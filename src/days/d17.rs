use std::usize;

use crate::utils::utils::{get_input_file, get_number_substr};

pub fn day17_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d17.txt");
    let l:Vec<&str> = input_string.lines().collect();
    let reg_a:usize = get_number_substr(l[0])[0].parse().expect("Valid number");
    let reg_b:usize = get_number_substr(l[1])[0].parse().expect("Valid number");
    let reg_c:usize = get_number_substr(l[2])[0].parse().expect("Valid number");
    let program:Vec<usize> = get_number_substr(l[4]).iter().map(|s|{s.parse().expect("Valid number")}).collect();

    println!("Reg a: {} b: {} c: {}; Program: {:?}", reg_a, reg_b, reg_c, program);

    let ops:Vec<(usize, usize)> = program.chunks(2).map(|c|(c[0], c[1])).collect();
    let part1_output = run_program(&ops, reg_a, reg_b, reg_c);
    let output_str = part1_output.iter().map(|v|{v.to_string()}).collect::<Vec<_>>().join(",");
    println!("Answer to part 1: {}", output_str);

    part2(&program);

}

fn operand_to_str(operand: usize)->String{
    match operand{
        0..=3 => format!("{}", operand),
        4 => format!("%a"),
        5 => format!("%b"),
        6 => format!("%c"),
        7 => format!("%x"),
        _ => unreachable!("") 
    }
}

fn op_to_str(op_code: usize, operand: usize)->String{
    match op_code{
        0 => format!("(adv) %a = %a >> {}", operand_to_str(operand)),
        1 => format!("(bxl) %b = %b ^ {}", operand),
        2 => format!("(bst) %b = {} % 8", operand_to_str(operand)),
        3 => format!("(jnz) jnz %a -> {}", operand),
        4 => format!("(bxc) %b = %b ^ %c"),
        5 => format!("(out) out {} % 8", operand_to_str(operand)),
        6 => format!("(bdv) %b = %a >> {}", operand_to_str(operand)),
        7 => format!("(cdv) %c = %a >> {}", operand_to_str(operand)),
        _ => unreachable!(),
    }
}

fn fetch_operand(operand: usize, reg_a: &mut usize, reg_b: &mut usize, reg_c: &mut usize)->usize{
  match operand{
        0..=3 => operand,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        7 | _ => unreachable!("") 
    }
}

fn apply_op(op_code: usize, operand: usize, pc: &mut usize, reg_a: &mut usize, reg_b: &mut usize, reg_c: &mut usize)->Option<usize>{
    let mut return_val = None;
    
    match op_code{
        0 => {*reg_a = *reg_a >> fetch_operand(operand, reg_a, reg_b, reg_c);},
        1 => {*reg_b = *reg_b ^ operand},
        2 => {*reg_b = fetch_operand(operand, reg_a, reg_b, reg_c)%8;},
        3 => {if *reg_a != 0 {*pc = operand;}else{*pc+=1;}},
        4 => {*reg_b = *reg_b ^ *reg_c;},
        5 => {return_val = Some(fetch_operand(operand, reg_a, reg_b, reg_c)%8)},
        6 => {*reg_b = *reg_a >> fetch_operand(operand, reg_a, reg_b, reg_c);},
        7 => {*reg_c = *reg_a >> fetch_operand(operand, reg_a, reg_b, reg_c);},
        _ => unreachable!(),
    }

    if op_code != 3 {
        *pc += 1
    }

    return_val
}

fn run_program(ops: &Vec<(usize, usize)>, mut reg_a: usize, mut reg_b: usize, mut reg_c: usize)->Vec<usize>{
    let mut pc = 0;
    let mut output: Vec<usize> = Vec::new();

    /*for (op_code, operand) in ops.iter(){
        println!("{}", op_to_str(*op_code, *operand));
    }*/

    while pc < ops.len(){
        let (op_code, operand) = ops[pc];
        //println!("[{}] {}", pc, op_to_str(op_code, operand));
        let result = apply_op(op_code, operand, &mut pc, &mut reg_a, &mut reg_b, &mut reg_c);
        if let Some(r) = result {output.push(r);}
    }

    output
}

fn part2(program: &Vec<usize>){
    /*for a_shift in 0..8{
        for a_low in 0..8{
            let b1 = a_low;
            let b2 = b1 ^ 1;
            let c1 = a_shift;
            let b3 = b2 ^ c1;
            let b4 = b3 ^ 4;
            println!("{} {} (takes {})-> {}", a_low, a_shift, b2, b4%8);
        }
    }*/
    /*for c in 0..8{
        for b in 0..8{
            println!("b: {} c: {} = {}", b, c, b^c^4^1)
        }
    }*/
    /*let solution = find_solution(15, 0, program);
    println!("Answer to part 2: {:?}", solution);
    if solution.is_none(){println!("Cannot get answer"); return;}
    */
    // Other: 202366627358720
    // First: 202366115129871
    println!("{:o}", 202366627358720 as usize);
    println!("{:o}", 202366115129871 as usize);
    let x = other_attempt2(program);
    let ops:Vec<(usize, usize)> = program.chunks(2).map(|c|(c[0], c[1])).collect();
    let part1_output = run_program(&ops, x, 0, 0);
    let output_str = part1_output.iter().map(|v|{v.to_string()}).collect::<Vec<_>>().join(",");
    println!("{}", output_str)
}

fn find_solution(digit_idx: usize, num: usize, target_digits: &Vec<usize>)->Option<usize>{
    if digit_idx == 9 {println!("hi, {} {} {:?}", digit_idx, num, target_digits);}
    for b in 0..8{
        let b1 = b^1;
        let c = num>>(b1+(digit_idx*3));
        let b2 = b1 ^ c ^ 4;
        let digit = b2 % 8;
        if digit != target_digits[digit_idx]{continue;} // No match - try again
        
        let new_num = num + (b<<(3*digit_idx));
        if digit_idx == 0 {return Some(new_num)} // Base case
        let sub_solution = find_solution(digit_idx-1, new_num, target_digits);
        if sub_solution.is_some() {return sub_solution;}
    }

    None
}

fn other_attempt(program: &Vec<usize>)->usize{
    let ops:Vec<(usize, usize)> = program.chunks(2).map(|c|(c[0], c[1])).collect();
    let mut a = 0;
    for j in 0..16{
        a*=8;
        for i in 0..8{
            let output = run_program(&ops, a+i, 0, 0);
            if output == &program[(15-j)..]{
                println!("{:?}, {:?}", output, &program[(15-j)..]);
                a+=i;
                break;
                println!("Match!")
            }
        }
    }

    println!("{a}");
    a
}


fn other_attempt2(program: &Vec<usize>)->usize{
    let ops:Vec<(usize, usize)> = program.chunks(2).map(|c|(c[0], c[1])).collect();
    for i in 0..(8*8*8*8){
        let a = 0o5600644674020000 + i;
            let output = run_program(&ops, a, 0, 0);
            if output == *program{
                println!("Match! {a}");
                return a
        }
    }
    return 0;
}