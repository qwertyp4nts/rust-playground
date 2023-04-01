fn main() {
    // The range is not inclusive, so i goes from 0 to 4. This is convenient in a language which indexes things like arrays from 0.
    for i in 0..5 {
        println!("Number {}", i);
    }

// i % 2 is zero if 2 can divide into i cleanly; Rust uses C-style operators.
// There are no brackets around the condition, just like in Go, but you must use curly brackets around the block.
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }

// This does the same as above, written in a more interesting way
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }

// let variables by default can only be assigned a value when declared. Adding the magic word mut (please make this variable mutable) does the trick
    let mut sum = 0;
    for j in 0..5 {
        sum += j;
        println!("sum= {}", sum);
    }

    let mut floatsum = 0.0;
    for k in 0..5 {
        floatsum += k as f64; // Rust likes to be explicit - it will not silently convert that integer into a float for you. We have to cast that value to a floating-point value explicitly.
        println!("sum= {}", floatsum);
    }
}

// Rust is both statically-typed and strongly-typed - these are often confused, but think of C (statically but weakly typed),
// and Python (dynamically but strongly typed). In static types the type is known at compile time, and dynamic types are only known at run time.