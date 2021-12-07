use itertools::Itertools;
static INPUT: &str = include_str!("input.txt");

fn main() {
  let mut words: Vec<&str> = INPUT.lines().filter(|c| *c != "").collect();
  let input = words.first().cloned().unwrap();
  let input: Vec<u8> = input
    .split(",")
    .map(|x| x.parse::<u8>().unwrap())
    .collect();
  words.remove(0);
  let mut board_index: usize = 0;
  let mut board = [[[0u8; 5]; 5]; 100];
  for (pos, s) in words.iter().enumerate() {
    let split = s.split(" ").filter(|c| *c != "");
    let mut index: u8 = 0;
    for (po, e) in split.enumerate() {
      board[board_index][po][pos % 5] = e.to_string().parse::<u8>().unwrap();
      if (pos % 5) + po == 8 {
        board_index += 1;
      }
    }

  }
  println!("{:?}", board);
  // println!("{}", board[0][2][0]);
}
