use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet}};

use crate::utils::utils::{get_input_file, Direction, Grid, Point};

pub fn day16_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d16.txt");
    let grid = Grid::new_from_str(&input_string);
    let start = grid.find_first_occurrence(&'S').expect("Contains start");

    let mut next_points: BinaryHeap<WeightedPoint> = BinaryHeap::new();
    let mut visited: HashSet<(Direction, Point)> = HashSet::new();
    next_points.push(WeightedPoint {score: 0, direction: Direction::Right, point: start});
    let mut best_score = 0;

    loop {
        let Some(p) = next_points.pop() else{panic!("We've ran out of points without reaching the end!")};
        if visited.contains(&(p.direction, p.point.clone())){continue;}
        if *grid.at_point(&p.point) == '#' {continue;}
        if *grid.at_point(&p.point) == 'E' {best_score = p.score; break;}
        visited.insert((p.direction, p.point.clone()));
        next_points.push(WeightedPoint{score: p.score + 1, direction: p.direction, point: p.point.step_direction(p.direction, grid.width(), grid.height()).expect("Inbounds")});
        next_points.push(WeightedPoint{score: p.score + 1000, direction: p.direction.turn_90_clockwise(), point: p.point.clone()});
        next_points.push(WeightedPoint{score: p.score + 1000, direction: p.direction.turn_90_anticlockwise(), point: p.point});
    }

    println!("Answer to part 1: {}", best_score);

}

#[derive(PartialEq, Eq, Debug)]
struct WeightedPoint{
    score: usize,
    direction: Direction,
    point: Point,
}

impl Ord for WeightedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score){
            Ordering::Equal => self.point.cmp(&other.point),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}


impl PartialOrd for WeightedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}