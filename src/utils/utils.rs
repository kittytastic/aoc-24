use std::path::Path;
use std::fs;

// Read an input file from the input data directory
pub fn get_input_file(file_name: &str) -> String{
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input_data").join(file_name);
    if !path.is_file(){
        println!("ERROR: file doesn't exist {:?}", path);
        std::process::exit(1);
    }

    return fs::read_to_string(path).expect("Should be able to read file");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn turn_90_clockwise(&self)->Direction{
        match self{
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft,
            Direction::UpLeft => Direction::UpRight,
        }
    }
}

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize
}

#[derive(Clone)]
pub struct GridPoint<'a, T>{
    grid: &'a Grid<T>,
    x: usize,
    y: usize
}

impl Grid<char>{
    pub fn new_from_str(txt: &str) -> Grid<char>{
        let input_data: Vec<Vec<char>> = txt.lines().map(|line: &str|{
            line.chars().map(|c|c).collect::<Vec<char>>()
        }).collect::<Vec<_>>();

        Grid::new(input_data)
    }
}

impl<T> Grid<T>{
    pub fn new(data: Vec<Vec<T>>) -> Self{
        let width = data[0].len();
        let height = data.len();
        Self {data, height, width}
    }

    pub fn height(&self)->usize{
        self.height
    }

    pub fn width(&self)->usize{
        self.width
    }

    pub fn at<'a>(&'a self, x:usize, y:usize) -> Option<GridPoint<'a, T>> {
        if x>=self.width {return None;}
        if y>=self.height {return None;}
        Some(GridPoint {grid: &self, x, y})
    }

    pub fn set_val_at_foreign_point<'a, S>(&mut self, value: T, point: &GridPoint<'a, S>)->Result<(), ()>{
        if point.x > self.width{return Err(())}
        if point.y > self.height{return Err(())}
        self.data[point.y][point.x] = value;
        Ok(())
    }
    
    pub fn get_val_at_foreign_point<'a, S>(&self, point: &GridPoint<'a, S>)->Result<&T, ()>{
        if point.x > self.width{return Err(())}
        if point.y > self.height{return Err(())}
        Ok(&self.data[point.y][point.x])
    }

    pub fn set_val_at(&mut self, value: T, x:usize, y:usize){
        self.data[y][x] = value;
    }
}

impl<T: Clone> Grid<T>{
    pub fn new_from(value: T, height: usize, width: usize)->Self{
        let data: Vec<Vec<T>> = (0..height).map(|_|{(0..width).map(|_|value.clone()).collect()}).collect();
        Self{data, height, width}
    }
}

impl<T: Eq> Grid<T>{
    pub fn find_first_occurrence(&self, look_for: &T)->Option<GridPoint<T>>{
        for y in 0..self.height{
            for x in 0..self.width{
                if self.data[y][x] == *look_for{
                    return Some(GridPoint{grid: &self, x, y})
                }
            }
        }
        None
    }
    
    pub fn count_all_occurrence_of(&self, look_for: &T)->usize{
        let mut c = 0;
        for y in 0..self.height{
            for x in 0..self.width{
                if self.data[y][x] == *look_for{
                    c+=1;
                }
            }
        }
        c
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height{
            for x in 0..self.width{
                write!(f, "{}", self.data[y][x])?
            }
            writeln!(f, "")?
        }
        writeln!(f, "height: {}  width: {}", self.height, self.width)
    }
}

impl<'a, T> GridPoint<'a, T>{
    pub fn value(&'a self) -> &'a T{
        &self.grid.data[self.y][self.x]
    } 

    pub fn left(&self) -> Option<GridPoint<'a, T>>{
        if self.x == 0 {return None}
        self.grid.at(self.x-1, self.y)
    }

    pub fn right(&self) -> Option<GridPoint<'a, T>> {
        if self.x == self.grid.width-1{return None}
        self.grid.at(self.x+1, self.y)
    }

    pub fn up(&self) -> Option<GridPoint<'a, T>> {
        if self.y == 0 {return None}
        self.grid.at(self.x, self.y-1)
    }

    pub fn down(&self) -> Option<GridPoint<'a, T>> {
        if self.y == self.grid.height-1{return None}
        self.grid.at(self.x, self.y+1)
    }
    
    pub fn up_left(&self) -> Option<GridPoint<'a, T>>{
        if self.x == 0 {return None}
        if self.y == 0 {return None}
        self.grid.at(self.x-1, self.y-1)
    }

    pub fn up_right(&self) -> Option<GridPoint<'a, T>> {
        if self.x == self.grid.width-1{return None}
        if self.y == 0 {return None}
        self.grid.at(self.x+1, self.y-1)
    }

    pub fn down_left(&self) -> Option<GridPoint<'a, T>> {
        if self.y == self.grid.height-1{return None}
        if self.x == 0 {return None}
        self.grid.at(self.x-1, self.y+1)
    }

    pub fn down_right(&self) -> Option<GridPoint<'a, T>> {
        if self.y == self.grid.height-1{return None}
        if self.x == self.grid.width-1{return None}
        self.grid.at(self.x+1, self.y+1)
    }

    pub fn step_direction(&self, dir: Direction)->Option<GridPoint<'a, T>>{
        match dir{
            Direction::Up => self.up(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::UpRight => self.up_right(),
            Direction::DownRight => self.down_right(),
            Direction::DownLeft => self.down_left(),
            Direction::UpLeft => self.up_left(),
        }
    }
}

impl<'a, T:Clone> GridPoint<'a, T>{
    pub fn right_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.x > self.grid.width-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.data[self.y][self.x+d].clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.y > self.grid.height-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.data[self.y+d][self.x].clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_right_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.y > self.grid.height-slice_len{return None}
        if self.x > self.grid.width-slice_len{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.data[self.y+d][self.x+d].clone()}).collect::<Vec<T>>())
    }
    
    pub fn down_left_slice(&self, slice_len: usize) -> Option<Vec<T>> {
        if self.y > self.grid.height-slice_len{return None}
        if self.x < slice_len-1{return None}
        Some((0..slice_len).into_iter().map(|d|{self.grid.data[self.y+d][self.x-d].clone()}).collect::<Vec<T>>())
    }
} 

impl<'a, T: std::fmt::Display> std::fmt::Display for GridPoint<'a, T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grid_slice_inbound() {
        //assert_eq!(bad_add(1, 2), 3);
    }
}