use std::io;
use std::string;

fn main() {
    let mut board = [' '; 9];
    let mut player = 'X';

    clearscreen::clear().expect("failureeeee");
    println!("TIC... TAC... TOE... rust edition");
    println!("Player 1: X and Player 2: O");

    loop{
        //drawing board
        println!(" {} | {} | {} ", board[0], board[1], board[2]);
        println!("-----------");
        println!(" {} | {} | {} ", board[3], board[4], board[5]);
        println!("-----------");
        println!(" {} | {} | {} ", board[6], board[7], board[8]);

        //getting user input and validating
            println!("Pick spot 1-9 to make ya move ya goober: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failureeeee");

            let player_choice: i32 = input.trim().parse().expect("failureeee");

            if (player_choice > 9) {
                println!("u stupid... try again");
                continue;
            }
            // else if(player_choice - 1 == "O" || player_choice - 1 == "X"){
            //     println!("pick again...");
            //     continue;

            // }

            //updating de board
            //CheckWin();
            //CheckDraw();
            break;
        }
    }

fn CheckWin(b: & [char; 9], p: char) -> bool {
    //all horizontal wins
    (b[0]==p)

    //all veritcal wins


    //all diagonal wins

}

fn CheckDraw() {
    
    println!("TIE GAME.. BAAAAANG");
    ResetGame();
}

fn ResetGame() {
    println!("ggs. wanna run it back? (Y/N): ");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("failureeeee");

    if (response == "Y") {
        println!("good luck we're running it again");
        main();
    } else if (response == "N") {
        println!("okie... byeeeeee")
    } else {
        println!("idk wtf u entered but um... byeeeeee")
    }
}
