use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}, hash::Hash};

use crate::utils::utils::{get_input_file, Direction, Grid, Point};

pub fn day18_main(_second_part: bool, _extra_args: &Vec<String>){
    let input_string = get_input_file("d18.txt");
    
    let barriers: HashSet<Point> = input_string.lines().take(1024).map(|l|{
        let s = l.split(',').collect::<Vec<_>>();
        Point::new(s[0].parse().expect("Valid number"), s[1].parse().expect("Valid number"))
    }).collect();

    //println!("Barriers: {:?}", barriers);
    let height = 71;
    let width = 71;
    let end = Point::new(width-1, height-1);

    let mut to_visit: BinaryHeap<MinHeapContainer<usize, Point>> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let length;

    to_visit.push(MinHeapContainer{weight: 0, value: Point::new(0, 0)});

    loop {
        let Some(next) = to_visit.pop() else {unreachable!("Failed to find end");};
        //println!("{:?}", next.value);
        if next.value == end {length = next.weight; break;}
        if !visited.insert(next.value.clone()){continue;}

        for d in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left]{
            if let Some(p) = next.value.step_direction(d, width, height){
                if visited.contains(&p){continue;}
                if barriers.contains(&p){continue;}
                to_visit.push(MinHeapContainer{weight: next.weight+1, value: p});
            }
        }
    }

    println!("Answer to part 1: {}", length);

}

#[derive(PartialEq, Eq, Debug)]
struct MinHeapContainer<T, S>{
    weight: T,
    value: S
}

impl<T: Ord, S: Ord> Ord for MinHeapContainer<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight.cmp(&other.weight){
            Ordering::Equal => self.value.cmp(&other.value),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}


impl<T: Ord, S: Ord> PartialOrd for MinHeapContainer<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}