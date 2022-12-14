use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{
    cursor::Cursor, key_actions::enter, word::Word, CELL_HEIGHT, CELL_SPACING, CELL_WIDTH, COLS,
    ROWS,
};

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

#[derive(Debug)]
pub enum State {
    Won,
    Lost,
    Playing,
}

impl Default for State {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Debug, Default)]
pub struct Wordle {
    pub selected_word: Word,
    pub words: Vec<String>,
    pub cursor: Cursor,
    pub fields: [[Field; COLS]; ROWS],
    pub game_state: State,
}

impl Wordle {
    pub fn new() -> Result<Wordle, std::io::Error> {
        let words = read_word_file()?;
        Ok(Wordle {
            selected_word: Word::random_from_list(&words),
            words,
            cursor: Cursor {
                selected: true,
                cursor_pos: (0, 0),
                rows: ROWS,
                cols: COLS,
            },
            fields: [[Field::default(); COLS]; ROWS],
            game_state: State::Playing,
        })
    }

    pub fn draw_won(&mut self) {
        draw_text(
            "Du hast das gesuchte Wort herausgefunden!",
            10.,
            30. + ROWS as f32 * CELL_HEIGHT * CELL_SPACING,
            20.,
            GREEN,
        );
    }

    pub fn draw_lost(&mut self) {
        draw_text(
            &format!(
                r#"Du konntest das gesuchte Wort ("{}") nicht finden!"#,
                self.selected_word.word
            ),
            10.,
            30. + ROWS as f32 * CELL_HEIGHT * CELL_SPACING,
            20.,
            RED,
        );
    }

    pub fn draw_grid(&mut self) {
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

    pub fn word_in_list(&self) -> bool {
        let word = &self.fields[self.cursor.cursor_pos.0];
        if !is_length_cols(word) {
            return false;
        }
        self.words
            .contains(&fields_to_string(word).to_ascii_uppercase())
    }

    pub fn field_in_current_row(&mut self) -> &mut [Field] {
        &mut self.fields[self.cursor.cursor_pos.0]
    }

    pub fn mark_chars(&mut self) {
        let selected_word = self.selected_word.clone();
        let word_list = self.field_in_current_row();
        let word = fields_to_string(word_list).to_ascii_uppercase();

        let mut zeroed_char_map = word
            .chars()
            .into_iter()
            .map(|char| (char, 0))
            .collect::<HashMap<char, usize>>();

        mark_correct_pos_chars(&selected_word, &word, word_list, &mut zeroed_char_map);
        mark_contained_chars(&selected_word, &word, word_list, &mut zeroed_char_map);
    }

    pub fn check_win(&self) -> State {
        if self.selected_word.word
            == fields_to_string(&self.fields[self.cursor.cursor_pos.0]).to_ascii_uppercase()
        {
            State::Won
        } else if self.cursor.cursor_pos.0 == ROWS - 1 {
            State::Lost
        } else {
            State::Playing
        }
    }

    pub fn game_logic(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            enter(self);
        } else if is_key_pressed(KeyCode::Left) {
            self.cursor.move_left();
        } else if is_key_pressed(KeyCode::Right) {
            self.cursor.move_right();
        } else if is_key_pressed(KeyCode::Backspace) {
            if self.cursor.selected && self.field().input_char == ' ' {
                self.cursor.move_left();
            }

            self.update_field(' ');
            self.cursor.selected = true;
        } else if let Some(pressed_char) = get_char_pressed() {
            if (('a'..='z').contains(&pressed_char) || ('A'..='Z').contains(&pressed_char))
                && self.cursor.selected
            {
                self.update_field(pressed_char.to_ascii_uppercase());
                self.cursor.move_right();
                //wordle.cursor.unselected_del = false;
            }
        }
    }
}

pub fn fields_to_string(word: &[Field]) -> String {
    let mut string_word = String::with_capacity(word.len());
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

pub fn mark_correct_pos_chars(selected_word: &Word, word: &str, word_list: &mut [Field], char_map: &mut HashMap<char, usize>) {
    for (idx, (lhs, rhs)) in word
        .chars()
        .zip(selected_word.word.chars())
        .enumerate()
    {
        let background_color = &mut word_list[idx].background_color;
        *background_color = LIGHTGRAY;

        let char_count = char_map.get_mut(&lhs).unwrap();

        if lhs == rhs {
            *background_color = GREEN;
            *char_count += 1;
        }
    }
}

pub fn mark_contained_chars(selected_word: &Word, word: &str, word_list: &mut [Field], char_map: &mut HashMap<char, usize>) {
    for (idx, lhs) in word
        .chars()
        .enumerate()
    {
        let background_color = &mut word_list[idx].background_color;

        let Some(max_mark_count_for_char) = selected_word.chars.get(&lhs) else {
            continue;
        };

        let char_count = char_map.get_mut(&lhs).unwrap();

        if *char_count < *max_mark_count_for_char && *background_color == LIGHTGRAY {
            *char_count += 1;
            word_list[idx].background_color = YELLOW;
        }
    }
}