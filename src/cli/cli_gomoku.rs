use std::io::{self, Write};

const SIZE: usize = 19;

fn print_board(board: &[[char; SIZE]; SIZE]) {
    print!("    ");
    for x in 0..SIZE {
        print!("{:^4}", x);
    }
    println!();

    print!("   ┌");
    for x in 0..SIZE {
        if x + 1 == SIZE {
            print!("───┐");
        } else {
            print!("───┬");
        }
    }
    println!();

    for y in 0..SIZE {
        print!("{:2} │", y);
        for x in 0..SIZE {
            print!("{:^3}", board[y][x]);
            if x + 1 < SIZE {
                print!("│");
            }
        }
        println!("│");

        if y + 1 < SIZE {
            print!("   ├");
            for x in 0..SIZE {
                if x + 1 == SIZE {
                    print!("───┤");
                } else {
                    print!("───┼");
                }
            }
            println!();
        }
    }


    print!("   └");
    for x in 0..SIZE {
        if x + 1 == SIZE {
            print!("───┘");
        } else {
            print!("───┴");
        }
    }
    println!();
}

pub fn start_game() {
    
    let mut board: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];
    
    let mut turn = 0;
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Starting CLI Gomoku...");
        print_board(&board);
        let mut turn_finished = false;
        while turn_finished == false {
            print!("Turn {} (player {}'s turn): Please enter coordinates separated by a space:", turn, turn%2+1);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }

            let x: usize = match parts[0].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let y: usize = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            if x >= SIZE || y >= SIZE {
                continue;
            }

            if board[y][x] == ' ' {
                if turn % 2 == 0 {
                    board[y][x] = 'X';
                } else {
                    board[y][x] = 'O';
                }
                turn += 1;
                turn_finished = true;
            } else {
                println!("Cell already occupied. Try again.");
                continue;
            }

        }
    }
}
