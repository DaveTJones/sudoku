#![allow(dead_code)]
// https://www.reddit.com/r/sveltejs/comments/vzf86d/sveltekit_with_webassembly_rust/
fn main() {
    let mut grid = Sudoku {
        board: [
            [0, 0, 0, 1, 0, 4, 8, 0, 0],
            [2, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 0, 9, 0, 0, 0, 1, 0, 0],
            [0, 2, 0, 0, 8, 9, 6, 0, 7],
            [0, 7, 0, 0, 1, 0, 0, 3, 0],
            [9, 0, 6, 4, 7, 0, 0, 8, 0],
            [0, 0, 7, 0, 0, 0, 2, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 0, 6],
            [0, 0, 2, 5, 0, 1, 0, 0, 0],
        ],
    };
    grid.display();
    grid.solve();
    grid.display();
}

const SIZE: usize = 9;

#[derive(Debug)]
struct Sudoku {
    board: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut is_empty = false;

        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    row = i;
                    col = j;
                    is_empty = true;
                    break;
                }
            }
            if is_empty {
                break;
            }
        }

        if !is_empty {
            return true;
        }

        for num in 1..=SIZE {
            if self.is_safe(row, col, num as u8) {
                self.board[row][col] = num as u8;
                if self.solve() {
                    return true;
                }
                self.board[row][col] = 0;
            }
        }
        false
    }

    fn is_safe(&mut self, row: usize, col: usize, num: u8) -> bool {
        for i in 0..SIZE {
            if self.board[row][i] == num {
                return false;
            }
        }
        for i in 0..SIZE {
            if self.board[i][col] == num {
                return false;
            }
        }

        let start_row = row - row % 3;
        let start_col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + start_row][j + start_col] == num {
                    return false;
                }
            }
        }
        true
    }
    fn display(&self) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                print!("{} ", self.board[row][col]);
                if col == 2 || col == 5 {
                    print!("|");
                }
            }
            if row == 2 || row == 5 {
                println!("\n______|______|_____");
            } else {
                println!("");
            }
        }
        println!("\n");
    }
}
