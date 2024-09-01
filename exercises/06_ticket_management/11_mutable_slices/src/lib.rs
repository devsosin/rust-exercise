// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

fn squared(slice: &mut [i32]) {
    // for i in 0..slice.len() {
    //     slice[i] = slice[i] * slice[i]
    // }
    // 요소를 바꾸는 것이 가능
    // 이거는 요소가 안바뀌는건가? 테스트 필요 -> 바뀐다.
    for i in slice {
        *i *= *i;
    }
    // 명시적인 표현?
    // for i in slice.iter_mut() {
    //     *i *= *i;
    // }
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
