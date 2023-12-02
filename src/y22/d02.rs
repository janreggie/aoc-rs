use anyhow::{bail, Context, Result};
use sscanf::sscanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl Token {
    fn new(c: char) -> Result<Token> {
        match c {
            'A' => Ok(Token::A),
            'B' => Ok(Token::B),
            'C' => Ok(Token::C),
            'X' => Ok(Token::X),
            'Y' => Ok(Token::Y),
            'Z' => Ok(Token::Z),
            _ => bail!("invalid char {}", c),
        }
    }
}

enum Opponent {
    A,
    B,
    C,
}

impl Opponent {
    fn new(t: &Token) -> Result<Opponent> {
        match t {
            Token::A => Ok(Opponent::A),
            Token::B => Ok(Opponent::B),
            Token::C => Ok(Opponent::C),
            _ => bail!("invalid token {:?}", t),
        }
    }
}

enum Player {
    X,
    Y,
    Z,
}

impl Player {
    fn new(t: &Token) -> Result<Player> {
        match t {
            Token::X => Ok(Player::X),
            Token::Y => Ok(Player::Y),
            Token::Z => Ok(Player::Z),
            _ => bail!("invalid token {:?}", t),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let tokens = lines
        .iter()
        .map(|line| {
            sscanf!(line, "{} {}", char, char)
                .ok()
                .with_context(|| format!("could not parse line {}", line))
        })
        .collect::<Result<Vec<(char, char)>>>()?
        .iter()
        .map(|(c1, c2)| (Token::new(*c1), Token::new(*c2)))
        .map(|(t1, t2)| match (t1, t2) {
            (Ok(t1), Ok(t2)) => match (Opponent::new(&t1), Player::new(&t2)) {
                (Ok(opponent), Ok(player)) => Ok((opponent, player)),
                _ => bail!("invalid tokens {:?} {:?}", t1, t2),
            },
            (r1, r2) => bail!("invalid tokens {:?} {:?}", r1, r2),
        })
        .collect::<Result<Vec<(Opponent, Player)>>>()?;

    // Part 1: X: rock, Y: paper, Z: scissors
    let ans1 = tokens
        .iter()
        .map(|(opponent, player)| match (opponent, player) {
            (Opponent::A, Player::X) => 1 + 3,
            (Opponent::A, Player::Y) => 2 + 6,
            (Opponent::A, Player::Z) => 3 + 0,
            (Opponent::B, Player::X) => 1 + 0,
            (Opponent::B, Player::Y) => 2 + 3,
            (Opponent::B, Player::Z) => 3 + 6,
            (Opponent::C, Player::X) => 1 + 6,
            (Opponent::C, Player::Y) => 2 + 0,
            (Opponent::C, Player::Z) => 3 + 3,
        })
        .sum::<u32>();
    let ans1 = Ok(ans1.to_string());

    // Part 2: X you lose, Y you draw, Z you win
    let ans2 = tokens
        .iter()
        .map(|(opponent, player)| match (opponent, player) {
            (Opponent::A, Player::X) => 3 + 0,
            (Opponent::A, Player::Y) => 1 + 3,
            (Opponent::A, Player::Z) => 2 + 6,
            (Opponent::B, Player::X) => 1 + 0,
            (Opponent::B, Player::Y) => 2 + 3,
            (Opponent::B, Player::Z) => 3 + 6,
            (Opponent::C, Player::X) => 2 + 0,
            (Opponent::C, Player::Y) => 3 + 3,
            (Opponent::C, Player::Z) => 1 + 6,
        })
        .sum::<u32>();
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
