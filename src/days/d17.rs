use std::usize;

use crate::utils::utils::{get_input_file, get_number_substr};

pub fn day17_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d17.txt");
    let l:Vec<&str> = input_string.lines().collect();
    let mut reg_a:usize = get_number_substr(l[0])[0].parse().expect("Valid number");
    let mut reg_b:usize = get_number_substr(l[1])[0].parse().expect("Valid number");
    let mut reg_c:usize = get_number_substr(l[2])[0].parse().expect("Valid number");
    let program:Vec<usize> = get_number_substr(l[4]).iter().map(|s|{s.parse().expect("Valid number")}).collect();

    println!("Reg a: {} b: {} c: {}; Program: {:?}", reg_a, reg_b, reg_c, program);

    let ops:Vec<(usize, usize)> = program.chunks(2).map(|c|(c[0], c[1])).collect();
    let mut output: Vec<usize> = Vec::new();
    let mut pc = 0;

    for (op_code, operand) in ops.iter(){
        println!("{}", op_to_str(*op_code, *operand));
    }

    while pc < ops.len(){
        let (op_code, operand) = ops[pc];
        //println!("[{}] {}", pc, op_to_str(op_code, operand));
        let result = apply_op(op_code, operand, &mut pc, &mut reg_a, &mut reg_b, &mut reg_c);
        if let Some(r) = result {output.push(r);}
    }
    let output_str = output.iter().map(|v|{v.to_string()}).collect::<Vec<_>>().join(",");
    println!("Answer to part 1: {}", output_str);

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