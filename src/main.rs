use std::io::Write;

enum MoveResult {
    FreeTurn,
    EmptySpace,
    GameOver,
    Avalanche(usize),
}
struct MancalaBoard {
    spaces: [u8; 13],
    opponent_store: u8,
    move_history: Vec<u8>,
    move_count: u8,
    did_avalanche: bool,
}

impl Default for MancalaBoard {
    fn default() -> Self {
        MancalaBoard {
            spaces: [0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            opponent_store: 0,
            move_history: vec![],
            move_count: 0,
            did_avalanche: false,
        }
    }
}

impl Clone for MancalaBoard {
    fn clone(&self) -> Self {
        return MancalaBoard {
            spaces: self.spaces,
            opponent_store: self.opponent_store,
            move_history: self.move_history.clone(),
            move_count: self.move_count,
            did_avalanche: false,
        };
    }
}

impl MancalaBoard {
    fn new(spaces: [u8; 13], opponent_store: u8) -> MancalaBoard {
        return MancalaBoard {
            spaces,
            opponent_store,
            move_history: vec![],
            move_count: 0,
            did_avalanche: false,
        };
    }

    fn move_piece(&mut self, mut space: usize) -> MoveResult {
        if !self.did_avalanche {
            self.move_history.push(
                space
                    .try_into()
                    .expect("Could not convert usize to u8 for move history"),
            );
        } else {
            self.did_avalanche = false;
        }

        self.move_count += 1;

        let mut hand = self.spaces[space];
        self.spaces[space] = 0;

        while hand > 0 {
            space += 1;
            if space >= 13 {
                space -= 13
            }
            self.spaces[space] += 1;
            hand -= 1;
        }

        if space == 0 && is_zero(&self.spaces[6..]) {
            return MoveResult::GameOver;
        } else if space == 0 {
            return MoveResult::FreeTurn;
        } else if self.spaces[space] > 1 {
            self.did_avalanche = true;
            return MoveResult::Avalanche(space);
        } else if is_zero(&self.spaces[6..]) {
            return MoveResult::GameOver;
        } else {
            return MoveResult::EmptySpace;
        }
    }
}

fn print_board(board: MancalaBoard) {
    println!(
        "[  ][{}][{}][{}][{}][{}][{}][  ]",
        board.spaces[12],
        board.spaces[11],
        board.spaces[10],
        board.spaces[9],
        board.spaces[8],
        board.spaces[7]
    );
    println!(
        "[{}]-------------------[{}]",
        board.spaces[0], board.opponent_store
    );
    println!(
        "[  ][{}][{}][{}][{}][{}][{}][  ]",
        board.spaces[1],
        board.spaces[2],
        board.spaces[3],
        board.spaces[4],
        board.spaces[5],
        board.spaces[6]
    );
    println!();
}

// https://stackoverflow.com/questions/65367552/how-to-efficiently-check-a-vecu8-to-see-if-its-all-zeros
fn is_zero(buf: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

fn get_user_board() -> MancalaBoard {
    let mut input = String::new();

    // Get player store
    print!("Player Store: ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let player_store: u8 = input
        .trim()
        .parse()
        .expect("error parsing player store input");
    input.clear();

    // Get opponent store
    print!("Opponent Store: ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let opponent_store: u8 = input
        .trim()
        .parse()
        .expect("error parsing player store input");
    input.clear();

    // Get player spaces
    print!("Player Spaces (opponent->player, space-seperated): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut player_spaces: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|number_string| number_string.parse::<u8>().unwrap().try_into().unwrap())
        .collect();

    input.clear();

    // Get opponent spaces
    print!("Opponent Spaces (player->opponent, space-seperated): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut opponent_spaces: Vec<u8> = input
        .trim()
        .split(' ')
        .map(|number_string| number_string.parse::<u8>().unwrap().try_into().unwrap())
        .collect();
    input.clear();

    // combine input into array
    let mut spaces = vec![player_store];
    spaces.append(&mut opponent_spaces);
    spaces.append(&mut player_spaces);

    MancalaBoard::new(
        spaces
            .try_into()
            .expect("Error converting spaces vec into array"),
        opponent_store,
    )
}

fn simulate(board: MancalaBoard, depth: u8) -> MancalaBoard {
    let mut initial_space_stack: Vec<MancalaBoard> = vec![];
    for mut space in 7..=12 {
        let mut stack: Vec<MancalaBoard> = vec![];
        let mut final_stack: Vec<MancalaBoard> = vec![];

        let mut board = board.clone();

        loop {
            match board.move_piece(space) {
                MoveResult::FreeTurn => {
                    stack.push(board);
                    break;
                }
                MoveResult::EmptySpace => {
                    final_stack.push(board);
                    break;
                }
                MoveResult::Avalanche(new_space) => {
                    space = new_space;
                }
                MoveResult::GameOver => {
                    final_stack.push(board);
                    break;
                }
            }
        }

        while stack.len() > 0 {
            let stack_board = stack.pop().expect("Stack Empty");

            for mut space in 7..=12 {
                let mut temp_board = stack_board.clone();
                loop {
                    match temp_board.move_piece(space) {
                        MoveResult::FreeTurn => {
                            if temp_board.move_count >= depth {
                                final_stack.push(temp_board);
                                break;
                            }
                            stack.push(temp_board);
                            break;
                        }
                        MoveResult::EmptySpace => {
                            final_stack.push(temp_board);
                            break;
                        }
                        MoveResult::Avalanche(new_space) => {
                            space = new_space;
                        }
                        MoveResult::GameOver => {
                            final_stack.push(temp_board);
                            break;
                        }
                    }
                }
            }
        }
        let mut top_board = final_stack.pop().unwrap();

        final_stack.into_iter().for_each(|final_board| {
            if final_board.spaces[0] > top_board.spaces[0] {
                top_board = final_board;
            };
        });
        initial_space_stack.push(top_board);
    }

    let mut final_board = initial_space_stack.pop().unwrap();
    initial_space_stack.into_iter().for_each(|board| {
        if board.spaces[0] > final_board.spaces[0] {
            final_board = board;
        }
    });
    return final_board;
}

fn main() {
    let board = get_user_board();
    let solved_board = simulate(board, 100);

    println!("{:?}", solved_board.move_history);
    print_board(solved_board);
}

#[cfg(test)]
mod tests {
    use crate::{simulate, MancalaBoard};

    #[test]
    fn default_board() {
        let board = MancalaBoard::default();
        assert_eq!([0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4], board.spaces);
    }

    #[test]
    fn new_board() {
        let board = MancalaBoard::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 0);
        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], board.spaces);
    }

    #[test]
    fn solve_default_board() {
        let board = MancalaBoard::default();
        let solved_board = simulate(board, 100);
        assert_eq!(
            solved_board.move_history,
            [
                12, 9, 8, 11, 7, 7, 11, 11, 12, 7, 9, 10, 12, 8, 12, 12, 11, 8, 12, 8, 12, 10, 12,
                11, 12, 7
            ]
        );
    }

    #[test]
    fn move_piece() {
        let mut board = MancalaBoard::default();
        board.move_piece(7);
        assert_eq!(board.spaces, [0, 4, 4, 4, 4, 4, 4, 0, 5, 5, 5, 5, 4]);
    }
}
