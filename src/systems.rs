use winit::event::*;
use winit::window::Window;

use crate::render::new;

pub struct GameState {
    pub pos: [i32; 2],
    pub board: [[bool; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize],
    dir: Option<Dir>,
    time: f32
}

pub enum Dir {
    Down,
    Right,
    Left,
}

pub const GRID_SIZE: [i32;2] = [16, 32];
pub const DEFAULT_POS: [i32;2] = [GRID_SIZE[0] / 2, GRID_SIZE[1] - 1];
const FRAME_TIME: f32 = 0.01;
const MOVE_TIME: f32 = 1.0;


impl GameState {
    pub fn new() -> Self {
        let mut test = [[false; GRID_SIZE[1] as usize]; GRID_SIZE[0] as usize];

        GameState {
            pos: DEFAULT_POS,
            board: test,
            dir: None,
            time: 0.0
        }
    }

    pub fn update(&mut self) {
        self.time += FRAME_TIME;

        let mut move_time = MOVE_TIME;
        
        if let Some(dir) = &self.dir {
            match dir {
                Dir::Down => move_time /= 3.0,
                Dir::Left => {
                    self.move_cell([-1, 0]);
                    self.dir = None;
                },
                Dir::Right => {
                    self.move_cell([1, 0]);
                    self.dir = None;
                }
            }
        }

        if self.time > move_time {
            self.move_cell([0, -1]);
            self.time = 0.0;
        }

        self.check_rows();
    }

    fn move_cell(&mut self, dir: [i32; 2]) {

        let new_pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]];

        if !self.in_bounds(new_pos) { return }

        if self.cell_exists(new_pos) {
            self.pos = DEFAULT_POS;
            return
        }

        self.board[self.pos[0] as usize][self.pos[1]as usize] = false;
        self.pos = new_pos;
        self.board[self.pos[0]as usize][self.pos[1]as usize] = true;
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
