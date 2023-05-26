fn main() {
    // Data types 1. Scalar 2. Compound
    //  1. Scalar data types learn 01 Variable section

    // Compound data 2 types
    // -> touple
    // -> Array

    // touple 
    let tup = (1,2,3);

   //  println!("{}", tup); this way touple not format {integer } {integer}
   let (a, b, c) = tup;
   println!("{} {} {}", a, b, c);

   // various type data in touple
   let tup2:(u32, bool, f32) = (34, true, 2.34);
   let (a, b, c) = tup2;
   println!("{} {} {}", a, b, c);

   // array

   let mut ar = [1,3,5,7]; // array defalut immutable

   ar[0] = 100;

   println!("{} {}", ar[0], ar[1]);
}
