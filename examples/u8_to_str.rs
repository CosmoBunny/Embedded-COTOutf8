use embedded_cotoutf8::COtoUTF8;

fn main() {
    let num: u8 = 27;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "27")
}
