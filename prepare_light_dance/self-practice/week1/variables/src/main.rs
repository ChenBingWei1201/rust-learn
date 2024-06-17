fn main() {    
    let y = vec![3, 2, 1];
    let mut c = 0;
    loop {
        println!("y[{c}]: {}", y[c]);
        c += 1;
        if c > 2 {
            break;
        }
    }

    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("spaces: {spaces}");
}