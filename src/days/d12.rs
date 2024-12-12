use std::{collections::HashSet, f32::consts::E};

use crate::utils::utils::{get_input_file, Direction, Grid, Point};
pub fn day12_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_str = get_input_file("d12.txt");

    let grid = Grid::new_from_str(&input_str);

    // Part 1
    let mut visited = Grid::new_from('.', grid.height(), grid.width());
    let mut total_cost = 0;
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let p = Point::new(x, y);
            if *visited.at_point(&p) == '.'{
                let current_crop = *grid.at_point(&p);
                let (area, perimeter) = dfs_p1(&Point::new(x, y), current_crop, &grid, &mut visited);
                total_cost += area*perimeter;
            }
        }       
    }
    println!("Answer to part 1: {}", total_cost);
    
    // Part 2
    let mut visited:Grid<i32> = Grid::new_from(0, grid.height(), grid.width());
    let mut total_cost = 0;
    let mut id = 0;
    for y in 0..grid.height(){
        for x in 0..grid.width(){
            let p = Point::new(x, y);
            if *visited.at_point(&p) == 0{
                let current_crop = *grid.at_point(&p);
                let mut boundaries: HashSet<(Point, Direction)> = HashSet::new();
                id += 1;
                let area = dfs_p2(&Point::new(x, y), id, current_crop, &grid, &mut visited, &mut boundaries);
                let edges = count_corners(&mut boundaries, &visited);
                total_cost += area*edges;
            }
        }       
    }
    println!("Answer to part 2: {}", total_cost)
}

fn dfs_p1(point: &Point, crop_val: char, grid: &Grid<char>, visited: &mut Grid<char>)->(usize, usize){
    if *grid.at_point(point) != crop_val {return (0,1)}
    if *visited.at_point(point) != '.' {return (0,0)}
    visited.set(point.get_x(), point.get_y(), 'X');    

    let mut area = 1;
    let mut perimeter = 0;
    for d in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
        if let Some(new_p) = point.step_direction(d, grid.width(), grid.height()){
            let (sub_area, sub_perimeter) = dfs_p1(&new_p, crop_val, grid, visited);
            area += sub_area;
            perimeter += sub_perimeter;
        }else{
            perimeter += 1
        }
    }

    (area, perimeter)
}


fn dfs_p2(point: &Point, id:i32, crop_val: char, grid: &Grid<char>, visited: &mut Grid<i32>, boundaries: &mut HashSet<(Point, Direction)>)->usize{
    if *grid.at_point(point) != crop_val {return 0}
    if *visited.at_point(point) != 0 {return 0}
    visited.set(point.get_x(), point.get_y(), id);    
    let mut area = 1;
    for d in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]{
        if let Some(new_p) = point.step_direction(d, grid.width(), grid.height()){
            if *grid.at_point(&new_p)!=crop_val {
                boundaries.insert((point.clone(), d));
            } 
            let sub_area = dfs_p2(&new_p, id, crop_val, grid, visited, boundaries);
            area += sub_area;
        }else{
                boundaries.insert((point.clone(), d));
        }
    }

    area
}

fn count_corners(boundaries: &mut HashSet<(Point, Direction)>, grid: &Grid<i32>)->usize{
    
    // For debug
    let mut sorted_b:Vec<_> = boundaries.iter().cloned().collect(); 
    sorted_b.sort(); 
    
    let mut corner_count = 0;
    for (point, edge) in sorted_b.iter(){
        let interior_corners = vec![
            (point.clone(), edge.turn_90_clockwise(), point.step_direction(edge.turn_45_clockwise(), grid.width(), grid.height())),
            (point.clone(), edge.turn_90_anticlockwise(), point.step_direction(edge.turn_45_anticlockwise(), grid.width(), grid.height())),
        ];
        for (new_point, new_edge, touching) in interior_corners{
            if boundaries.contains(&(new_point.clone(), new_edge)){
                let connected_corner = if let Some(p) = touching{
                    grid.at_point(point) == grid.at_point(&p)
                }else{false};
                if  !connected_corner{
                    corner_count += 1;
                }
            }
        }


        let exterior_corners = vec![
            (point.step_direction(edge.turn_45_anticlockwise(), grid.width(), grid.height()), edge.turn_90_clockwise()),
            (point.step_direction(edge.turn_45_clockwise(), grid.width(), grid.height()), edge.turn_90_anticlockwise())
        ];
        for (new_point, new_edge) in exterior_corners{
            if let Some(p) = new_point{
                if boundaries.contains(&(p.clone(), new_edge)){
                    corner_count += 1;
                }
            }
        } 
    }
    corner_count/2
}


// Hmmmm.. I think this one would have worked, got a bug tbd. I'm undecided if I like it more than the other version
/*fn walk_path(start_pos: &(Point, Direction), pos: &(Point, Direction), boundaries: &mut HashSet<(Point, Direction)>, grid: &Grid<char>)->usize{
    println!("Walking: {:?}", pos);
    boundaries.remove(pos);
    let (point, side) = pos;

    let possible_moves = vec![
        // Straight
        (point.step_direction(side.turn_90_clockwise(), grid.width(), grid.height()), *side, 0),
        (point.step_direction(side.turn_90_anticlockwise(), grid.width(), grid.height()), *side, 0),
        
        // Turn
        (point.step_direction(*side, grid.width(), grid.height()), side.turn_90_clockwise(), 1),
        (point.step_direction(*side, grid.width(), grid.height()), side.turn_90_anticlockwise(), 1),
        (Some(point.clone()), side.turn_90_clockwise(), 1),
        (Some(point.clone()), side.turn_90_anticlockwise(), 1),
        ];

    for (new_point, new_side, turn_count) in possible_moves{
        if let Some(new_point) = new_point{
            if (new_point.clone(), new_side) == *start_pos{return turn_count;}
            if boundaries.contains(&(new_point.clone(), new_side)){
                return turn_count + walk_path(start_pos, &(new_point, new_side), boundaries, grid)
            }
        }
    }
    unreachable!();
}

fn walk_boundaries(boundaries: &mut HashSet<(Point, Direction)>, grid: &Grid<char>)->usize{
    println!("boundaries: {:?}", boundaries);

   let mut turns = 0;

    while !boundaries.is_empty(){
        let start_point = boundaries.iter().next().unwrap().clone();
        println!("Starting walk at: {:?}", start_point);
        turns += walk_path(&start_point, &start_point, boundaries, grid);
    }
        
    
    turns
}*/