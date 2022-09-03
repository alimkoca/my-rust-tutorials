use std::io::{stdin, stdout, Write};

const X: u8 = 1;
const O: u8 = 2;

fn is_not_zero(dt: u8) -> bool {
    match dt {
        1 => true,
        0 => false,
        _ => false,
    }
}

fn check_table_is_complete(t: &mut [u8; 9]) -> bool {
    for x in 0..8 {
        if t[x] == 0 {
            return false;
        }
    }
    return true;
}

fn check_win_or_not(t: &mut [u8; 9]) -> bool {
        if is_not_zero(t[0]) && is_not_zero(t[1]) && is_not_zero(t[2]) {
            return true;
        } 
        else if is_not_zero(t[3]) && is_not_zero(t[4]) && is_not_zero(t[5]) {
            return true;
        }
        else if is_not_zero(t[6]) && is_not_zero(t[7]) && is_not_zero(t[8]) {
            return true;
        }
        else if is_not_zero(t[0]) && is_not_zero(t[3]) && is_not_zero(t[6]) {
            return true;
        }
        else if is_not_zero(t[1]) && is_not_zero(t[4]) && is_not_zero(t[7]) {
            return true;
        }
        else if is_not_zero(t[2]) && is_not_zero(t[5]) && is_not_zero(t[8]) {
            return true;
        }
        else if is_not_zero(t[0]) && is_not_zero(t[4]) && is_not_zero(t[8]) {
            return true;
        }
        else if is_not_zero(t[2]) && is_not_zero(t[4]) && is_not_zero(t[6]) {
            return true;
        }

    return false;
}

fn get_move_input(t: &mut [u8; 9], mvbywho: u8) {
    match mvbywho {
        0 => {
            let mut num: usize;

            loop {
                print!("Player X, please enter move: ");
                num = input().trim().parse::<usize>().unwrap();

                if do_move(t, X, num) {
                    break;
                }
            }
        }

        1 => {
            let mut num: usize;

            loop {
                print!("Player O, please enter move: ");
                num = input().trim().parse::<usize>().unwrap();

                if do_move(t, O, num) {
                    break;
                }
            }
        }

        _ => println!("How did you jump here")
    }
}

fn input() -> String {
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)
        .expect("input error");
    return buffer;
}

fn do_move(t: &mut [u8; 9], x_or_o: u8, 
           num: usize) -> bool {
    if num > 9 || num == 0 {
        return false;
    }

    if t[num-1] != 0 {
        return false;
    }

    t[num-1] = x_or_o;
    return true;
}

fn main() {
    let mut a = 0;
    let mut mvbywho: u8 = 0;
    let mut ttt: [u8; 9] = [0; 9];

    loop {
        a = 0;

        for x in ttt {
            a += 1;

            print!("||{x}||");

            if a % 3 == 0 {
                print!("\n");
            }
        }

        get_move_input(&mut ttt, mvbywho);

        if check_win_or_not(&mut ttt) {
            match mvbywho {
                0 => println!("Win by X!"),
                1 => println!("Win by O!"),
                // Unreachable case
                _ => println!("How did you find here?")
            }

            break;
        }

        if check_table_is_complete(&mut ttt) {
            println!("No winner, found two losers, haha");
            break;
        }

        mvbywho = (mvbywho + 1) % 2;
    }
}
