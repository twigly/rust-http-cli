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

#[macro_export]
macro_rules! rh_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[macro_export]
macro_rules! rh_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[macro_export]
macro_rules! rh_homepage {
    () => {
        env!("CARGO_PKG_HOMEPAGE")
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn ifelse() {
        let res = ifelse![true, true, false];
        assert_eq!(res, true);

        let res = ifelse![false, true, false];
        assert_eq!(res, false);

        let val = true;
        let res = ifelse![val, 1, 2];
        assert_eq!(res, 1);

        let val = false;
        let res = ifelse![val, 1, 2];
        assert_eq!(res, 2);

        let val = 2;
        let res = ifelse![val == 2, "yes", "no"];
        assert_eq!(res, "yes");

        let val = 3;
        let res = ifelse![val == 2, "yes", "no"];
        assert_eq!(res, "no");
    }

    #[test]
    fn crate_name() {
        assert_eq!(rh_name!(), "rh");
    }

    #[test]
    fn rh_version() {
        assert_eq!(rh_version!(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn rh_homepage() {
        assert_eq!(rh_homepage!(), env!("CARGO_PKG_HOMEPAGE"));
    }
}
