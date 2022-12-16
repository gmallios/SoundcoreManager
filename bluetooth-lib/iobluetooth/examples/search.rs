use iobluetooth::scan;


fn main () {
    let res = scan();
    println!("{:?}", res);
}