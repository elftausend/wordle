use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{cursor::Cursor, word::Word, CELL_HEIGHT, CELL_SPACING, CELL_WIDTH, COLS, ROWS};

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub input_char: char,
    pub background_color: Color,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            input_char: ' ',
            background_color: WHITE,
        }
    }
}

pub fn read_word_file() -> Result<Vec<String>, std::io::Error> {
    std::fs::read_to_string("5char-wordlist-de.txt").map(|read| {
        let mut lines = read
            .lines()
            .map(|line| line.to_string().to_ascii_uppercase())
            .collect::<Vec<String>>();
        lines.sort();
        lines
    })
}

#[derive(Debug, Default)]
pub struct Wordle {
    pub selected_word: Word,
    pub words: Vec<String>,
    pub cursor: Cursor,
    pub fields: [[Field; COLS]; ROWS],
}

impl Wordle {
    pub fn new() -> Result<Wordle, std::io::Error> {
        Ok(Wordle {
            selected_word: Word::new("kanne"),
            words: read_word_file()?,
            cursor: Cursor {
                selected: true,
                cursor_pos: (0, 0),
                rows: ROWS,
                cols: COLS,
            },
            fields: [[Field::default(); COLS]; ROWS],
        })
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

                draw_rectangle(
                    x + thickness * 0.5,
                    y + thickness * 0.5,
                    CELL_WIDTH - thickness,
                    CELL_HEIGHT - thickness,
                    field.background_color,
                );

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
        let word = &self.fields[self.cursor.cursor_pos.0];
        if !is_length_cols(word) {
            return false;
        }
        self.words
            .contains(&fields_to_string(word).to_ascii_uppercase())
    }

    pub fn mark_chars(&mut self) {
        let word_list = &mut self.fields[self.cursor.cursor_pos.0];
        let word = fields_to_string(word_list).to_ascii_uppercase();
        
        let mut zeroed_char_map = word
            .chars()
            .into_iter()
            .map(|char| (char, 0))
            .collect::<HashMap<char, usize>>();

        for (idx, (lhs, rhs)) in word
            .chars()
            .zip(self.selected_word.word.chars())
            .enumerate()
        {
            let background_color = &mut word_list[idx].background_color;
            *background_color = LIGHTGRAY;

            if lhs == rhs {
                *background_color = GREEN;
                continue;
            }
            if !self.selected_word.word.contains(lhs) {
                continue;
            }

            let max_mark_count_for_char = self.selected_word.chars[&lhs];
            let char_count = zeroed_char_map.get_mut(&lhs).unwrap();
                
            if *char_count < max_mark_count_for_char {
                *char_count += 1;
                *background_color = YELLOW;
            }                

        }
    }

    pub fn check_win(&self) -> bool {
        self.selected_word.word
            == fields_to_string(&self.fields[self.cursor.cursor_pos.0]).to_ascii_uppercase()
    }
}

pub fn fields_to_string(word: &[Field]) -> String {
    let mut string_word = String::with_capacity(5);
    for char in word {
        string_word.push(char.input_char)
    }
    string_word
}

pub fn is_length_cols(word: &[Field]) -> bool {
    for char in word {
        if char.input_char == ' ' {
            return false;
        }
    }
    true
}
