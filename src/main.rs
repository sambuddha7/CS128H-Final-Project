#[macro_use] extern crate rocket;

use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use black_jack::message::Message;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

pub mod black_jack;
use black_jack::runner::BlackJackRunner;
use crate::black_jack::player::Player;
use crate::black_jack::deck::Deck;

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    unsafe { if STATE != 0 { state(STATE, form.message.clone()); }}
    if form.message.clone() == "Dealer" {
        unsafe { RESPOND = 15; }
    } else if form.message.clone() == "State" {
        unsafe { RESPOND = 404; }
    } else if form.message.clone() == "Rest" {
        unsafe { STATE = 100; }
    }
    let _res = queue.send(form.into_inner());
}

#[get("/game")]
fn game() {
    let mut blackjack_runner = BlackJackRunner::new();
    blackjack_runner.run();
}

static mut STATE: u32 = 0;
static mut RESPOND: u32 = 0;
static mut ITERATOR: usize = 0;
static mut N_PLAYER: usize = 0;
static mut PLAYERS: Vec<Player> = vec![];
static mut DECK: Vec<Deck> = vec![];

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);

            unsafe {
            if (RESPOND == 0 && msg.message.to_lowercase() == "blackjack") {
                STATE = 1;
                yield respond(1);
            }
            if (RESPOND != 0) {
                if (msg.message == "quit") {
                    STATE = 100;
                    yield respond(0);
                } else {
                    yield respond(RESPOND);
                }
            }}
        }
    }
}

fn state(state: u32, input: String) {
    unsafe {
    if state == 1 { // Start
        STATE = 2;
        RESPOND = 2;
    } else if state == 2 { // Ask for number of decks
        match input.parse::<usize>() {
            Ok(val) => {
                if val >= 6 && val <= 8 {
                    STATE = 3;
                    RESPOND = 3;
                    DECK.push(Deck::new(val));
                    DECK[0].shuffle();
                } else {
                    RESPOND = 200;
                }
            }
            Err(_) => {
                RESPOND = 210;
            }
        }
    } else if state == 3 { // Game start
        STATE = 4;
        RESPOND = 4
    } else if state == 4 { // Ask for number of players
        match input.parse::<usize>() {
            Ok(val) => {
                if val >= 1 && val <= 7 {
                    STATE = 5;
                    RESPOND = 5;
                    N_PLAYER = val;
                } else {
                    RESPOND = 220;
                }
            }
            Err(_) => {
                RESPOND = 210;
            }
        }
    } else if state == 5 { // Establish new players
        if ITERATOR < N_PLAYER {
            PLAYERS.push(Player::new(String::from(input.trim())));
            PLAYERS[ITERATOR + 1].initial_r(&mut DECK[0]);
            ITERATOR += 1;
        }
        if ITERATOR >= N_PLAYER { // Show dealer's first card
            PLAYERS[0].initial_r(&mut DECK[0]);
            STATE = 6;
            RESPOND = 6;
            ITERATOR = 0;
        }
    } else if state == 6 { // Show player's cards
        if ITERATOR < N_PLAYER {
            if PLAYERS[ITERATOR + 1].has_black_jack() { // Blackjack
                RESPOND = 100;
                ITERATOR += 1;
            } else { // Player's turn
                STATE = 7;
                RESPOND = 7;
            }
        }
        if ITERATOR >= N_PLAYER { // Dealer's turn
            STATE = 9;
            RESPOND = 9
        }
    } else if state == 7 { // Player choose to hit or stand
        match input.to_lowercase().as_str() {
            "h" | "hit" => {
                PLAYERS[ITERATOR + 1].hit(&mut DECK[0]);
                let score = PLAYERS[ITERATOR + 1].get_score();
                if score < 21 { // Same player
                    RESPOND = 8;
                } else if score == 21{ // 21 points, next player
                    STATE = 6;
                    RESPOND = 110;
                    ITERATOR += 1;
                } else { // Bust, next plyaer
                    STATE = 6;
                    RESPOND = 120;
                    ITERATOR += 1;
                }
            }
            "s" | "stand" => {
                STATE = 6;
                RESPOND = 130;
                ITERATOR += 1;
            }
            _ => RESPOND = 230,
        }
    } else if state == 9 { // Dealer draw cards
        while PLAYERS[0].get_score() < 17 {
            PLAYERS[0].hit(&mut DECK[0]);
        }
        STATE = 10;
        if PLAYERS[0].has_black_jack() {
            RESPOND = 100;
        } else if PLAYERS[0].get_score() == 21 {
            RESPOND = 140;
        } else if PLAYERS[0].get_score() > 21{
            RESPOND = 150;
        } else {
            RESPOND = 140;
        }
    } else if state == 10 { // Game end
        STATE = 11;
        RESPOND = 11;
    } else if state == 11 { // Show result
        STATE = 100;
        RESPOND = 12;
    } else if state == 100 { // Abort
        STATE = 0;
        RESPOND = 0;
        ITERATOR = 0;
        N_PLAYER = 0;
        PLAYERS = Vec::new();
        PLAYERS.push(Player::new(String::from("Dealer")));
        DECK = Vec::new();
    } else { // Unexpected error
        RESPOND = 404;
    }}
}

