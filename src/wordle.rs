use macroquad::prelude::*;

use crate::{cursor::Cursor, ROWS, COLS, CELL_WIDTH, CELL_SPACING, CELL_HEIGHT};


#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub input_char: char,
}

impl Default for Field {
    fn default() -> Self {
        Self { input_char: ' ' }
    }
}

#[derive(Debug, Default)]
pub struct Wordle {
    pub cursor: Cursor,
    pub fields: [[Field; COLS]; ROWS],
}

impl Wordle {
    pub fn new() -> Wordle {
        Wordle {
            cursor: Cursor {
                selected: true,
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
                let thickness = if self.cursor.is_cursor_on_pos((row, col)) && self.cursor.selected
                {
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

        if self.cursor.cursor_pos.1 == COLS - 1 {
            self.cursor.selected = false;
        }
    }

    pub fn field(&self) -> Field {
        self.fields[self.cursor.cursor_pos.0][self.cursor.cursor_pos.1]
    }

    pub fn field_on_col(&self, col: usize) -> Option<Field> {
        self.fields[self.cursor.cursor_pos.0].get(col).copied()
    }

    pub fn word_in_list(&self) -> bool {
        is_length_cols(&self.fields[self.cursor.cursor_pos.0])
    }
}

pub fn is_length_cols(word: &[Field]) -> bool {
    for char in word {
        if char.input_char == ' ' {
            return false;
        }
    }
    true
}