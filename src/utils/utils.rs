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


pub struct Grid<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize
}

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
}

impl<'a, T> GridPoint<'a, T>{
    pub fn value(&'a self) -> &'a T{
        &self.grid.data[self.y][self.x]
    } 

    pub fn _left(&self) -> Option<GridPoint<T>>{
        if self.x == 0 {return None}
        self.grid.at(self.x-1, self.y)
    }

    pub fn _right(&self) -> Option<GridPoint<T>> {
        if self.x == self.grid.width-1{return None}
        self.grid.at(self.x+1, self.y)
    }

    pub fn _up(&self) -> Option<GridPoint<T>> {
        if self.y == 0 {return None}
        self.grid.at(self.x, self.y-1)
    }

    pub fn _down(&self) -> Option<GridPoint<T>> {
        if self.y == self.grid.height-1{return None}
        self.grid.at(self.x, self.y+1)
    }
    
    pub fn up_left(&self) -> Option<GridPoint<T>>{
        if self.x == 0 {return None}
        if self.y == 0 {return None}
        self.grid.at(self.x-1, self.y-1)
    }

    pub fn up_right(&self) -> Option<GridPoint<T>> {
        if self.x == self.grid.width-1{return None}
        if self.y == 0 {return None}
        self.grid.at(self.x+1, self.y-1)
    }

    pub fn down_left(&self) -> Option<GridPoint<T>> {
        if self.y == self.grid.height-1{return None}
        if self.x == 0 {return None}
        self.grid.at(self.x-1, self.y+1)
    }

    pub fn down_right(&self) -> Option<GridPoint<T>> {
        if self.y == self.grid.height-1{return None}
        if self.x == self.grid.width-1{return None}
        self.grid.at(self.x+1, self.y+1)
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



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_grid_slice_inbound() {
        //assert_eq!(bad_add(1, 2), 3);
    }
}