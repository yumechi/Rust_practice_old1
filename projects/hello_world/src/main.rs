fn samples1() {
    let x1 = 5;
    let (x2, y2) = (1, 2);
    let x3: i32 = 5;
    let mut x4 = 7;
    // イミュータブルなので変更不可
    // x1 = 10;
    x4 = 57;

    println!("{}", x1);
    println!("{} {}", x2, y2);
    println!("{}", x3);
    println!("{}", x4);

    // 初期化しないとCE
    // let x: i32;
    // println!("The value of x is: {}", x);

    let x5: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x5, y);
    }
    // スコープ外なのでCE
    // println!("The value of x is {} and value of y is {}", x5, y); // これは動きません

    // シャドーイング
    {
        let x: i32 = 8;
        {
            println!("{}", x); // "8"をprint
            let x = 12;
            println!("{}", x); // "12"をprint
        }
        println!("{}", x); // "8"をprint
        let x = 42;
        println!("{}", x); // "42"をprint
    }
}

fn foo() {}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn main() {
    println!("Hello, World!");
    samples1();
    print_number(add_one(5));
    print_sum(19, 3);

    // let x: i32 = diverges();
    // let x: String = diverges();
    // diverges();
    let f: fn(i32) -> i32 = add_one;
    let f = add_one;
    println!("{}", f(5));
}
