//TIC TAC TOE AGAIN... YAAAAAAYYYY

use std::io;
use std::string;

fn main() {
    let board: &str = {
        "1";
        "2";
        "3";
        "4";
        "5";
        "6";
        "7";
        "8";
        "9"
    };
    let player = 1;

    clearscreen::clear().expect("failureeeee");
    println!("TIC... TAC... TOE... rust edition");
    println!("Player 1: X and Player 2: O");

    while (true) {
        if (player % 2 == 0) {
            println!("player 2 turn");
        } else {
            println!("player 1 turn");
        }

        //getting user input and validating
        while (true) {
            println!("Pick spot 1-9 to make ya move ya goober: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failureeeee");

            let player_choice: i32 = input.trim().parse().expect("failureeee");

            if (player_choice > 9) {
                println!("u stupid... try again");
                continue;
            }
            // else if(board(player_choice) - 1 == "O" || board(player_choice) - 1 == "X"){
            //     println!("pick again...");
            //     continue;

            // }

            //updating de board
            CheckWin();
            CheckDraw();
            break;
        }
    }

    //switching de player
    println!("player 2 turn");
    while (true) {
        println!("pick a spot on the de board u goober: ");

        //updating board again
        CheckWin();
        CheckDraw();
        break;
    }
}

fn DrawBoard() {}

fn CheckWin() {}

fn CheckDraw() {
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
