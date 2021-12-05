use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

// fn read2<R: Read>(io: R) -> Result<Vec<i64>, Error> {
//     let br = BufReader::new(io);
//     let mut v = vec![];
//     for line in br.lines() {
//         v.push(
//             line?
//                 .trim()
//                 .parse()
//                 .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
//         );
//     }
//     Ok(v)
// }

fn main() -> Result<(), Error> {
    let lines = read(File::open(
        "/home/alex/Documents/aoc/01/aoc01/src/02/input.txt",
    )?)?;
    let mut hor: u32 = 0;
    let mut ver: u32 = 0;
    for line in lines.iter() {
        let dir = line.chars().nth(0).unwrap();
        let value = line.chars().last().unwrap();
        let value: u32 = value.to_digit(10).unwrap();
        match dir {
            'd' => {
                hor = hor + value;
            }
            'u' => {
                hor = hor - value;
            }
            _ => ver = ver + value,
        }
    }
    println!("{},{}", hor, ver);
    println!("{}", hor * ver);
    Ok(())
}
