// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.

fn test_function(arg: u8) -> bool {
    if (arg < 10) {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_function;

    #[test]
    fn you_can_assert_eq() {
        let val = test_function(8);

        assert_eq!(val, true);
    }
}
