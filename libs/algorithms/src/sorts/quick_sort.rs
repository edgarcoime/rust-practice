pub fn quick_sort<T, F>(v: &mut [T], f: &F)
    where F: Fn(&T, &T) -> bool
{
    let len = v.len();
    if len >= 2 {
        let pivot_idx = partition(v, f);
        quick_sort(&mut v[0..pivot_idx], f);
        quick_sort(&mut v[pivot_idx + 1..len], f)
    }
}

pub fn partition<T, F>(v: &mut [T], f: &F) -> usize
    where F: Fn(&T, &T) -> bool
{
    let len = v.len();
    let pivot_idx = len / 2;
    let last_idx = len - 1;

    // Why?
    v.swap(pivot_idx, last_idx);

    let mut store_idx = 0;
    for i in 0..last_idx {
        if f(&v[i], &v[last_idx]) {
            v.swap(i, store_idx);
            store_idx += 1;
        }
    }

    v.swap(store_idx, len - 1);
    store_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);

        quick_sort(&mut numbers, &|a, b| a < b);
        println!("After: {:?}", numbers);

        let soln = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
        assert_eq!(numbers, soln);
    }
}