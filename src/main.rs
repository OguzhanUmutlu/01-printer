use rand::{Rng, thread_rng};
#[macro_use]
extern crate colour;

fn main() {
    let mut rng = thread_rng();
    loop {
        let termsize::Size { rows: _rows, cols } = termsize::get().unwrap();
        let mut str = String::new();
        for _ in 0..cols {
            str = str + &*rng.gen_range(0..2).to_string()
        }
        green!("{}", &str);
    }
}
