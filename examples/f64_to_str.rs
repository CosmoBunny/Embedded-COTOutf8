use embedded_cotoutf8::{COtoUTF8, DebugODisplay};

fn main() {
    let num: f64 = 1.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f64 = -123.0005;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f64 = 1524.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f64 = 44245.12;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    let num: f64 = 442400005.001;
    let binding = num.coto_utf8();
    let result = core::str::from_utf8(&binding).unwrap();
    println!("{}", result);

    // Alternative use this
    let num = DebugODisplay(44245.12f64);
    println!("{}", num)
}
