use std::ops::AddAssign;

pub fn render_newline_if(has_rendered: HasRendered) {
    if has_rendered == HasRendered::Something {
        println!("");
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HasRendered {
    Nothing,
    Something,
}

impl AddAssign for HasRendered {
    fn add_assign(&mut self, other: Self) {
        if *self == Self::Something || other == Self::Something {
            *self = Self::Something;
        } else {
            *self = Self::Nothing;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HasRendered;

    #[test]
    fn test_add_assign_enum_has_rendered() {
        let mut res = HasRendered::Nothing;
        res += HasRendered::Nothing;
        assert!(res == HasRendered::Nothing);

        let mut res = HasRendered::Nothing;
        res += HasRendered::Something;
        assert!(res == HasRendered::Something);

        let mut res = HasRendered::Something;
        res += HasRendered::Nothing;
        assert!(res == HasRendered::Something);

        let mut res = HasRendered::Something;
        res += HasRendered::Something;
        assert!(res == HasRendered::Something);
    }
}
