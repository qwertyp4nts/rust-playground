fn main() {
    let answer = 42;
    println!("The meaning of life is {}", answer);

    // Another very useful macro is assert_eq!. This is the workhorse of testing in Rust; you assert that two things must be equal, and if not, panic.
    assert_eq!(answer,42);
}
// Spelling mistakes are compile errors, not runtime errors like with dynamic languages like Python or JavaScript.
//  if I wrote 'answr' instead of 'answer', the compiler is actually nice about it