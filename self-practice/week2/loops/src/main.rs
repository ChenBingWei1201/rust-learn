fn main() {
    let mut count = 0;

    let result = 'outer: loop {
            let mut bruh = 3;
    
            loop {
                    bruh -= 1;
                    if count == 2 {
                            break 'outer bruh;
                    }
                    if bruh == 0 {
                            break;
                    }
            }
    
            count += 1;
    };
    
    println!("result: {}", result); // result: 2
}