fn respond(respond: u32) -> Event {
    unsafe {
    if respond == 0 {
        bot("Game aborted.")
    } else if respond == 1 {
        bot("Welcome to BlackJack! Enter \"quit\" to leave the game\t(Type anything to continue)")
    } else if respond == 2 {
        bot("How many decks do you wanna use? (6-8)")
    } else if respond == 3 {
        bot("####### Game Started! #######\t(Type anything to continue)")
    } else if respond == 4 {
        bot("How many players are playing? (1-7)")
    } else if respond == 5 {
        bot(format!("Player {}, please, enter your name.", ITERATOR + 1).as_str())
    } else if respond == 6 {
        bot(format!("The first card of the dealer is {}\t(Type anything to continue)", PLAYERS[0].get_hand()[0]).as_str())
    } else if respond == 7 {
        let initial_cards = PLAYERS[ITERATOR + 1].get_hand();
        bot(format!("{}'s cards are:\n{} and {} ({} points)\tWhat do you want to do?\t(h)it, (s)tand", 
        PLAYERS[ITERATOR + 1], 
        initial_cards[0], 
        initial_cards[1], 
        PLAYERS[ITERATOR + 1].get_score()).as_str())
    } else if respond == 8 {
        bot(format!("Now, your cards are: {} ({} points)\tWhat do you want to do?\t(h)it, (s)tand",
        show_cards(&PLAYERS[ITERATOR + 1]), 
        PLAYERS[ITERATOR + 1].get_score()).as_str())
    } else if respond == 9 {
        bot("Dealer's turn...\t(Type anything to continue)")
    } else if respond == 11 {
        bot("####### Game Finished #######\t(Type anything to continue)")
    } else if respond == 12 {
        let mut end = "".to_string();
        let mut winner: Vec<&Player> = Vec::new();
        let mut tie: Vec<&Player> = Vec::new();
        let mut lost: Vec<&Player> = Vec::new();
        if PLAYERS[0].has_black_jack() { // Dealer blackjack
            println!("234");
            for i in 1..PLAYERS.len() {
                if PLAYERS[i].has_black_jack() {
                    tie.push(&PLAYERS[i]);
                } else {
                    lost.push(&PLAYERS[i]);
                }
            }
        } else if PLAYERS[0].bust() { // Dealer bust
            println!("243");
            for i in 1..PLAYERS.len() {
                if !PLAYERS[i].bust() {
                    winner.push(&PLAYERS[i]);
                } else {
                    tie.push(&PLAYERS[i]);
                }
            }
        } else {
            println!("254");
            for i in 1..PLAYERS.len() {
                if !PLAYERS[i].bust() && PLAYERS[i].get_score() > PLAYERS[0].get_score() {
                    winner.push(&PLAYERS[i]);
                } else if PLAYERS[i].get_score() == PLAYERS[0].get_score() {
                    tie.push(&PLAYERS[i]);
                } else {
                    lost.push(&PLAYERS[i]);
                }
            }
        }
        println!("win: {}, tie: {}, lost: {}", winner.len(), tie.len(), lost.len());
        for i in 0..winner.len() {
            end.push_str(winner[i].to_string().as_str());
            if i as i8 == winner.len() as i8 - 2 {
                end.push_str(", and ");
            } else if i != winner.len() - 1 {
                end.push_str(", ");
            }
        }
        if winner.len() != 0 {
            end.push_str(" won!\t:)\t");
        }
        for i in 0..tie.len() {
            end.push_str(tie[i].to_string().as_str());
            if i as i8 == tie.len() as i8 - 2 {
                end.push_str(", and ");
            } else if i != tie.len() - 1 {
                end.push_str(", ");
            }
        }
        if tie.len() != 0 {
            end.push_str(" tied with Dealer!\t:|\t");
        }
        for i in 0..lost.len() {
            end.push_str(lost[i].to_string().as_str());
            if i as i8 == lost.len() as i8 - 2 {
                end.push_str(", and ");
            } else if i != lost.len() - 1 {
                end.push_str(", ");
            }
        }
        if lost.len() != 0 {
            end.push_str(" lost!\t:(");
        }
        bot(end.as_str())
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    } else if respond == 100 {
        let initial_cards = PLAYERS[ITERATOR + 1].get_hand();
        bot(format!("{}'s cards are:\n{} and {} ({} points)\tBLACKJACK!\t(Type anything to continue)", 
        PLAYERS[ITERATOR], 
        initial_cards[0], 
        initial_cards[1], 
        PLAYERS[ITERATOR + 1].get_score()).as_str())
    } else if respond == 110 {
        bot(format!("Now, {}'s cards are: {} ({} points)\tYOU GOT 21 POINTS!\t(Type anything to continue)", 
        PLAYERS[ITERATOR], 
        show_cards(&PLAYERS[ITERATOR]), 
        PLAYERS[ITERATOR].get_score()).as_str())
    } else if respond == 120 {
        bot(format!("Now, {}'s cards are: {} ({} points)\tBUST :( {} lost!\t\t(Type anything to continue)",
        PLAYERS[ITERATOR], 
        show_cards(&PLAYERS[ITERATOR]), 
        PLAYERS[ITERATOR].get_score(), 
        PLAYERS[ITERATOR]).as_str())
    } else if respond == 130 {
        bot(format!("{} stood\t(Type anything to continue)", PLAYERS[ITERATOR]).as_str())
    } else if respond == 140 {
        bot(format!("Dealer's cards are: {} ({} points)\tDealer stood\t(Type anything to continue)", 
        show_cards(&PLAYERS[0]), 
        PLAYERS[0].get_score()).as_str())
    } else if respond == 150 {
        bot(format!("Dealer's cards are: {} ({} points)\tBUST :(\t\t(Type anything to continue)", 
        show_cards(&PLAYERS[0]), 
        PLAYERS[0].get_score()).as_str())
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    } else if respond == 200 {
        bot("The number of decks must be between 6 and 8\t(Enter \"quit\" to leave the game)")
    } else if respond == 210 {
        bot("Expect integer input\t(Enter \"quit\" to leave the game)")
    } else if respond == 220 {
        bot("The number of players must be between 1 and 7\t(Enter \"quit\" to leave the game)")
    } else if respond == 230 {
        bot("Invalid command!\tAvailable Commands: (h)it, (s)tand\t(Enter \"quit\" to leave the game)")
    } else {
        bot(format!("Error! STATE: {}\tRESPOND: {}\t Please restart program.", STATE, RESPOND).as_str())
    }}
}

fn bot(str: &str) -> Event {
    let send = Message{room: String::from("lobby"),
                               username: String::from("bot"),
                               message: String::from(str)};
    return Event::json(&send);
}

fn show_cards(player: &Player) -> String {
    let mut str = player.get_hand()[0].to_string();
    for i in 1..player.get_hand().len() {
        str.push_str(", ");
        str.push_str(player.get_hand()[i].to_string().as_str());
    }
    return str.to_string();
}

#[launch]
fn rocket() -> _ {
    unsafe { PLAYERS.push(Player::new(String::from("Dealer")));
    println!("number of players: {}", PLAYERS.len()); }
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events, game])
        .mount("/", FileServer::from(relative!("static")))
}
