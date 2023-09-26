use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

const SIZE: usize = 9;

#[derive(Serialize, Deserialize)]
struct Sudoku {
    board: [[u8; SIZE]; SIZE],
}

impl Sudoku {
    fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut is_empty = false;

        // Find the first empty cell
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

        // Return true if there are no empty cells
        if !is_empty {
            return true;
        }

        // Fill the safe number into cell
        for num in 1..=SIZE {
            if is_safe(self, row, col, num as u8) {
                self.board[row][col] = num as u8;

                if self.solve() {
                    return true;
                }

                self.board[row][col] = 0;
            }
        }

        false
    }
}

fn is_safe(sudoku: &Sudoku, row: usize, col: usize, num: u8) -> bool {
    // Check row
    for i in 0..SIZE {
        if sudoku.board[row][i] == num {
            return false;
        }
    }

    // Check col
    for i in 0..SIZE {
        if sudoku.board[i][col] == num {
            return false;
        }
    }

    // Check box
    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in 0..3 {
        for j in 0..3 {
            if sudoku.board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

async fn solve_sudoku(Json(mut sudoku): Json<Sudoku>) -> Result<Json<Sudoku>, StatusCode> {
    if sudoku.solve() {
        Ok(Json(sudoku))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/solve", post(solve_sudoku));

    Ok(router.into())
}
