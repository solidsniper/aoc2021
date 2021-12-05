use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
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
    let vec = read(File::open(
        "/home/alex/Documents/aoc/01/aoc01/src/input.txt",
    )?)?;
    let mut i: i32 = 0;
    for (pos, e) in vec.iter().enumerate() {
        if vec.get(pos + 1) > Some(e){
          i = i + 1;
          // println!("{}: {:?}, increased", pos, e);
        } else {
          // println!("{}: {:?}, decreased", pos, e);
        }
    }

    println!("{}", i);
    Ok(())
}
