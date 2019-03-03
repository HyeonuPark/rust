// ignore-cloudabi no std::fs support

// Test that we do some basic error correction in the tokeniser (and don't spew
// too many bogus errors).

pub mod raw {
    use std::{io, fs};
    use std::path::Path;

    pub fn ensure_dir_exists<P: AsRef<Path>, F: FnOnce(&Path)>(path: P,
                                                               callback: F)
                                                               -> io::Result<bool> {
        if !is_directory(path.as_ref()) {
            //~^ ERROR cannot find function `is_directory`
            callback(path.as_ref();
            //~^ ERROR expected one of
            //~| ERROR this function takes 1 parameter but 2 parameters were supplied
            fs::create_dir_all(path.as_ref()).map(|()| true)
        } else {
            //~^ ERROR incorrect close delimiter: `}`
            Ok(false);
        }

        panic!();
    }
}

fn main() {}
