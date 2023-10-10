#[cfg(target_os = "macos")]
use iobluetooth::scan;

fn main() {
    #[cfg(target_os = "macos")]
    let res = scan();
    #[cfg(target_os = "macos")]
    println!("{:?}", res);
}
