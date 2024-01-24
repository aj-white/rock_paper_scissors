use crate::hands::Hand;

#[derive(Debug)]
pub enum GameOutcome {
    Win,
    Lose,
    Draw
}

impl std::fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            GameOutcome::Win => write!(f, "✨ Win ✨"),
            GameOutcome::Lose => write!(f, "😭 Lose 😭"),
            GameOutcome::Draw => write!(f, "🤷‍♂️ Draw 🤷‍♀️"),
        }
    }
}

pub struct GameRound {
    comp: Hand,
    player: Hand,
}

impl GameRound {
    pub fn new(comp: Hand, player: Hand) -> Self {
        GameRound{
            comp: comp,
            player: player
        }
    }

    pub fn play_game(&self) -> GameOutcome {
        match *self {
            GameRound {comp: Hand::Rock, player: Hand::Scissors} => GameOutcome::Lose,
            GameRound {comp: Hand::Rock, player: Hand::Paper} => GameOutcome::Win,
            GameRound {comp: Hand::Rock, player: Hand::Rock} => GameOutcome::Draw,

            GameRound {comp: Hand::Paper, player: Hand::Rock} => GameOutcome::Lose,
            GameRound {comp: Hand::Paper, player: Hand::Scissors} => GameOutcome::Win,
            GameRound {comp: Hand::Paper, player: Hand::Paper} => GameOutcome::Draw,

            GameRound {comp: Hand::Scissors, player: Hand::Paper} => GameOutcome::Lose,
            GameRound {comp: Hand::Scissors, player: Hand::Rock} => GameOutcome::Win,
            GameRound {comp: Hand::Scissors, player: Hand::Scissors} => GameOutcome::Draw,
        }
    }
}