#[macro_export]
macro_rules! ifelse {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition {
            $_true
        } else {
            $_false
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_ifelse() {
        let res = ifelse![true, true, false];
        assert!(res == true);

        let res = ifelse![false, true, false];
        assert!(res == false);

        let val = true;
        let res = ifelse![val, 1, 2];
        assert!(res == 1);

        let val = false;
        let res = ifelse![val, 1, 2];
        assert!(res == 2);

        let val = 2;
        let res = ifelse![val == 2, "yes", "no"];
        assert!(res == "yes");

        let val = 3;
        let res = ifelse![val == 2, "yes", "no"];
        assert!(res == "no");
    }
}
