// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

fn squared(num: &mut [i32]) -> &mut [i32] {
    //num.iter().map(|num| num.pow(2)).collect::<Vec<i32>>()

    // for i in num.iter_mut() {
    //     num[*i as usize] = num[*i as usize].pow(2);
    // }
    if num.len() > 0 {
        num[0] = num[0].pow(2);
    }

    if num.len() > 1 {
        num[1] = num[1].pow(2);
    }

    num

    // num.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
