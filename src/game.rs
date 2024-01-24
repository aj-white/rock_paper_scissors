use crate::hands::Hand;

#[derive(Debug, PartialEq)]
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
            Self {comp: Hand::Rock, player: Hand::Scissors} => GameOutcome::Lose,
            Self {comp: Hand::Rock, player: Hand::Paper} => GameOutcome::Win,
            Self {comp: Hand::Paper, player: Hand::Rock} => GameOutcome::Lose,
            Self {comp: Hand::Paper, player: Hand::Scissors} => GameOutcome::Win,
            Self {comp: Hand::Scissors, player: Hand::Paper} => GameOutcome::Lose,
            Self {comp: Hand::Scissors, player: Hand::Rock} => GameOutcome::Win,
            _ => GameOutcome::Draw,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Hand::*;
    use super::GameRound;
    use super::GameOutcome::*;

    #[test]
    fn play_game() {
        assert_eq!(GameRound::new(Rock, Paper).play_game(), Win);
        assert_eq!(GameRound::new(Rock, Scissors).play_game(), Lose);
        assert_eq!(GameRound::new(Rock, Rock).play_game(), Draw);
        assert_eq!(GameRound::new(Paper, Scissors).play_game(), Win);
        assert_eq!(GameRound::new(Paper, Rock).play_game(), Lose);
        assert_eq!(GameRound::new(Paper, Paper).play_game(), Draw);
        assert_eq!(GameRound::new(Scissors, Rock).play_game(), Win);
        assert_eq!(GameRound::new(Scissors, Paper).play_game(), Lose);
        assert_eq!(GameRound::new(Scissors, Scissors).play_game(), Draw);
    }
}