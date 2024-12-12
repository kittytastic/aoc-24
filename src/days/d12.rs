use crate::utils::utils::{get_input_file, Direction, Grid, Point};
pub fn day12_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d12.txt");

    let grid = Grid::new_from_str(&input_str);
    let mut visited = Grid::new_from('.', grid.height(), grid.width());

    let mut total_cost = 0;

    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let p = Point::new(x, y);
            if *visited.at_point(&p) == '.'{
                let current_crop = *grid.at_point(&p);
                let (area, perimeter) = dfs(&Point::new(x, y), current_crop, &grid, &mut visited);
                //println!("Crop {}  {} * {}", current_crop, area, perimeter);
                total_cost += area*perimeter;
            }
        }       
    }
    println!("Answer to part 1: {}", total_cost)
}

fn dfs(point: &Point, crop_val: char, grid: &Grid<char>, visited: &mut Grid<char>)->(usize, usize){
    //print!("DFS: {}", point);
    if *grid.at_point(point) != crop_val {return (0,1)}
    if *visited.at_point(point) != '.' {return (0,0)}
    visited.set(point.get_x(), point.get_y(), 'X');    

    let mut area = 1;
    let mut perimeter = 0;
    for d in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
        if let Some(new_p) = point.step_direction(d, grid.width(), grid.height()){
            let (sub_area, sub_perimeter) = dfs(&new_p, crop_val, grid, visited);
            area += sub_area;
            perimeter += sub_perimeter;
        }else{
            perimeter += 1
        }
    }

    (area, perimeter)
}