use rand::distributions::Uniform;
use rand::Rng;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct World {
    height: i32,
    width: i32,
    grid: Vec<Vec<State>>,
}

impl World {

    pub fn default(h: i32, w: i32) -> Self {
        Self {
            height: h,
            width: w,
            grid: vec![],
        }
    }

    pub fn new(h: i32, w: i32) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let range: Uniform<i32> = Uniform::new(0, 2);
        let mut data: Vec<Vec<State>> = vec![];

        for _ in 0..h {
            data.push(
                (0..w)
                    .map(|_| {
                        let num = rng.sample(&range);
                        if num == 1 {
                            State::Alive
                        } else {
                            State::Dead
                        }
                    })
                    .collect(),
            );
        }

        Self {
            height: h,
            width: w,
            grid: data,
        }
    }

    pub fn next_generation(&mut self) {
        let mut next = self.clone();
    
        //DA LI JE MOGUCE DA BEZ KLONIRANJA U SVAKOJ ITERACIJI SE OVA FJA POZIVA a ne inline da se izvrsava?
        for i in 0..next.height {
            for j in 0..next.width {
                let live = {
                    let operations: [[i32; 2]; 8] = [
                        [0, 1],
                        [0, -1],
                        [1, 0],
                        [-1, 0],
                        [1, 1],
                        [1, -1],
                        [-1, 1],
                        [-1, -1],
                    ];
                    let mut lives = 0;

                    let operation_iter = operations.iter();
                    for op in operation_iter {

                        if let Some(row) = self.grid.get((i + op[0]) as usize) {
                            if let Some(cell) = row.get((j + op[1]) as usize){
                                if let State::Alive = cell{
                                    lives += 1;
                                }
                            }
                        }
                    }

                    lives
                };
    
                let current_state = self[(i as usize, j as usize)];
    
                next[(i as usize, j as usize)] = match (current_state, live) {
                    (State::Alive, 2 | 3) => State::Alive,
                    (State::Alive, _) => State::Dead,
                    (State::Dead, 3) => State::Alive,
                    (_, _) => current_state,
                }
            }
        }

        self.grid = next.grid.clone();
    }


    pub fn draw(&self) {
        for line in &self.grid{
            println!(
                "{}",
                line.iter()
                    .map(|l| {
                        match l {
                            State::Alive => "A",
                            State::Dead => ".",
                        }
                    })
                    .collect::<String>()
            )
        }
    }
}

impl Index<(usize, usize)> for World {
    type Output = State;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}
impl IndexMut<(usize, usize)> for World {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}



fn main() {
    let mut game: World = World::new(5, 5);
    
    let mut count = 0;
    'counting_up: loop {

    println!("kdjcsn");
        //clear
    game.draw();
        //sleep
    game.next_generation();
    
        count += 1;
        if count == 2{
            break 'counting_up;
        }
    }
}
