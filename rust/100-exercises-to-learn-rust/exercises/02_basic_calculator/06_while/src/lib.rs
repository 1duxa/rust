// Rewrite the factorial function using a `while` loop.
// 5 * 4 *3 *2 *1

pub fn factorial(n: u32) -> u32 {
    let mut res = n;
    let mut res2 =1u32;
    while res >= 1 {
        res2*=res;

        res-=1;
    }
    res2
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
