use macroquad::{
    prelude::{get_char_pressed, is_key_pressed, KeyCode, BLACK, WHITE},
    shapes::draw_rectangle_lines,
    text::draw_text,
    window::{clear_background, next_frame},
};

const ROWS: usize = 6;
const COLS: usize = 5;
const CELL_WIDTH: f32 = 40.;
const CELL_HEIGHT: f32 = 40.;
const CELL_SPACING: f32 = 1.1;

#[derive(Debug, Clone, Copy)]
struct Field {
    input_char: char,
}

impl Default for Field {
    fn default() -> Self {
        Self { input_char: ' ' }
    }
}

#[derive(Debug, Default)]
pub struct Cursor {
    cursor_pos: (usize, usize),
    cols: usize,
    rows: usize,
}

impl Cursor {
    pub fn move_right(&mut self) -> bool {
        if self.cursor_pos.1 >= self.cols - 1 {
            return false;
        }
        self.cursor_pos.1 += 1;
        true
    }

    pub fn move_left(&mut self) -> bool {
        if self.cursor_pos.1 <= 0 {
            return false;
        }
        self.cursor_pos.1 -= 1;
        true
    }

    pub fn move_down(&mut self) -> bool {
        if self.cursor_pos.0 >= self.rows - 1 {
            return false;
        }
        self.cursor_pos.0 += 1;
        true
    }

    pub fn move_up(&mut self) -> bool {
        if self.cursor_pos.0 <= 0 {
            return false;
        }
        self.cursor_pos.0 -= 1;
        true
    }

    pub fn is_cursor_on_pos(&self, pos: (usize, usize)) -> bool {
        pos.0 == self.cursor_pos.0 && pos.1 == self.cursor_pos.1
    }
}

#[derive(Debug, Default)]
pub struct Wordle {
    cursor: Cursor,
    fields: [[Field; COLS]; ROWS],
}

impl Wordle {
    pub fn new() -> Wordle {
        Wordle {
            cursor: Cursor {
                cursor_pos: (0, 0),
                rows: ROWS,
                cols: COLS,
            },
            fields: [[Field::default(); COLS]; ROWS],
        }
    }

    pub fn draw(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let x = 10. + CELL_WIDTH * col as f32 * CELL_SPACING;
                let y = 10. + CELL_HEIGHT * row as f32 * CELL_SPACING;
                let thickness = if self.cursor.is_cursor_on_pos((row, col)) {
                    10.5
                } else {
                    2.5
                };
                let field = self.fields[row][col];

                draw_text(
                    &field.input_char.to_string(),
                    x + (CELL_WIDTH / 4.),
                    y + (CELL_HEIGHT / 1.3),
                    45.,
                    BLACK,
                );
                draw_rectangle_lines(x, y, CELL_WIDTH, CELL_HEIGHT, thickness, BLACK);
            }
        }
    }

    pub fn update_field(&mut self, char: char) {
        self.fields[self.cursor.cursor_pos.0][self.cursor.cursor_pos.1].input_char = char;
    }
}

#[macroquad::main("Wordle")]
async fn main() {
    let mut wordle = Wordle::new();

    loop {
        clear_background(WHITE);

        wordle.draw();

        if is_key_pressed(KeyCode::Left) {
            wordle.cursor.move_left();
        } else if is_key_pressed(KeyCode::Right) {
            wordle.cursor.move_right();
        }

        if is_key_pressed(KeyCode::Down) {
            wordle.cursor.move_down();
        } else if is_key_pressed(KeyCode::Up) {
            wordle.cursor.move_up();
        } else if is_key_pressed(KeyCode::Backspace) {
            wordle.update_field(' ');
            wordle.cursor.move_left();
        } else {
            if let Some(pressed_char) = get_char_pressed() {
                if ('a'..'z').contains(&pressed_char) || ('A'..'Z').contains(&pressed_char) {
                    wordle.update_field(pressed_char.to_ascii_uppercase());
                    wordle.cursor.move_right();
                }
            }
        }

        next_frame().await;
    }
}
