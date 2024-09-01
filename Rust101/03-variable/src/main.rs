fn main() {
    let x = 4; // x is immutable
    println!("x is {}", x);

    let mut y = 5; // y is mutable
    println!("y is {}", y);
    y = 6;
    println!("y is changed to {}", y);
    // y = "yes"; // will show error

    // let z = 7; // will show warning if not used
    {
        let x = 7; // x is shadowed
        println!("x is shadowed to {}", x);
    }
    println!("x is {}", x);

    let x = 8; // shadowing
    println!("x is shadowed to {}", x);

    const MAX_POINTS: u32 = 100_000; // constant

    // const MAX_POINTS: u32 = 200_000; // shadowing will fail
    println!("MAX_POINTS is {}", MAX_POINTS);
}
