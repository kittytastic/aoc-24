use crate::utils::utils::get_input_file;

pub fn day1_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d1.txt");

    let split_input: Vec<Vec<&str>> = input_file_str.lines().map(|line: &str|{
        line.split_ascii_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    println!("{:?}", split_input);
    // TODO solve pt1
}