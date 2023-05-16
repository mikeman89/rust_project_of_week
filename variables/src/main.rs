fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let x = 2.0; // f32

    let y: f32 = 3.0; // f32

    println!("The value of x & y are: {x} & {y}");
}
