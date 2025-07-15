fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("value of x is {x}");
    x = 10;
    println!("New value of x is {x}");

    const a: u32 = 10;
    println!("Value of constant 'a' is: {}", a);

    let b = 10;
    let b = b + 20;
    {
        let b = b * 10;
        println!("Value of b in scope: {}", b);
    }
    println!("Value of b in outside scope is : {}", b);

    let c = "sachin";
    let c = c.len();
    println!("Length of string is {c}");
    println!("string c is {}", c.to_string());
    
    let s = "3.14";
    let f: f64 = s.parse().unwrap();

    let i: i32 = "100".parse().expect("Not a number");
    println!("Value of i is {}", i);
}
