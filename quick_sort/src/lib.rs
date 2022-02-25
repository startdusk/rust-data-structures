use std::fmt::Debug;

use crossbeam;

mod b_rand;

// Move first element to the correct place
// Everything lower should be before it,
// everything highter should be after it
// return it's location
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
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
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // Middle element already sorted
}

// T: 'static 只能是静态类型(静态类型只是一种类型)
pub fn threaded_quicksort_safe<T: PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", p);

    let (a, b) = v.split_at_mut(p);

    // 讨论：https://users.rust-lang.org/t/does-a-threaded-quick-sort-necessarily-require-unsafe-and-raw-pointers/49988/3
    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            threaded_quicksort_safe(a);
        });
        threaded_quicksort_safe(&mut b[1..]);
    })
    .unwrap(); // thread is also implicitly joined here
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
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
    }
    #[test]
    fn test_quick_sort_threaded() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        threaded_quicksort_safe(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
