use std::fmt::{self};
use std::thread;
use std::time::Duration;

use rand::distributions::Uniform;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Dead,
    Alive,
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

    pub fn generate_grid(h: i32, w: i32, data: Vec<Vec<i32>>) -> Self {
        Self {
            height: h,
            width: w,
            grid: Wrapper::from(data).into(),
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

    pub fn next_generation(&mut self) -> Vec<Vec<State>> {
        let mut next = self.clone();

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
                            if let Some(cell) = row.get((j + op[1]) as usize) {
                                if let State::Alive = cell {
                                    lives += 1;
                                }
                            }
                        }
                    }

                    lives
                };

                let current_state = self.grid[i as usize][j as usize];

                next.grid[i as usize][j as usize] = match (current_state, live) {
                    (State::Alive, 2 | 3) => State::Alive,
                    (State::Alive, _) => State::Dead,
                    (State::Dead, 3) => State::Alive,
                    (_, _) => current_state,
                }
            }
        }

        self.grid = next.grid.clone();
        self.grid.clone()
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.grid {
            writeln!(
                f,
                "{}",
                line.iter()
                    .map(|l| {
                        match l {
                            State::Alive => " A ",
                            State::Dead => " . ",
                        }
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl From<i32> for State {
    fn from(item: i32) -> State {
        if item == 1 {
            State::Alive
        } else {
            State::Dead
        }
    }
}

struct Wrapper(Vec<Vec<State>>);

impl Into<Vec<Vec<State>>> for Wrapper {
    fn into(self) -> Vec<Vec<State>> {
        self.0
    }
}

impl From<Vec<Vec<i32>>> for Wrapper {
    fn from(grid: Vec<Vec<i32>>) -> Wrapper {
        let mut converted_grid: Vec<Vec<State>> = vec![];

        for item in grid.iter() {
            let row = item
                .iter()
                .map(|cell| State::from(*cell))
                .collect::<Vec<State>>();
            converted_grid.push(row);
        }

        Wrapper(converted_grid)
    }
}

fn main() {
    let mut game: World = World::new(5, 5);

    let mut count = 0;
    'counting_up: loop {
        println!("{:}", game);
        thread::sleep(Duration::from_secs(2));
        game.next_generation();

        count += 1;
        if count == 10 {
            break 'counting_up;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{State, World, Wrapper};

    #[test]
    fn test_generation() {
        let data = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
        ];

        let expected_data = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];
        let mut world = World::generate_grid(5, 5, data.clone());
        world.next_generation();

        let converted_expected: Vec<Vec<State>> = Wrapper::from(expected_data.clone()).into();

        assert_eq!(converted_expected, world.grid);
    }
}
