use std::cmp::{max, min};
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Combat {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

struct RecursiveCombat {
    current_state: Combat,
    prev_states: Vec<Combat>,
}

impl Combat {
    fn parse(s: &str) -> Result<Combat, util::Error> {
        let mut sections = s.split("\n\n");
        Ok(Combat {
            player1: Self::parse_player(
                sections.next().ok_or(util::Error::NoneError)?,
            )?,
            player2: Self::parse_player(
                sections.next().ok_or(util::Error::NoneError)?,
            )?,
        })
    }

    fn parse_player(s: &str) -> Result<VecDeque<u8>, util::Error> {
        Ok(s.lines()
            .skip(1)
            .map(|line| line.parse::<u8>())
            .collect::<Result<VecDeque<_>, _>>()?)
    }

    fn do_round(&mut self) -> Result<(), util::Error> {
        if self.player1.is_empty() || self.player2.is_empty() {
            Err(util::Error::GameFinished)
        } else {
            let card1 = self.player1.pop_front().unwrap();
            let card2 = self.player2.pop_front().unwrap();

            let victor = if card1 > card2 {
                &mut self.player1
            } else {
                &mut self.player2
            };
            victor.push_back(max(card1, card2));
            victor.push_back(min(card1, card2));

            Ok(())
        }
    }

    fn score(&self) -> Result<u64, util::Error> {
        if self.player1.is_empty() || self.player2.is_empty() {
            let victor = if self.player1.is_empty() {
                &self.player2
            } else {
                &self.player1
            };
            Ok(victor
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &card)| ((i + 1) as u64) * (card as u64))
                .sum())
        } else {
            Err(util::Error::GameNotFinished)
        }
    }
}

impl RecursiveCombat {
    fn new(combat: Combat) -> Self {
        Self {
            current_state: combat,
            prev_states: Vec::new(),
        }
    }

    fn matches_previous_state(&self) -> bool {
        self.prev_states
            .iter()
            .any(|state| *state == self.current_state)
    }

    fn do_round(&mut self) -> Option<Player> {
        if self.current_state.player1.is_empty() {
            return Some(Player::Player2);
        } else if self.current_state.player2.is_empty()
            || self.matches_previous_state()
        {
            return Some(Player::Player1);
        }

        self.prev_states.push(self.current_state.clone());

        let card1 = self.current_state.player1.pop_front().unwrap();
        let card2 = self.current_state.player2.pop_front().unwrap();

        let round_winner = self.round_winner(card1, card2);
        match round_winner {
            Player::Player1 => {
                self.current_state.player1.push_back(card1);
                self.current_state.player1.push_back(card2);
            }
            Player::Player2 => {
                self.current_state.player2.push_back(card2);
                self.current_state.player2.push_back(card1);
            }
        };

        None
    }

    fn game_winner(&mut self) -> Player {
        loop {
            let round = self.do_round();
            if let Some(winner) = round {
                return winner;
            }
        }
    }

    fn round_winner(&self, card1: u8, card2: u8) -> Player {
        let card1 = card1 as usize;
        let card2 = card2 as usize;

        if (card1 <= self.current_state.player1.len())
            && (card2 <= self.current_state.player2.len())
        {
            // Recursive game
            let mut subgame = RecursiveCombat {
                current_state: Combat {
                    player1: self
                        .current_state
                        .player1
                        .iter()
                        .take(card1)
                        .copied()
                        .collect(),
                    player2: self
                        .current_state
                        .player2
                        .iter()
                        .take(card2)
                        .copied()
                        .collect(),
                },
                prev_states: Vec::new(),
            };
            subgame.game_winner()
        } else {
            // Not enough cards for a recursive game
            if card1 > card2 {
                Player::Player1
            } else {
                Player::Player2
            }
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let mut combat = Combat::parse(&text)?;

    while combat.do_round().is_ok() {}

    println!("Part 1, score = {}", combat.score()?);

    let combat = Combat::parse(&text)?;
    let mut recur = RecursiveCombat::new(combat);

    recur.game_winner();
    println!("Part 2, score = {}", recur.current_state.score()?);

    Ok(())
}
