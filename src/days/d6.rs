use std::collections::HashMap;

use crate::utils::utils::{get_input_file, Direction, Grid, GridPoint};
pub fn day6_main(second_part: bool, _extra_args: &Vec<String>){
    let input_file_str = get_input_file("d6.txt");
    if second_part {
        part2(&input_file_str);
    }else{
        part1(&input_file_str);
    }
}



fn part1(input: &str){
    let grid = Grid::new_from_str(&input);

    let mut visited = Grid::new_from('.', grid.height(), grid.width());
    let start = grid.find_first_occurrence(&'^').expect("Should have start");
    visited.set_val_at_foreign_point('X', &start).expect("Same size grids");

    let mut current_dir = Direction::Up;
    let mut current_point = Some(start);

    while let Some(cur_point) = current_point {
        let try_point = cur_point.step_direction(current_dir);
        let new_point = match try_point{
            Some(new_point_val) if *(new_point_val.value()) == '#' =>{
                current_dir = current_dir.turn_90_clockwise();
                Some(cur_point)
            }
            Some(new_point_val) => {
                visited.set_val_at_foreign_point('X', &new_point_val).expect("Grid same size");
                Some(new_point_val)
            }
            None => {None}
        };
        current_point = new_point;
    }

    println!("Answer part 1: {}", visited.count_all_occurrence_of(&'X'))
}


fn part2(input: &str){
    // Super lazy brute force
    let mut grid = Grid::new_from_str(&input);
    let mut loop_count = 0;
    for y in 0..grid.height(){
        println!("y: {}", y);
        for x in 0..grid.width(){
            let v = grid.get_point(x,y).expect("Inbounds").value().clone();
            if v == '#' || v == '^' {continue;}
            grid.set(x, y, '#');
            if does_grid_loop(&grid){
                loop_count +=1
            }
            grid.set(x, y, '.');
        }
    }
    println!("Answer to part 2: {}", loop_count);
}

fn does_grid_loop<'a>(grid: &'a Grid<char>)->bool{
    let start = grid.find_first_occurrence(&'^').expect("Should have start");
    let mut visited = HashMap::<Direction, Grid<char>>::new();
    visited.insert(Direction::Up, Grid::new_from('.', grid.height(), grid.width()));
    visited.insert(Direction::Down, Grid::new_from('.', grid.height(), grid.width()));
    visited.insert(Direction::Left, Grid::new_from('.', grid.height(), grid.width()));
    visited.insert(Direction::Right, Grid::new_from('.', grid.height(), grid.width()));
    visited.get_mut(&Direction::Up).expect("").set_val_at_foreign_point('X', &start).expect("Same size grids");

    let mut current_dir = Direction::Up;
    let mut current_point = Some(start);

    while let Some(cur_point) = current_point {
        let try_point = cur_point.step_direction(current_dir);
        let new_point = match try_point{
            Some(new_point_val) if *(new_point_val.value()) == '#' =>{
                current_dir = current_dir.turn_90_clockwise();
                Some(cur_point)
            }
            Some(new_point_val) => {
                if *(visited.get_mut(&current_dir).expect("").get_val_at_foreign_point(&new_point_val).expect("")) == 'X' {return true;}
                visited.get_mut(&current_dir).expect("").set_val_at_foreign_point('X', &new_point_val).expect("Grid same size");
                Some(new_point_val)
            }
            None => {None}
        };
        current_point = new_point;
    }

    false
}