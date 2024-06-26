use embedded_cotoutf8::COtoUTF8;

fn main() {
    let mut num: f32 = 103.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "+1.0300E2");
    num = f32::INFINITY;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    assert_eq!(result, "+INFINITY");
}
