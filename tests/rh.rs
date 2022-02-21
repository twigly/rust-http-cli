// use httpmock::prelude::*;
// use rh::shell::{
//     os::{DefaultOsDirs, OsDirs},
//     Shell,
// };

// fn shell<'a, OD: OsDirs>(os_dirs: &'a OD) -> Shell<'a, OD, Vec<u8>, Vec<u8>> {
//     let out = Vec::new();
//     let err = Vec::new();
//     Shell::new(os_dirs, out, err)
// }

// #[test]
// fn no_args() {
//     let os_dirs = DefaultOsDirs::default();
//     let mut shell = shell(&os_dirs);

//     let mut args = rh::test::args![];
//     let exit_code = rh::run(&mut args, &mut shell);
//     assert_eq!(exit_code, 100);
// }

// #[test]
// fn get_localhost() {
//     let server = MockServer::start();
//     let http_mock = server.mock(|when, then| {
//         when.path("/");
//         then.status(200).header("content-type", "text/plain").body("ohi");
//     });
//     let url = server.url("/");

//     let os_dirs = DefaultOsDirs::default();
//     let mut shell = shell(&os_dirs);

//     let mut args = rh::test::args![url];
//     let exit_code = rh::run(&mut args, &mut shell);
//     assert_eq!(exit_code, 0);
//     http_mock.assert();
// }
