use crate::wordle::{State, Wordle};

pub fn enter(wordle: &mut Wordle) {
    if wordle.word_in_list() {
        wordle.game_state = wordle.check_win();

        wordle.mark_chars();

        wordle.cursor.move_down();
        wordle.cursor.col_reset();
        wordle.cursor.selected = true;
        return;
    }

    wordle.game_state = State::Playing;
}
