// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/*
Orphan rule:
Things get more nuanced when multiple crates are involved. In particular, at least one of the following must be true:

The trait is defined in the current crate
The implementor type is defined in the current crate
This is known as Rust's orphan rule. Its goal is to make the method resolution process unambiguous.

Imagine the following situation:

Crate A defines the IsEven trait
Crate B implements IsEven for u32
Crate C provides a (different) implementation of the IsEven trait for u32
Crate D depends on both B and C and calls 1.is_even()
Which implementation should be used? The one defined in B? Or the one defined in C?
There's no good answer, therefore the orphan rule was defined to prevent this scenario. Thanks to the orphan rule, neither crate B nor crate C would compile.

Further reading
There are some caveats and exceptions to the orphan rule as stated above.
Check out the reference if you want to get familiar with its nuances.
https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
*/

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }
