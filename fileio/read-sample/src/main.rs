use std::error::Error;
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, mem};

fn main() {
    let path = Path::new("inn.txt");
    let mut finn = BufReader::new(match fs::File::open(path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            path.display(),
            Error::description(&why)
        ),
        Ok(file) => file,
    });

    let mut b: [u8; 4] = unsafe { mem::uninitialized() };
    for _ in 0..2 {
        // finn.read_exact(&mut b).unwrap();
        match finn.read_exact(&mut b) {
            Err(why) => panic!(
                "couldn't read {}: {}",
                path.display(),
                Error::description(&why)
            ),
            Ok(_) => println!("{} contains:\n {:?}", path.display(), b),
        }
    }
}
