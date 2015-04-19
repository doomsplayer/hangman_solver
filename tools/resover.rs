#![feature(plugin,io)]
#![plugin(docopt_macros)]
#[macro_use] extern crate log;
extern crate env_logger;
extern crate docopt;
extern crate rustc_serialize;
extern crate hangman;
use hangman::*;
use std::io::stdin;
use std::io::Read;

docopt!(Args, r"
Usage: hangman guess [-a <api-addr>] [-u <user-id>] -d <dict> [-j <num>]
       hangman continue <word> [-a <api-addr>] [-u <user-id>] -s <session_id> -d <dict> [-j <num>] -h <history>

Options:
       -a <api-addr>     Api address [default: https://strikingly-hangman.herokuapp.com/game/on].
       -u <user-id>      User id [default: doomsplayer@gmail.com].
       -s <session-id>   Session id.
       -d <dict>         Path to dict file.
       -j <num>          Jump words whose length is less than <num> [default: 3]
       -h <history>      Guess history
");

fn main() {
    
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit()); // parsing arguments
    
    /* 
     * init the logger
     */
    env_logger::init().unwrap();


    let d = Dict::new(&args.flag_d).unwrap(); // new a dict to guess

    
    let mut game = Game::new(args.flag_a, args.flag_u); // new a game object
    let word = args.arg_word;
    let history = args.flag_h;
    if args.cmd_guess {
        // start a new guess
        let ret = game.start_game();
        if let Err(e) = ret {
            error!("start game: game error: {:?}", e);
            return;
        }
    } else {
        game.force_start(args.flag_s);
        let mut guesser = d.guess();
        guesser.set_history(history.chars().collect());
        game.current_word = Some(word);

        let mut retry_time = 0;
        let mut a_guess = guesser.guess(&game.current_word.as_ref().unwrap()[..]);
        loop {
            if a_guess.is_none() {
                break;
            }
            if let Err(e) = game.guess(a_guess.unwrap()) {
                match e {
                    GameError::GuessFailed|GameError::GuessComplete => break,
                    GameError::HttpError(e) => {
                        error!("{:?}", e);
                        retry_time += 1;
                        if retry_time > 10 {
                            error!("retry exceed");
                            return;
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        error!("game error: {:?}", e);
                        return;
                    }
                }
            } else {
                a_guess = guesser.guess(&game.current_word.as_ref().unwrap()[..]);
            }
        }
    }
    
    let mut retry_time = 0;
    loop {
        if let Err(e) = game.next_word() {
            match e {
                GameError::GameFinished => {
                    let ret = game.get_result().unwrap();
                    info!(
                        "game finished, score is: {}, correct words: {}, total wrong guess: {}",
                        ret.data.score,
                        ret.data.correctWordCount,
                        ret.data.totalWrongGuessCount);
                    
                    println!("submit score? y/n");
                    
                    if stdin().chars().take(1).any(|c| c.unwrap() == 'y') {
                        info!("{:?}", game.submit_result());
                    }
                    return;
                }
                GameError::HttpError(e) => {
                    error!("{:?}", e);
                    retry_time += 1;
                    if retry_time > 10 {
                        error!("retry exceed");
                        return;
                    } else {
                        continue;
                    }
                },
                _ => {
                    error!("game error: {:?}", e);
                    return;
                }
            }
        }
        if game.current_word.as_ref().unwrap().len() <= args.flag_j.parse().unwrap() {
            info!("jump word: {}", game.current_word.as_ref().unwrap());
            continue;
        }
        let mut guesser = d.guess();
        
        let mut retry_time = 0;
        loop {
            let a_guess = guesser.guess(&game.current_word.as_ref().unwrap()[..]);
            if a_guess.is_none() {
                break;
            }
            if let Err(e) = game.guess(a_guess.unwrap()) {
                match e {
                    GameError::GuessFailed|GameError::GuessComplete => break,
                    GameError::HttpError(e) => {
                        error!("{:?}", e);
                        retry_time += 1;
                        if retry_time > 10 {
                            error!("retry exceed");
                            return;
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        error!("game error: {:?}", e);
                        return;
                    }
                }
            }
        }
    }
}

    
