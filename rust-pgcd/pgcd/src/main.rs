fn main() {
    println!("Hello, world!");
}

fn gcd(x: usize, y: usize) -> usize {
    let big = std::cmp::max(x, y);
    let small = std::cmp::min(x, y);
    let rem = big % small;
    if rem == 0 {
        small
    } else {
        gcd(small, rem)
    }
}


#[cfg(test)]
mod tests {
    use gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 10), 10);
        assert_eq!(gcd(30, 18), 6);
        assert_eq!(gcd(1071, 1029), 21);
    }
}
