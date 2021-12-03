use std::error::Error;
use regex::Regex;

const TEST: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[derive(Debug)]
enum Command {
  Forward(i32),
  Down(i32),
  Up(i32),
}

impl From<&str> for Command {
  fn from(s: &str) -> Command {
    let re = Regex::new(r"^(?P<command>[a-z]+) (?P<number>[0-9]+)").unwrap();
    let cap = re.captures(s).unwrap();
    let command = cap.name("command").unwrap().as_str();
    let number = cap.name("number").map(|m| i32::from_str_radix(m.as_str(), 10).unwrap()).unwrap();
    match command {
      "forward" => Command::Forward(number),
      "up" => Command::Up(number),
      "down" => Command::Down(number),
      _ => panic!("unknown command {}", command),
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let input = lib::read_input(2, TEST)?;
  let lines: Vec<&str> = input
    .split_terminator("\n")
    .collect();

  let mut horiz1 = 0;
  let mut depth1 = 0;
  let mut horiz2 = 0;
  let mut depth2 = 0;
  let mut aim2 = 0;
  for l in lines {
    match l.into() {
      Command::Forward(n) => {horiz1 += n; horiz2 += n; depth2 += aim2 * n; }
      Command::Down(n) => {depth1 += n; aim2 += n},
      Command::Up(n) => {depth1 -= n; aim2 -= n},
    };
  }
  println!("Part 1: {}", horiz1 * depth1);
  println!("Part 2: {}", horiz2 * depth2);

  Ok(())
}
