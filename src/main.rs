use std::io::Write;
use std::io;
use std::ops::{IndexMut, Index};
use rand::Rng;

enum Status{
    Alive,
    Dead,
}

struct Dimension{
    height: u32,
    weight: u32,
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(data:Vec<Vec<T>>) -> Self {

        Self { data}
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
impl<T> IndexMut<(usize, usize)> for Matrix<T> {

    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

fn user_input()-> (u32, u32){

    let mut dimensions:[u32;2] = [0;2];

    for i in 0..2 {

        loop {

            println!("Input {}. dimension of grid:", i + 1);

            let mut line = String::new();
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
        

            dimensions[i] = match line.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            line.clear();
            break;

        }

        println!("{}. dim is: {}", i + 1, dimensions[i]);

    }

    (dimensions[0], dimensions[1])
}

fn main(){

    let (height, width) = user_input();

    let mut tmp:Vec<Vec<u32>> = vec![];

    for i in 0..height{
        tmp.push(vec![]);

        for j in 0..width{
            tmp.last_mut().unwrap().push(rand::thread_rng().gen_range(0..2));
        }
    }

    let grid:Matrix<u32> = Matrix {
        data: tmp
    };

    print_matrix(grid.clone(),height, width);

    next_generation(grid, height, width);
}

fn print_matrix(matrix: Matrix<u32>, height:u32, width:u32){

    for i in 0..height as usize{

        for j in 0..width as usize{
            print!("{}", matrix[(i,j)]);
        }
        println!("\n")
    }
}

fn next_generation(grid: Matrix<u32>, mut height:u32, mut width:u32){

    let mut tmp:Vec<Vec<u32>> = vec![];

    let mut count = 0;
    while count < height {
        tmp.push(vec![0; width as usize]);
        count +=1;
    }

    let mut next:Matrix<u32> = Matrix {
        data: tmp
    };

    height = height-1;
    width = width-1;
    for i in 1..height as isize{

        for j in 1..width as isize{
            
            let mut alive:u32 = 0;

            for k in -1..2 as isize{
                for q in -1..2 as isize{
                    alive += grid[((i+k) as usize, (j+q) as usize)];
                }
            }

            alive -= grid[(i as usize,j as usize)];

            if grid[(i as usize,j as usize)] == 1 && alive < 2{
                next[(i as usize,j as usize)] = 0;
            }

            if grid[(i as usize,j as usize)] == 1 && alive >= 2 || alive == 3{
                next[(i as usize,j as usize)] = grid[(i as usize,j as usize)];
            }

            if grid[(i as usize,j as usize)] == 1 && alive > 3{
                next[(i as usize,j as usize)] = 0;
            }

            if grid[(i as usize,j as usize)] == 0 && alive == 3{
                next[(i as usize,j as usize)] = 1;
            }
        }
    }

    print_matrix(next, height + 1, width + 1);
}
