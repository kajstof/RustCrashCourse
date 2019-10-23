enum EnumEx {
    A,
    B,
    C(u8),
    D { x: u8, y: u8 },
}

pub fn run() {
    let en1 = EnumEx::C(5);
    let en2 = EnumEx::D { x: 1, y: 2 };
    let mut en3 = EnumEx::A;
    let x = 10;
    sample_match(en1);
    sample_match(en2);
    sample_match(en3);
    en3 = EnumEx::B;
    sample_match(en3);

    let _z1 = None::<i32>;
    let _z2: Option<i32> = None;
    sample_option(Some(x));

    let input: i32 = 13;
    match h(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    if let Ok(i) = h(input) {
        println!("if let example, i: {}", i)
    };

    let mut _sum = 0.0;
    for i in 0..5 {
        _sum += i as f64;
    }

    let res = sqr(2 as f64);
    println!("Square is {}", res);

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("by_ref: {} {}", res1, res2);

    let abc: i32 = 222;
    println!("{}", abc);
}

fn sample_match(ex: EnumEx) {
    match ex {
        EnumEx::A | EnumEx::B => println!("EnumEx: A or B"),
        EnumEx::C(x) => println!("EnumEx: C - {}", x),
        EnumEx::D { x, y } => println!("EnumEx: D - [{}, {}]", x, y),
    }
}

fn sample_option(nmb: Option<i32>) {
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

fn sqr(x: f64) -> f64 {
    x * x
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}
