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

fn main() {
    println!("Hello, World!");
    samples1();
}
