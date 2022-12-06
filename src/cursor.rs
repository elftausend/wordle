#[derive(Debug, Default)]
pub struct Cursor {
    pub selected: bool,
    pub cursor_pos: (usize, usize),
    pub cols: usize,
    pub rows: usize,
}

impl Cursor {
    pub fn move_right(&mut self) -> bool {
        if self.cursor_pos.1 >= self.cols - 1 {
            return false;
        }
        self.cursor_pos.1 += 1;
        self.selected = true;
        true
    }

    pub fn move_left(&mut self) -> bool {
        if self.cursor_pos.1 == 0 {
            return false;
        }
        self.cursor_pos.1 -= self.selected as usize;
        self.selected = true;

        true
    }

    pub fn move_down(&mut self) -> bool {
        if self.cursor_pos.0 >= self.rows - 1 {
            return false;
        }
        self.cursor_pos.0 += 1;
        true
    }

    pub fn is_cursor_on_pos(&self, pos: (usize, usize)) -> bool {
        pos.0 == self.cursor_pos.0 && pos.1 == self.cursor_pos.1
    }

    pub fn col_reset(&mut self) {
        self.cursor_pos.1 = 0;
    }
}
