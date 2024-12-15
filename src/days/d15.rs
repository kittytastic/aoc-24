use crate::utils::utils::{get_input_file, Direction, Grid};

pub fn day15_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d15.txt");
    let parts: Vec<&str> = input_string.split("\r\n\r\n").collect();
    let grid = Grid::new_from_str(parts[0]);
    let directions: Vec<Direction> = parts[1].chars().filter_map(|c|{match c{
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        _ => None,
    }}).collect();

    part1(&grid, &directions);
    
}

fn part1(original_grid: &Grid<char>, directions: &Vec<Direction>){
    let mut grid = original_grid.clone();

    let mut cur_robot = grid.find_first_occurrence(&'@').expect("Must be a robot");
    for d in directions{
        let next_robot = cur_robot.step_direction(*d, grid.width(), grid.height()).expect("Inbounds");
        if *grid.at_point(&next_robot) == '.' {
            grid.set_point(&next_robot, '@');
            grid.set_point(&cur_robot, '.');
            cur_robot = next_robot; 
            continue;}
        if *grid.at_point(&next_robot) == '#' {cur_robot = cur_robot; continue;}

        let first_box = next_robot.clone();
        let mut current_box = first_box;
        loop {
            current_box = current_box.step_direction(*d, grid.width(), grid.height()).expect("Inbounds");
            match grid.at_point(&current_box){
                '#' | '.' => break,
                'O' => continue,
                _ => unreachable!("Unknown point while following boxes: {}", grid.at_point(&current_box))
            }
        }

        match grid.at_point(&current_box){
            '#' => {cur_robot = cur_robot; continue;}
            '.' => {
                grid.set_point(&current_box, 'O');
                grid.set_point(&next_robot, '@');
                grid.set_point(&cur_robot, '.');
                cur_robot = next_robot;
                continue;
            },
            _ => unreachable!("Unknown point after following boxes"),
        }
    }

    let mut score = 0;
    for x in 0..grid.width(){
        for y in 0..grid.height(){
            if *grid.get(x, y) == 'O' {
                score += y*100 + x;
            } 
        }
    }
    println!("Answer to part 1: {}", score);


}