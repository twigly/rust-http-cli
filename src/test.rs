#[macro_export]
macro_rules! args {
        () => {{
            let v = Vec::<String>::new();
            v
        }};
        ($($elem:expr),+ $(,)?) => {{
            let v = vec![
                $( String::from($elem), )*
            ];
            v
        }};
    }

#[macro_export]
macro_rules! assert_str_eq {
    ($url:expr, $expected:expr) => {
        assert_eq!($url, $expected.to_string())
    };
}

mod basics {
    #[test]
    fn macro_args() {
        let args = args![];
        let expected: Vec<String> = vec![];
        assert_eq!(args, expected);

        let args = args!["one", "two", "three"];
        let expected: Vec<String> = vec!["one".into(), "two".into(), "three".into()];
        assert_eq!(args, expected);
    }

    #[test]
    fn macro_assert_url_eq() {
        let url = "http://test.com";
        assert_str_eq!(url.to_string(), url);
    }
}
