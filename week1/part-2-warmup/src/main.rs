/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ret_v: Vec<i32> = Vec::new();
    let mut tmp;
    for i in v.iter() {
        tmp = i + n;
        ret_v.push(tmp);
    }
    ret_v
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    let length = v.len();
    let mut i = 0;
    while i < length {
        v[i] += n;
        i += 1;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut elems = HashSet::new();
    let mut length = v.len();
    let mut i = 0;
    let mut tmp ;
    while i < length {
        tmp = v[i];
        if elems.contains(&tmp) {v.remove(i); i -= 1;} else {elems.insert(tmp);}
        i += 1;
        // need to update during the running
        length = v.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
