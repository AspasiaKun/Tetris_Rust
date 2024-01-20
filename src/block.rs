use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;

//方块的生成与组合逻辑
use rand::seq::SliceRandom;
use ggez::graphics::{self,DrawParam,Canvas};
use ggez::{Context,GameError};
use ggez::glam::Vec2;

use crate::util::{self, GridPosition};


pub struct Block {
    image_blue_block: graphics::Image,
    image_green_block: graphics::Image,
    image_purple_block: graphics::Image,
    image_red_block: graphics::Image,
    image_yellow_block: graphics::Image,
    position: GridPosition,
}

impl Block {
    pub fn new(ctx: &mut Context) -> Result<Self, GameError>{
        let blue = graphics::Image::from_path(ctx,"/assets/pic/blue_block.png")?;
        let green = graphics::Image::from_path(ctx, "/assets/pic/green_block.png")?;
        let purple = graphics::Image::from_path(ctx, "/assets/pic/purple_block.png")?;
        let red = graphics::Image::from_path(ctx, "/assets/pic/red_block.png")?;
        let yellow = graphics::Image::from_path(ctx, "/assets/pic/yellow_block.png")?;

        let position = GridPosition::new(0, 0);
        Ok(Self{image_blue_block: blue,
            image_green_block: green,
            image_purple_block: purple,
            image_red_block: red,
            image_yellow_block: yellow,
            position: position,
        })
    }

    pub fn set_block_position(&self, pos:(i32,i32)) {
        &self.position.set_grid_position(pos);
    }

    pub fn get_rand_pic(&self) -> &graphics::Image {
        let mut rng = rand::thread_rng();
        let images = [
            &self.image_blue_block,
            &self.image_green_block,
            &self.image_purple_block,
            &self.image_red_block,
            &self.image_yellow_block,
        ];
        let random_image = *images.choose(&mut rng).unwrap();
        random_image
    }

    pub fn move_to_left(&mut self) -> Result<(),GameError> {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.x <= 0.0 {
            return Ok(()) ;
        }
        Ok(self.position.move_to_left())
    }

    pub fn move_to_right(&mut self) -> Result<(),GameError>{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.x >= 10.0 {
            return Ok(());
        }
        Ok(self.position.move_to_right())
    }

    pub fn move_to_top(&mut self) -> Result<(),GameError>{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.y <= 0.0 {
            return Ok(());
        }
        Ok(self.position.move_to_top())
    }

    pub fn move_to_bottom(&mut self) -> Result<(),GameError>{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.y >= 20.0 {
            return Ok(());
        }
        Ok(self.position.move_to_bottom())
    }

    pub fn draw(&mut self, canvas: &mut Canvas, pic: &graphics::Image) {
        canvas.draw(pic, DrawParam::new()
        .dest(self.position.get_actual_position())
        .scale(util::PIC_SCALE_NUMBER));
    }

}


struct BlockGroup {
    block1: Block,
    block2: Block,
    block3: Block,
    block4: Block,
    image: graphics::Image,
    position: GridPosition,
}

impl BlockGroup {
    fn random_group_generation(ctx: &mut Context) -> Self{
        let block1 = Block::new(ctx).unwrap();
        let block2 = Block::new(ctx).unwrap();
        let block3 = Block::new(ctx).unwrap();
        let block4 = Block::new(ctx).unwrap();

        

    }


    fn draw(&mut self, canvas: &mut Canvas) {

    }
}