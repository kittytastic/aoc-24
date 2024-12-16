use std::collections::HashMap;

use crate::utils::utils::{get_input_file, Direction, Grid, Point};

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
    part2(&grid, &directions);
    
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

fn part2(original_grid: &Grid<char>, directions: &Vec<Direction>){
    let mut grid = expand_grid(original_grid);
    
    let mut cur_robot = grid.find_first_occurrence(&'@').expect("Must be a robot");
    for d in directions{
        let next_robot = cur_robot.step_direction(*d, grid.width(), grid.height()).expect("Inbounds");
        if *grid.at_point(&next_robot) == '.' {
            grid.set_point(&next_robot, '@');
            grid.set_point(&cur_robot, '.');
            cur_robot = next_robot; 
            continue;}
        if *grid.at_point(&next_robot) == '#' {cur_robot = cur_robot; continue;}

        let mut in_push:HashMap<Point, char> = HashMap::new();
        let can_push = try_push(&next_robot, d, &grid, &mut in_push);
        if can_push {
            push(d, &mut grid, &in_push);
            grid.set_point(&next_robot, '@');
            grid.set_point(&cur_robot, '.');
            cur_robot = next_robot; 
        }else{
            cur_robot = cur_robot;
        }
    }

    let mut score = 0;
    for x in 0..grid.width(){
        for y in 0..grid.height(){
            if *grid.get(x, y) == '[' {
                score += y*100 + x;
            } 
        }
    }
    println!("Answer to part 2: {}", score);
}


fn expand_grid(original_grid: &Grid<char>) ->Grid<char>{
    let mut grid = Grid::new_from('.', original_grid.height(), original_grid.width()*2);
    for y in 0..original_grid.height(){
        for x in 0..original_grid.width(){
            let v = *original_grid.get(x, y);
            match v {
                '#' => {
                    grid.set(x*2, y, '#');
                    grid.set(x*2+1, y, '#');
                }
                'O' => {
                    grid.set(x*2, y, '[');
                    grid.set(x*2+1, y, ']');
                }
                '@' => {
                    grid.set(x*2, y, '@');
                }
                _ => ()
            }
        }
    }
    grid
}

fn try_push(point: &Point, direction: &Direction, grid: &Grid<char>, visited: &mut HashMap<Point, char>)->bool{
    if visited.contains_key(point){return true;}
    let point_val = grid.at_point(point);
    if *point_val == '.' {return true;}
    if *point_val == '#' {return false;}

    visited.insert(point.clone(), *point_val);
    let other_half = if *point_val == '['{
        try_push(&point.right(grid.width(), grid.height()).expect("Inbounds"), direction, grid, visited)
    }else{
        try_push(&point.left(grid.width(), grid.height()).expect("Inbounds"), direction, grid, visited)
    };

    let move_half = try_push(&point.step_direction(*direction, grid.width(), grid.height()).expect("Inbounds"), direction, grid, visited);
    other_half && move_half
}

fn push(direction: &Direction, grid: &mut Grid<char>, points: &HashMap<Point, char>){
    for (p, _) in points.iter(){
        grid.set_point(p, '.');
    }

    for (p, v) in points.iter(){
        let moved_p = p.step_direction(*direction, grid.width(), grid.height()).expect("Boxes stay inbound");
        grid.set_point(&moved_p, *v);
    }
}