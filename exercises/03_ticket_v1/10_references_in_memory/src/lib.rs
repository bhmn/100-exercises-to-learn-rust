pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    /*
    Most references(
    Later in the course we'll talk about "fat pointers", i.e. pointers with additional metadata.
    As the name implies, they are larger than the pointers we discussed in this chapter, also known as "thin pointers".
    )
    in Rust are represented, in memory, as a pointer to a memory location.
    It follows that their size is the same as the size of a pointer, a usize  .
    */
    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
