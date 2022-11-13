use scanner::BthScanner;
use rfcomm::RFCOMM;

mod scanner;
mod rfcomm;
mod error;
mod types;
mod util;


fn main() {
    let results = BthScanner::new().scan();
    results.iter().for_each(|device| println!("{}", device));
    let rfcomm = RFCOMM::new().create_rfcomm_socket().unwrap();
    println!("Hello, world!");
    
}
