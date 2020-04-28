use std::fs::File;
use std::env;

use sf::SceneTemplate;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let f = File::open(filename).unwrap();
    let sf = SceneTemplate::from_read(f).unwrap();
    println!("{:#?}", sf);
}