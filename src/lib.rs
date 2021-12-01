pub fn read_input(day: i32, test_input: &str) -> Result<String, std::io::Error> {
  let is_test = std::env::args()
    .next_back()
    .filter(|s| s == "test")
    .is_some();
  if is_test {
    Ok(String::from(test_input))
  } else {
    std::fs::read_to_string(format!("src/bin/day{}.txt", day))
  }}
