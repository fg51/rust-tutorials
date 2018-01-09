use std::error::Error;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("out.txt");
    let fout = match File::create(&path) {
        Err(why) => panic!(
            "couldn't create {}: {}",
            path.display(),
            Error::description(&why)
        ),
        Ok(file) => file,
    };
    let mut fout = BufWriter::new(fout);

    let txt = b"hello rust\n";
    for _ in 0..2 {
        match fout.write(txt) {
            Err(why) => panic!(
                "couldn't write to {}: {}",
                path.display(),
                Error::description(&why)
            ),
            Ok(_) => (),
        };
    }
}
