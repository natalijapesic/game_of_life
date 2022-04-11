use std::fmt::{self, Display};
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
            grid: World::convert_into_enum(data, h as usize, w as usize),
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

    fn compare_vectors(a: Vec<State>, b: Vec<State>) -> usize {
        a.iter().zip(&b).filter(|&(a, b)| a == b).count()
    }

    fn compare_generations(&mut self, expected_data: Vec<Vec<i32>>) {
        let generated_next = self.next_generation();
        let expected_next: Vec<Vec<State>> = World::convert_into_enum(expected_data, 5, 5);

        for i in 0..self.height {
            let same_cells = World::compare_vectors(
                generated_next[i as usize].clone(),
                expected_next[i as usize].clone(),
            );

            if same_cells < 5 {
                panic!("Row {} has only {} same cells", i, same_cells)
            }
        }
    }

    // for i in 0..h {
    //     copy_data[i as usize].push(
    //         data[i as usize]
    //             .iter()
    //             .map(|element| {
    //                 if let 1 = element {
    //                     State::Alive
    //                 } else {
    //                     State::Dead
    //                 }
    //             })
    //             .collect(),
    //     );
    // }

    pub fn convert_into_enum(data: Vec<Vec<i32>>, h: usize, w: usize) -> Vec<Vec<State>> {
        let mut converted_grid: Vec<Vec<State>> = vec![];

        for i in 0..h {
            let mut row: Vec<State> = vec![];

            // row.push(if let 1 = data[i].iter().unzip() {
            //     State::Alive
            // } else {
            //     State::Dead
            // });

            for j in 0..w {
                row.push(if let 1 = data[i][j] {
                    State::Alive
                } else {
                    State::Dead
                });
            }

            converted_grid.push(row);
        }

        converted_grid
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

    pub fn draw(&self) {
        for line in &self.grid {
            println!(
                "{}",
                line.iter()
                    .map(|l| {
                        match l {
                            State::Alive => " A ",
                            State::Dead => " . ",
                        }
                    })
                    .collect::<String>()
            )
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 10)
    }
}

fn main() {
    let mut game: World = World::new(5, 5);

    let mut count = 0;
    'counting_up: loop {
        game.draw();
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
    use crate::World;

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
        let mut world = World::generate_grid(5, 5, data);

        world.compare_generations(expected_data);
    }
}
