enum EnumEx {
    A,
    B,
    C(u8),
    D { x: u8, y: u8 },
}

pub fn run() {
    let en1 = EnumEx::C(5);
    let en2 = EnumEx::D { x: 1, y: 2 };
    let x = 10;
    sampleMatch(en1);
    sampleMatch(en2);

    let z1 = None::<i32>;
    let z2: Option<i32> = None;
    sampleOption(Some(x));

    let input: i32 = 13;
    match h(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    if let Ok(i) = h(input) {
        println!("if let example, i: {}", i)
    };
}

fn sampleMatch(ex: EnumEx) {
    match ex {
        EnumEx::A | EnumEx::B => println!("EnumEx: A or B"),
        EnumEx::C(x) => println!("EnumEx: C - {}", x),
        EnumEx::D { x, y } => println!("EnumEx: D - [{}, {}]", x, y),
    }
}

fn sampleOption(nmb: Option<i32>) {
    match nmb {
        None => println!("None"),
        Some(x) => println!("{}", x),
    }
}

fn h(i: i32) -> Result<i32, String> {
    match i {
        i if i >= 0 => Ok(i + 10),
        _ => Err(format!("Input to h less that 0, found: {}", i)),
    }
}
