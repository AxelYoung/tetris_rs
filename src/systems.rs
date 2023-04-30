use winit::event::*;
use rand::Rng;

pub struct GameState<'a> {
    pub board: [[bool; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize],
    pub tetrimino: &'a [Cell],
    pub pos: [i32; 2],
    dir: Option<Dir>,
    time: f32,
    paused: bool,
    score: u8
}

pub enum Dir {
    Down,
    Right,
    Left,
}

pub const GRID_SIZE: [i32;2] = [10, 20];
pub const DEFAULT_POS: [i32;2] = [GRID_SIZE[0] / 2, GRID_SIZE[1] - 1];
const FRAME_TIME: f32 = 0.005;
const MOVE_TIME: f32 = 1.0;

const SQUARE_TETRIMINO: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: 0, y: -1},
    Cell {x: -1, y: 0},
    Cell {x: -1, y: -1},
];

const T_TETRIMINO: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: -1, y: -1},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: -1},
];

const T_TETRIMINO_U: &[Cell] = &[
    Cell {x: 0, y: -2},
    Cell {x: -1, y: -1},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: -1},
];

const T_TETRIMINO_L: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: -1, y: -1},
    Cell {x: 0, y: -1},
    Cell {x: 0, y: -2},
];

const T_TETRIMINO_R: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: -1},
    Cell {x: 0, y: -2},
];

const LINE_TETRIMINO: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: 0, y: 0},
    Cell {x: 2, y: 0},
    Cell {x: -1, y: 0},
];

const LINE_TETRIMINO_U: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: 0, y: -1},
    Cell {x: 0, y: -2},
    Cell {x: 0, y: -3},
];

const L_TETRIMINO: &[Cell] = &[
    Cell {x: -1, y: 0},
    Cell {x: -1, y: -1},
    Cell {x: 0, y: -1},
    Cell {x: 1, y: -1},
];

const L_TETRIMINO_R: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: 0, y: -1},
    Cell {x: 0, y: -2},
    Cell {x: 1, y: 0},
];

const L_TETRIMINO_U: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: 0},
    Cell {x: -1, y: 0},
];

const L_TETRIMINO_L: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: -2},
    Cell {x: 1, y: -2},
];

const RL_TETRIMINO: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: -1, y: -1},
    Cell {x: 0, y: -1},
    Cell {x: 1, y: -1},
];

const RL_TETRIMINO_R: &[Cell] = &[
    Cell {x: 0, y: 0},
    Cell {x: 0, y: -1},
    Cell {x: 0, y: -2},
    Cell {x: 1, y: -2},
];

const RL_TETRIMINO_U: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: -1, y: -1},
    Cell {x: 0, y: 0},
    Cell {x: -1, y: 0},
];

const RL_TETRIMINO_L: &[Cell] = &[
    Cell {x: 1, y: 0},
    Cell {x: 1, y: -1},
    Cell {x: 0, y: 0},
    Cell {x: 1, y: -2},
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub x: i32,
    pub y: i32
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        let mut test = [[false; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        GameState {
            board: test,
            dir: None,
            time: 0.0,
            tetrimino: SQUARE_TETRIMINO,
            pos: DEFAULT_POS,
            paused: true,
            score: 0
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
        for cell in self.tetrimino {
            let new_pos = [cell.x + self.pos[0] + dir[0], cell.y + self.pos[1] + dir[1]];

            if !self.in_bounds(new_pos) { return }
    
            if self.cell_exists(new_pos) {
                for cell in self.tetrimino {
                    self.board[(cell.x + self.pos[0]) as usize][(cell.y + self.pos[1]) as usize] = true;
                }
                self.random_tetrimino();
                self.pos = DEFAULT_POS;
                return
            }
        }

        self.pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]];
    }

    fn random_tetrimino(&mut self) {
        let num = rand::thread_rng().gen_range(0..5);
        match num {
            0 => {self.tetrimino = SQUARE_TETRIMINO},
            1 => {self.tetrimino = T_TETRIMINO},
            2 => {self.tetrimino = L_TETRIMINO},
            3 => {self.tetrimino = LINE_TETRIMINO},
            4 => {self.tetrimino = RL_TETRIMINO},
            _ => {}
        }
    }

    fn rotate_tetrimino(&mut self){
        match self.tetrimino {
            SQUARE_TETRIMINO => {},
            LINE_TETRIMINO => {self.tetrimino = LINE_TETRIMINO_U},
            LINE_TETRIMINO_U => {self.tetrimino = LINE_TETRIMINO},
            T_TETRIMINO => {self.tetrimino = T_TETRIMINO_R},
            T_TETRIMINO_R => {self.tetrimino = T_TETRIMINO_U},
            T_TETRIMINO_U => {self.tetrimino = T_TETRIMINO_L},
            T_TETRIMINO_L => {self.tetrimino = T_TETRIMINO},
            L_TETRIMINO => {self.tetrimino = L_TETRIMINO_R},
            L_TETRIMINO_R => {self.tetrimino = L_TETRIMINO_U},
            L_TETRIMINO_U => {self.tetrimino = L_TETRIMINO_L},
            L_TETRIMINO_L => {self.tetrimino = L_TETRIMINO},
            RL_TETRIMINO => {self.tetrimino = RL_TETRIMINO_R},
            RL_TETRIMINO_R => {self.tetrimino = RL_TETRIMINO_U},
            RL_TETRIMINO_U => {self.tetrimino = RL_TETRIMINO_L},
            RL_TETRIMINO_L => {self.tetrimino = RL_TETRIMINO},
            _ => {}
        }
    }

    fn in_bounds(&mut self, pos: [i32; 2]) -> bool {
        if pos[0] >= 0 && pos[0] <= GRID_SIZE[0] - 1 && pos[1] <= GRID_SIZE[1] - 1 {
            true
        } else {
            false
        }
    }

    fn cell_exists(&mut self, pos: [i32; 2]) -> bool {
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
