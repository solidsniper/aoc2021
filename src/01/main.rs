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
    let mut window: i32 = 0;
    for (pos, e) in vec.iter().enumerate() {
        if vec.get(pos + 1) > Some(e) {
            i = i + 1;
        }
        //#### part 2
        if vec.get(pos + 1).unwrap_or(&0)
            + vec.get(pos + 2).unwrap_or(&0)
            + vec.get(pos + 3).unwrap_or(&0)
            > vec.get(pos).unwrap_or(&0)
                + vec.get(pos + 1).unwrap_or(&0)
                + vec.get(pos + 2).unwrap_or(&0)
        {
            window = window + 1;
        }
    }
    println!("Answer part 1: {}", i);
    println!("Answer part 2: {}", window);
    Ok(())
}
