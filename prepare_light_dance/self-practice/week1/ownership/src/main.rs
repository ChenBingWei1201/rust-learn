fn main() {
    let x = String::from("Hello, world!");
    let y = x.clone();  // value is "cloned" instead of "moved"
    println!("x: {x}");  // still available
    println!("y: {y}");

    let z = 1000;
    let w = z;  // value of x is automatically cloned to y
    println!("z: {z}");  // this will pass instead
    println!("w: {w}");

    let a = &x;
    print!("a: {a}");

}
