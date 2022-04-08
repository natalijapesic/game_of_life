use rand::distributions::Uniform;
use rand::Rng;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Grid {
    height: Option<i32>,
    width: Option<i32>,
    board: Vec<Vec<State>>,
}

impl Grid {
    pub fn default(h: i32, w: i32) -> Self {
        Self {
            height: Some(h),
            width: Some(w),
            board: vec![],
        }
    }

    pub fn new(h: i32, w: i32) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let range: Uniform<i32> = Uniform::new(0, w);
        let mut data: Vec<Vec<State>> = vec![];

        for _ in 0..h {
            data.push(
                (0..2)
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
            height: Some(h),
            width: Some(w),
            board: data,
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = State;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.board[index.0][index.1]
    }
}
impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.board[index.0][index.1]
    }
}

pub fn count_neighbours(first_generation: &Grid, row: i32, column: i32) -> i32 {
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
        let new_row: usize = (row + op[0]) as usize;
        let new_col: usize = (column + op[1]) as usize;

        if let State::Alive = first_generation[(new_row, new_col)] {
            lives += 1;
        }
    }

    lives
}

pub fn next_generation(first_generation: Grid) -> Grid {
    let mut next = first_generation.clone();

    for i in 0..next.height.unwrap() {
        for j in 0..next.width.unwrap() {
            let live = count_neighbours(&first_generation, i, j);

            let current_state = first_generation[(i as usize, j as usize)];

            next[(i as usize, j as usize)] = match (current_state, live) {
                (State::Alive, 2 | 3) => State::Alive,
                (State::Alive, _) => State::Dead,
                (State::Dead, 3) => State::Alive,
                (_, _) => current_state,
            }
        }
    }

    next
}

pub fn draw(board: Vec<Vec<State>>) {
    for line in board {
        println!(
            "{}",
            line.iter()
                .map(|l| {
                    match l {
                        State::Alive => "O",
                        State::Dead => ".",
                    }
                })
                .collect::<String>()
        )
    }
}

fn main() {
    let mut game: Grid = Grid::new(20, 20);
    loop {
        //clear
        //draw(game.board);
        //sleep
        //game = next_generation(game);
    }
}
