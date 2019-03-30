extern crate daedalus;

use daedalus::Generator;

fn main() {
    let map = Generator::new().generate();
    print!("{}", map);
}
