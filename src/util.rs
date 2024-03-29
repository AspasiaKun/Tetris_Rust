use ggez::glam::Vec2;

//total grid size in whole game board
pub const GRID_SIZE: (i32,i32) = (10,20);

//the whole window should wilder than the game goard
const WINDOW_GRID: (i32,i32) = (GRID_SIZE.0 + 10, GRID_SIZE.1 + 10);

//cell size per grid
pub const CELL_SIZE_PER_GRID: (i32,i32) = (32,32);
//decided screen size: width,height
pub const SCREEN_SIZE: (f32,f32) = (
    WINDOW_GRID.0 as f32 * CELL_SIZE_PER_GRID.0 as f32,
    WINDOW_GRID.1 as f32 * CELL_SIZE_PER_GRID.1 as f32,
);

//pic scale number
pub const PIC_SCALE_NUMBER: Vec2 = Vec2::new(0.22, 0.22);

//from top-left
pub const GAME_BOARD_START_POSITION_X: f32 = (WINDOW_GRID.0 - GRID_SIZE.0) as f32/2.0 * WINDOW_GRID.0 as f32;
pub const GAME_BOARD_START_POSITION_Y: f32 = (WINDOW_GRID.1 - GRID_SIZE.1) as f32/2.0 * WINDOW_GRID.1 as f32;

pub const SCORE_WORD_START_POSITION: Vec2 = Vec2::new(SCREEN_SIZE.0 / 10.0 * 3.0, SCREEN_SIZE.1 /10.0 * 1.0);

#[derive(Clone)]
pub struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self{
        GridPosition {x,y}
    }


    pub fn get_grid_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_grid_position(&mut self, pos:(i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    pub fn add(&self, pos: (i32, i32)) -> (i32, i32) {
        let x = self.x + pos.0;
        let y = self.y + pos.1;
        (x, y)
    }

    pub fn get_actual_position(&self) -> Vec2 {
        let x = (self.x as f32 + 0.5) * CELL_SIZE_PER_GRID.0 as f32 + GAME_BOARD_START_POSITION_X;
        let y = (self.y as f32 + 0.5) * CELL_SIZE_PER_GRID.1 as f32 + GAME_BOARD_START_POSITION_Y;
        Vec2::new(x, y)
    }

    pub fn move_to_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_to_right(&mut self) {
        self.x += 1;
    }

    pub fn move_to_top(&mut self) {
        self.y -= 1;
    }

    pub fn move_to_bottom(&mut self) {
        self.y += 1;
    }

}