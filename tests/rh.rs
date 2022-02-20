// use httpmock::prelude::*;

// use rh::args;

// #[test]
// fn no_args() {
//     let mut args = args![];
//     rh::run(&mut args);
// }

// #[test]
// fn localhost() {
//     let server = MockServer::start();
//     let http_mock = server.mock(|when, then| {
//         when.path("/");
//         then.status(200)
//             .header("content-type", "text/html")
//             .body("ohi");
//     });
//     let response = isahc::get(server.url("/")).unwrap();
//     let url = format!("http://localhost:{}", server.port());

//     let mut args = args![url];
//     rh::run(&mut args);

//     http_mock.assert();
//     assert_eq!(response.status(), 200);
// }
