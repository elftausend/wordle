mod cursor;
mod key_actions;
mod word;
mod wordle;
use wordle::{Wordle, State};

use macroquad::prelude::*;

const ROWS: usize = 6;
const COLS: usize = 5;
const CELL_WIDTH: f32 = 40.;
const CELL_HEIGHT: f32 = 40.;
const CELL_SPACING: f32 = 1.1;

#[macroquad::main("Wordle")]
async fn main() -> Result<(), std::io::Error> {
    let mut wordle = Wordle::new()?;

    loop {
        clear_background(WHITE);

        wordle.draw_grid();
        
        match wordle.game_state {
            State::Won => wordle.draw_won(),
            State::Lost => wordle.draw_lost(),
            State::Playing => wordle.game_logic(),
    }
        
        next_frame().await;
    }
}
