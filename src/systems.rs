use winit::event::*;
use rand::Rng;

pub const GRID_SIZE: [i32;2] = [10, 20];
pub const DEFAULT_POS: [i32;2] = [GRID_SIZE[0] / 2, GRID_SIZE[1] - 1];
const FRAME_TIME: f32 = 0.005;
const MOVE_TIME: f32 = 1.0;

pub struct GameState {
    pub board: [[bool; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize],
    pub pos: [i32; 2],
    pub tetrimino: Vec<Vec<bool>>,
    dir: Option<Dir>,
    time: f32,
    paused: bool,
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
        let mut test = [[false; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        GameState {
            board: test,
            dir: None,
            time: 0.0,
            pos: DEFAULT_POS,
            paused: true,
            score: 0,
            tetrimino: GameState::random_tetrimino()
        }
    }

    pub fn update(&mut self) {
        if self.paused {return}

        self.time += FRAME_TIME;

        let mut move_time = MOVE_TIME;
        
        if let Some(dir) = &self.dir {
            match dir {
                Dir::Down => move_time /= 3.0,
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

        if self.time > move_time {
            self.move_tetrimino([0, -1]);
            self.time = 0.0;
        }

        self.check_rows();
    }

    fn move_tetrimino(&mut self, dir: [i32; 2]) {
        for (y, row) in self.tetrimino.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if !val { continue }

                let new_pos = [x as i32 + self.pos[0] + dir[0], y as i32 + self.pos[1] + dir[1]];

                if !self.in_bounds(new_pos) { return }
        
                if self.cell_exists(new_pos) {
                    for (y2, row2) in self.tetrimino.iter().enumerate() {
                        for (x2, val2) in row2.iter().enumerate() {
                            if *val2 { self.board[x2 + self.pos[0] as usize][y2 + self.pos[1] as usize] = true; }
                        }
                    }
                    self.tetrimino = GameState::random_tetrimino();
                    self.pos = DEFAULT_POS;
                    return
                }
            }
        }

        self.pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]];
    }

    fn random_tetrimino() -> Vec<Vec<bool>> {
        TETRIMINOS[rand::thread_rng().gen_range(0..7)].iter().map(|row| row.to_vec()).collect::<Vec<_>>()
    }

    fn rotate_tetrimino(&mut self){
        let mut vec_tetrimino : Vec<Vec<bool>> = vec![];

        for col in 0..self.tetrimino[0].len() {
            let mut new_row : Vec<bool> = vec![];
            for (row, val) in self.tetrimino.iter().rev().enumerate() {
                new_row.push(self.tetrimino[row][col]);
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
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.paused = !self.paused;
                return true;
            }
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
