use std::fmt;

#[derive(Debug, Clone)]
struct Phrase {
    symbols: Vec<char>,
    visibility: Vec<bool>,
    visible_count: usize,
}

impl Phrase {
    fn new(s: &str) -> Self {
        let symbols: Vec<char> = s.chars().collect();
        let mut visible_count = 0;
        let visibility = symbols
            .iter()
            .map(|ch| {
                let is_visible = !ch.is_ascii_alphanumeric();
                if is_visible {
                    visible_count += 1;
                }
                is_visible
            })
            .collect();
        Self {
            symbols,
            visibility,
            visible_count,
        }
    }

    fn has_char(&self, ch: &char) -> bool {
        self.symbols
            .iter()
            .any(|sym| sym.to_lowercase().to_string() == ch.to_lowercase().to_string())
    }

    fn mark_visibility(&mut self, ch: &char) -> usize {
        let mut marked = 0;
        self.symbols
            .iter()
            .enumerate()
            .filter(|(_, sym)| ch.to_lowercase().to_string() == sym.to_lowercase().to_string())
            .for_each(|(i, _)| {
                self.visibility[i] = true;
                self.visible_count += 1;
                marked += 1;
            });
        marked
    }

    fn is_completely_visible(&self) -> bool {
        self.symbols.len() == self.visible_count
    }

    fn set_visible(&mut self) {
        self.visibility.iter_mut().for_each(|v| *v = true);
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, ch) in self.symbols.iter().enumerate() {
            if self.visibility[idx] {
                write!(f, "{}", ch)?;
            } else {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Guess {
    Good,
    Bad,
    Used,
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Good => write!(f, "Good guess! Make another"),
            Self::Bad => write!(f, "That's not it"),
            Self::Used => write!(f, "Already guessed this"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    PreStart,
    InProgress(Guess),
    Won,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Game {
    phrase: Phrase,
    guesses: Vec<char>,
    remaining_chances: u8,
    state: GameState,
}

impl Game {
    pub fn new(phrase: &str) -> Self {
        let phrase = Phrase::new(phrase);
        let remaining_chances = 7;
        let state = GameState::PreStart;
        Self {
            phrase,
            guesses: vec![],
            remaining_chances,
            state,
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn guess_char(&mut self, guess: &char) -> GameState {
        match self.state {
            GameState::Lost | GameState::Won => (),
            GameState::PreStart | GameState::InProgress(_) => {
                if self.guesses.contains(guess) {
                    self.state = GameState::InProgress(Guess::Used);
                } else if self.phrase.has_char(guess) {
                    self.phrase.mark_visibility(guess);
                    self.guesses.push(*guess);
                    if self.phrase.is_completely_visible() {
                        self.state = GameState::Won;
                    } else {
                        self.state = GameState::InProgress(Guess::Good);
                    }
                } else {
                    self.guesses.push(*guess);
                    self.remaining_chances -= 1;
                    if self.remaining_chances == 0 {
                        self.phrase.set_visible();
                        self.state = GameState::Lost;
                    } else {
                        self.state = GameState::InProgress(Guess::Bad);
                    }
                }
            }
        }
        return self.state;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.phrase)?;
        write!(f, "Remaining guesses: {}\n", self.remaining_chances)?;
        match self.state {
            GameState::Won => write!(f, "You won!"),
            GameState::Lost => write!(f, "You lost!"),
            GameState::InProgress(g) => write!(f, "{}", g),
            GameState::PreStart => write!(f, "Ready to start?"),
        }
    }
}
