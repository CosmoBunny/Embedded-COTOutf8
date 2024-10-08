use embedded_cotoutf8::COtoUTF8;

fn main() {
    let num: f32 = 1.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);
    let num: f32 = -123.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);
    let num: f32 = 152345435.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);
}
