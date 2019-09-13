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
    let z1 = None::<i32>;
    let z2: Option<i32> = None;

    sampleMatch(en1);
    sampleMatch(en2);
    sampleOption(Some(x));
}

fn sampleMatch(ex: EnumEx) {
    match ex {
        EnumEx::A | EnumEx::B => println!("EnumEx: A or B"),
        EnumEx::C(x) => println!("EnumEx: C - {}", x),
        EnumEx::D { x, y } => println!("EnumEx: D - [{}, {}]", x, y),
    }
}

fn sampleOption(nmb: Option<i32>){
    match nmb{
        None => println!("None"),
        Some(x) => println!("{}", x),
    }
}
