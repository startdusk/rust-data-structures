use std::fmt::Debug;

// Move first element to the correct place
// Everything lower should be before it,
// everything highter should be after it
// return it's location
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }

    p
}

// O(n^2)
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    // dbg!(&v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // Middle element already sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
