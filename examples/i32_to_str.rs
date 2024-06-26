use embedded_cotoutf8::COtoUTF8;

fn main() {
    let num: i32 = -189434527;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "-0189434527")
}
