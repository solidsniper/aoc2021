use itertools::Itertools;
static INPUT: &str = include_str!("input.txt");

fn main() {
  let words: Vec<&str> = INPUT.split('\n').collect();
  let mut x = words.len() / 2;
  let mut t: u32 = x.try_into().unwrap();
  let mut xs: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  for pos in words.iter() {
    for (i, o) in pos.chars().enumerate() {
      xs[i] += o.to_digit(10).unwrap();
      // println!("{}", x)
    }
  }
  let mut bytestring: String = "".to_owned();
  xs.iter_mut().for_each(|x| {
    if x > &mut t {
      *x = 1;
      bytestring.push_str("1");
    } else {
      *x = 0;
      bytestring.push_str("0");
    }
  });
  println!("{:?}", xs);
  let mut t: u32 = 0b000100011100;
  let mut y: u32 = 0b111011100011;
  println!("{},{},{}", t,y,t*y);
}