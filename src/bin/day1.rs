use std::error::Error;

const TEST: &str = "199
200
208
210
200
207
240
269
260
263";

fn main() -> Result<(), Box<dyn Error>> {
  let input = lib::read_input(1, TEST)?;
  use std::str::FromStr;
  let lines: Vec<i32> = input
    .split_terminator("\n")
    .filter_map(|s| i32::from_str(s).ok())
    .collect();

  struct IncreaseCounting {
    last: i32,
    count: i32,
  }

  let p1 = lines.iter().fold(None, |state, cur| match state {
    None => Some(IncreaseCounting {
      last: *cur,
      count: 0,
    }),
    Some(IncreaseCounting { last, count }) => Some(IncreaseCounting {
      last: *cur,
      count: if *cur > last { count + 1 } else { count },
    }),
  });
  println!("Part 1: {}", p1.unwrap().count);

  let p2 = lines.windows(3).fold(None, |state, cur| {
    let sum = cur.iter().sum();
    match state {
      None => Some(IncreaseCounting {
        last: sum,
        count: 0,
      }),
      Some(IncreaseCounting { last, count }) => Some(IncreaseCounting {
        last: sum,
        count: if sum > last { count + 1 } else { count },
      }),
    }
  });
  println!("Part 2: {}", p2.unwrap().count);

  Ok(())
}
