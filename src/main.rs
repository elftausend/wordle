mod wordle;
mod cursor;
use wordle::Wordle;

use macroquad::prelude::*;

const ROWS: usize = 6;
const COLS: usize = 5;
const CELL_WIDTH: f32 = 40.;
const CELL_HEIGHT: f32 = 40.;
const CELL_SPACING: f32 = 1.1;


#[macroquad::main("Wordle")]
async fn main() {
    let mut wordle = Wordle::new();

    loop {
        clear_background(WHITE);

        wordle.draw();
        if is_key_pressed(KeyCode::Enter) {
            if wordle.word_in_list() {
                wordle.cursor.move_down();
                wordle.cursor.col_reset();
                wordle.cursor.selected = true;
            }
        } else if is_key_pressed(KeyCode::Left) {
            wordle.cursor.move_left();
        } else if is_key_pressed(KeyCode::Right) {
            wordle.cursor.move_right();
        } else if is_key_pressed(KeyCode::Backspace) {
            if wordle.cursor.selected && wordle.field().input_char == ' ' {
                wordle.cursor.move_left();
            }

            wordle.update_field(' ');
            wordle.cursor.selected = true;
        } else {
            if let Some(pressed_char) = get_char_pressed() {
                if (('a'..'z').contains(&pressed_char) || ('A'..'Z').contains(&pressed_char))
                    && wordle.cursor.selected
                {
                    wordle.update_field(pressed_char.to_ascii_uppercase());
                    wordle.cursor.move_right();
                    //wordle.cursor.unselected_del = false;
                }
            }
        }

        next_frame().await;
    }
}
