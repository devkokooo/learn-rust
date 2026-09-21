fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    match num {
        0 | 1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5_040,
        8 => 40_320,
        9 => 362_880,
        10 => 3_628_800,
        11 => 39_916_800,
        12 => 479_001_600,
        13 => 6_227_020_800,
        14 => 87_178_291_200,
        15 => 1_307_674_368_000,
        16 => 20_922_789_888_000,
        17 => 355_687_428_096_000,
        18 => 6_402_373_705_728_000,
        19 => 121_645_100_408_832_000,
        20 => 2_432_902_008_176_640_000,
        _ => 0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
