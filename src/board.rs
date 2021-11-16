use crate::{print, get_size, repeat_str};

pub fn board_template() {
    let board = get_size();
    print(["#", repeat_str(" ", board.1-10)].join("").as_ref(), 0x7, (0,0));
}