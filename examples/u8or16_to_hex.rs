use embedded_cotoutf8::COtoHex;

fn main() {
    let (x, y): (u8, u16) = (0xFA, 0x55AA);
    println!(
        "x to hex is {}",
        core::str::from_utf8(&x.coto_hex()).unwrap()
    );
    println!(
        "y to hex is {}",
        core::str::from_utf8(&y.coto_hex()).unwrap()
    );
    println!("{:?}, {:?}", x.coto_hex(), y.coto_hex())
}
