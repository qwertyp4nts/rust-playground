// Functions are one place where the compiler will not work out types for you.
// And this in fact was a deliberate decision, since languages like Haskell have 
// such powerful type inference that there are hardly any explicit type names.
// It's actually good Haskell style to put in explicit type signatures for functions.
// Rust requires this always.

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
}

// Rust goes back to an older style of argument declaration, where the type follows the name
// This is how it was done in Algol-derived languages like Pascal.
fn sqr(x: f64) -> f64 {
    return x * x;
}