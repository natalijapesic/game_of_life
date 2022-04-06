use std::io::Write;
use std::io;
use std::ops::{IndexMut, Index};
use rand::Rng;

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

fn main(){
    

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

    let mut tmp:Vec<Vec<u32>> = vec![];

    for i in 0..dimensions[0]{
        tmp.push(vec![]);

        for j in 0..dimensions[1]{
            tmp.last_mut().unwrap().push(rand::thread_rng().gen_range(0..2));
        }
    }

    let grid:Matrix<u32> = Matrix {
        data: tmp
    };

    print_matrix(grid.clone(), dimensions[0], dimensions[1]);

    next_generation(grid, dimensions[0], dimensions[1]);
}

fn print_matrix(matrix: Matrix<u32>, n:u32, m:u32){

    for i in 0..n as usize{

        for j in 0..m as usize{
            print!("{}", matrix[(i,j)]);
        }
        println!("\n")
    }
}

fn next_generation(grid: Matrix<u32>, mut n:u32, mut m:u32){

    let mut tmp:Vec<Vec<u32>> = vec![];

    let mut count = 0;
    while count < n {
        tmp.push(vec![0; m as usize]);
        count +=1;
    }

    let mut next:Matrix<u32> = Matrix {
        data: tmp
    };

    n = n-1;
    m = m-1;
    for i in 1..n as isize{

        for j in 1..m as isize{
            
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

    print_matrix(next, n + 1, m + 1);
}
