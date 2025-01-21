/*
مهم خیلی

Dynamically sized types
str is a dynamically sized type (DST).
A DST is a type whose size is not known at compile time.
Whenever you have a reference to a DST, like &str, it has to include additional information about the data it points to.
It is a "fat pointer".
In the case of &str, it stores the length of the slice it points to.
We'll see more examples of DSTs in the rest of the course.

The Sized trait
Rust's std library defines a trait called Sized.

pub trait Sized {
    // This is an empty trait, no methods to implement.
}
A type is Sized if its size is known at compile time. In other words, it's not a DST.

Marker traits
Sized is your first example of a marker trait.
A marker trait is a trait that doesn't require any methods to be implemented.
It doesn't define any behavior. It only serves to mark a type as having certain properties.
The mark is then leveraged by the compiler to enable certain behaviors or optimizations.

Auto traits
In particular, Sized is also an auto trait.
You don't need to implement it explicitly; the compiler implements it automatically for you based on the type's definition.

Examples
All the types we've seen so far are Sized: u32, String, bool, etc.

str, as we just saw, is not Sized.
&str is Sized though! We know its size at compile time: two usizes, one for the pointer and one for the length.

*/

pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
   // std::mem::size_of::<str>();
}
