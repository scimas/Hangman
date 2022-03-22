use std::{
    char::ParseCharError,
    env, error, fmt,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use hangman::{Game, GameState};
use rand::{prelude::SliceRandom, thread_rng};

#[derive(Debug, Clone)]
enum InputError {
    Empty,
    Excess,
    Parse(ParseCharError),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::Excess => write!(f, "more than one char in input"),
            Self::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for InputError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Parse(ref e) => Some(e),
            Self::Empty | Self::Excess => None,
        }
    }
}

#[derive(Debug, Clone)]
enum GameError {
    Io(InputError),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for GameError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Io(ref e) => Some(e),
        }
    }
}

impl From<InputError> for GameError {
    fn from(e: InputError) -> Self {
        Self::Io(e)
    }
}

fn main() -> Result<(), GameError> {
    let cur_dir = env::current_dir().expect("can't find current directory");
    let phrases_f = File::open(cur_dir.join("phrases.txt")).expect("can't open pharses asset");
    let phrases_reader = BufReader::new(phrases_f);
    let maybe_phrases: Result<Vec<String>, _> = phrases_reader.lines().collect();
    let phrases = maybe_phrases.expect("couldn't read phrases");
    let mut rng = thread_rng();
    let mut phrase_chooser = phrases.choose_multiple(&mut rng, phrases.len());
    while let Some(phrase) = phrase_chooser.next() {
        let mut game = Game::new(phrase);
        while let GameState::PreStart | GameState::InProgress(_) = game.state() {
            println!("{}\n", game);
            let input = get_input("Make a guess: ").expect("Error on std io");
            match validate_game_input(&input) {
                Ok(ch) => {
                    game.guess_char(&ch);
                }
                Err(e) => {
                    println!("{}", e);
                    println!("Try again");
                }
            }
        }
        println!("{}\n", game);
        if !continue_game() {
            break;
        }
    }
    Ok(())
}

fn get_input(msg: &str) -> io::Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn validate_game_input(input: &str) -> Result<char, InputError> {
    let input = input.trim();
    if input.chars().count() == 0 {
        return Err(InputError::Empty);
    } else if input.chars().count() > 1 {
        return Err(InputError::Excess);
    }
    match input.parse() {
        Ok(ch) => Ok(ch),
        Err(e) => Err(InputError::Parse(e)),
    }
}

fn continue_game() -> bool {
    let mut input = get_input("Another game? [y]/n: ").expect("Error on std io");
    while !["", "y", "n"].contains(&input.trim()) {
        println!("Unrecognized option");
        input = get_input("Another game? [y]/n: ").expect("Error on std io");
    }
    if input.trim() == "n" {
        false
    } else {
        true
    }
}
