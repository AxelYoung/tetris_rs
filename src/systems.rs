use winit::event::*;
use rand::Rng;

pub const GRID_SIZE: [i32;2] = [10, 20];
pub const DEFAULT_POS: [i32;2] = [(GRID_SIZE[0] / 2) - 1, GRID_SIZE[1] - 1];
const TICKS_PER_SECOND: f32 = 8.0;
const TICK_TIME: f32 = 1.0 / TICKS_PER_SECOND;

pub struct GameState {
    pub board: [[bool; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize],
    pub pos: [i32; 2],
    pub tetrimino: Vec<Vec<bool>>,
    dir: Option<Dir>,
    previous_time: instant::Instant,
    tick: f32,
    score: u8
}

type Tetrimino<'a> = &'a[&'a[bool]];

pub enum Dir {
    Down,
    Right,
    Left,
}

const TETRIMINOS : [Tetrimino; 7] = [
    O_TETRIMINO,
    T_TETRIMINO,
    S_TETRIMINO,
    Z_TETRIMINO,
    J_TETRIMINO,
    L_TETRIMINO,
    I_TETRIMINO
];

const O_TETRIMINO : Tetrimino = &[
    &[true, true],
    &[true, true]
];

const T_TETRIMINO : Tetrimino = &[
    &[false, true, false],
    &[true, true, true]
];

const S_TETRIMINO : Tetrimino = &[
    &[false, true, true],
    &[true, true, false]
];

const Z_TETRIMINO : Tetrimino = &[
    &[true, true, false],
    &[false, true, true]
];

const J_TETRIMINO : Tetrimino = &[
    &[true, false, false],
    &[true, true, true]
];

const L_TETRIMINO : Tetrimino = &[
    &[false, false, true],
    &[true, true, true]
];

const I_TETRIMINO : Tetrimino = &[
    &[true, true, true, true]
];

impl GameState {
    pub fn new() -> Self {
        let test = [[false; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        GameState {
            board: test,
            dir: None,
            previous_time: instant::Instant::now(),
            tick: 0.0,
            pos: DEFAULT_POS,
            score: 0,
            tetrimino: GameState::random_tetrimino()
        }
    }

    pub fn update(&mut self) {
        let current_time = instant::Instant::now();
        let elapsed_time = current_time.duration_since(self.previous_time).as_secs_f32();
        self.previous_time = current_time;

        self.tick += elapsed_time;

        if let Some(dir) = &self.dir {
            match dir {
                Dir::Down => {self.tick += elapsed_time;},
                Dir::Left => {
                    self.move_tetrimino([-1, 0]);
                    self.dir = None;
                },
                Dir::Right => {
                    self.move_tetrimino([1, 0]);
                    self.dir = None;
                }
            }
        }

        if self.tick > TICK_TIME {
            self.move_tetrimino([0, -1]);
            self.tick -= TICK_TIME;
        }

        self.check_rows();
    }

    fn move_tetrimino(&mut self, dir: [i32; 2]) {
        for (y, row) in self.tetrimino.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if !val { continue }

                let new_pos = [x as i32 + self.pos[0] + dir[0], y as i32 + self.pos[1] + dir[1]];

                if !self.in_bounds(new_pos) { return }
        
                if dir[1] == -1 { 
                    if self.cell_exists(new_pos) {
                        for (y2, row2) in self.tetrimino.iter().enumerate() {
                            for (x2, val2) in row2.iter().enumerate() {
                                if !self.in_bounds([x2 as i32+ self.pos[0], y2 as i32 + self.pos[1]]) { 
                                    self.reset_game();
                                    return
                                }
                                if *val2 { self.board[x2 + self.pos[0] as usize][y2 + self.pos[1] as usize] = true; }
                            }
                        }
                        self.tetrimino = GameState::random_tetrimino();
                        self.pos = DEFAULT_POS;
                        for (y2, row2) in self.tetrimino.iter().enumerate() {
                            for (x2, _) in row2.iter().enumerate() {
                                let pos = [x2 as i32 + self.pos[0], y2 as i32+ self.pos[1]];
                                if self.in_bounds(pos) && self.cell_exists(pos) {
                                    self.reset_game();
                                    return
                                }
                            }
                        } 
                        return
                    }
                 } else {
                    if self.cell_exists(new_pos) {
                        return
                    }
                 }

                
            }
        }

        self.pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]];
    }

    fn reset_game(&mut self) {
        self.board = [[false; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];
        println!("Your score was: {}", self.score);
        self.score = 0;
    }

    fn random_tetrimino() -> Vec<Vec<bool>> {
        TETRIMINOS[rand::thread_rng().gen_range(0..7)].iter().map(|row| row.to_vec()).collect::<Vec<_>>()
    }

    fn rotate_tetrimino(&mut self){
        let mut vec_tetrimino : Vec<Vec<bool>> = vec![];

        for col in 0..self.tetrimino[0].len() {
            let mut new_row : Vec<bool> = vec![];
            for (row, _) in self.tetrimino.iter().rev().enumerate() {
                new_row.push(self.tetrimino[row][col]);
                let pos = [row as i32 + self.pos[0], col as i32 + self.pos[1]];
                if !self.in_bounds(pos) || self.cell_exists(pos)  { return }
            }
            vec_tetrimino.insert(0, new_row);
        }

        self.tetrimino = vec_tetrimino;
    }

    fn in_bounds(&self, pos: [i32; 2]) -> bool {
        if pos[0] >= 0 && pos[0] <= GRID_SIZE[0] - 1 && pos[1] <= GRID_SIZE[1] - 1 {
            true
        } else {
            false
        }
    }

    fn cell_exists(&self, pos: [i32; 2]) -> bool {
        if pos[1] == -1 { return true }

        let x = pos[0] as usize;
        let y = pos[1] as usize;

        if self.board[x][y] == true {
            return true
        } else {
            return false
        }
    }

    fn check_rows(&mut self) {
        let mut full_rows : Vec<i32> = vec![];

        for y in 0..GRID_SIZE[1] {
            for x in 0..GRID_SIZE[0] {
                if !self.board[x as usize][y as usize] {
                    break;
                }
                if x == GRID_SIZE[0] - 1 {
                    full_rows.push(y);
                }
            }        
        }

        full_rows.reverse();

        for y in full_rows {
            for x in 0..GRID_SIZE[0] {
                self.board[x as usize][y as usize] = false;
            }
            for y_above in (y + 1)..GRID_SIZE[1] {
                for x in 0..GRID_SIZE[0] {
                    if self.board[x as usize][y_above as usize] {
                        self.board[x as usize][y_above as usize] = false;
                        self.board[x as usize][(y_above - 1) as usize] = true;
                    }
                }
            }
            self.score += 1;
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    },
                ..
            } => {
                self.dir = Some(Dir::Down);
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    },
                ..
            } => {
                self.rotate_tetrimino();
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    },
                ..
            } => {
                self.dir = Some(Dir::Left);
                return true;
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    },
                ..
            } => {
                self.dir = Some(Dir::Right);
                return true;
            }
            _ => {
                self.dir = None;
            }
        }
        false
    }
}
