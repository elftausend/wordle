use crate::wordle::Wordle;

pub fn enter(wordle: &mut Wordle) {
    if wordle.word_in_list() {
        wordle.mark_chars();
        wordle.cursor.move_down();
        wordle.cursor.col_reset();
        wordle.cursor.selected = true;
    }
    // improve
    if wordle.check_win() {
        wordle.mark_chars();
    }
}
