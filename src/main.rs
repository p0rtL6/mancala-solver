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
}

impl Default for MancalaBoard {
    fn default() -> Self {
        MancalaBoard {
            spaces: [0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
            opponent_store: 0,
            move_history: vec![],
        }
    }
}

impl Clone for MancalaBoard {
    fn clone(&self) -> Self {
        return MancalaBoard {
            spaces: self.spaces,
            opponent_store: self.opponent_store,
            move_history: self.move_history.clone(),
        };
    }
}

impl MancalaBoard {
    fn new(spaces: [u8; 13], opponent_store: u8) -> MancalaBoard {
        return MancalaBoard {
            spaces,
            opponent_store,
            move_history: vec![],
        };
    }

    fn move_piece(&mut self, space: usize) -> MoveResult {
        // add 6 to make player spaces 1-6
        let mut space: usize = space;
        if space >= 13 {
            space -= 13
        }

        self.move_history.push(
            space
                .try_into()
                .expect("Could not convert usize to u8 for move history"),
        );

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

        print_board(self.clone());

        if space == 0 && is_zero(&self.spaces[6..]) {
            return MoveResult::GameOver;
        } else if space == 0 {
            return MoveResult::FreeTurn;
        } else if self.spaces[space] > 1 {
            return MoveResult::Avalanche(space);
        } else if is_zero(&self.spaces[6..]) {
            return MoveResult::GameOver;
        } else {
            return MoveResult::EmptySpace;
        }
    }
}

fn print_board(board: MancalaBoard) {
    println!("[  ][{}][{}][{}][{}][{}][{}][  ]", board.spaces[12], board.spaces[11], board.spaces[10], board.spaces[9], board.spaces[8], board.spaces[7]);
    println!("[{}]-------------------[{}]", board.spaces[0], board.opponent_store);
    println!("[  ][{}][{}][{}][{}][{}][{}][  ]", board.spaces[1], board.spaces[2], board.spaces[3], board.spaces[4], board.spaces[5], board.spaces[6]);
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
    print!("Player Spaces (opponent->player): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut player_spaces: Vec<u8> = input
        .trim()
        .chars()
        .map(|number_string| number_string.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    input.clear();

    // Get opponent spaces
    print!("Opponent Spaces (player->opponent): ");
    std::io::stdout().flush().expect("error flushing stdout");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");

    let mut opponent_spaces: Vec<u8> = input
        .trim()
        .chars()
        .map(|number_string| number_string.to_digit(10).unwrap().try_into().unwrap())
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

fn simulate(board: MancalaBoard, depth: usize) -> MancalaBoard {
    let mut space_stack = vec![];
    for mut space in 7..=12 {
        let mut stack: Vec<MancalaBoard> = vec![];
        let mut final_stack: Vec<MancalaBoard> = vec![];

        let mut board = board.clone();

        // while board.move_piece(space) {

        // }

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
            println!("iterating stack, left: {}", stack.len());
            let stack_board = stack.pop().expect("Stack Empty");

            for mut space in 7..=12 {
                let mut temp_board = stack_board.clone();
                loop {
                    match temp_board.move_piece(space) {
                        MoveResult::FreeTurn => {
                            if temp_board.move_history.len() >= depth {
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
        println!("starting board compare with: {}", top_board.spaces[0]);

        final_stack.into_iter().for_each(|final_board| {
            println!(
                "comparing board, old: {}, new: {}",
                top_board.spaces[0], final_board.spaces[0]
            );
            if final_board.spaces[0] > top_board.spaces[0] {
                top_board = final_board;
            };
        });
        space_stack.push(top_board);
    }

    let mut final_board = space_stack.pop().unwrap();
    println!("starting board compare with: {}", final_board.spaces[0]);
    space_stack.into_iter().for_each(|board| {
        println!(
            "comparing board, old: {}, new: {}",
            final_board.spaces[0], board.spaces[0]
        );
        if board.spaces[0] > final_board.spaces[0] {
            final_board = board;
        }
    });
    return final_board;
}

fn main() {
    let board = get_user_board();

    print_board(board.clone());
    // let board = MancalaBoard::default();

    let final_board = simulate(board, 3);
    println!("{:?}", final_board.move_history);
    print_board(final_board);


    // board.move_piece(12);
    // println!("{:?}", board.move_history);
}
