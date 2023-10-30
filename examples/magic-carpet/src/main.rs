use rouille::{self, router, Response};
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    hello: String,
}

fn main() {
    rouille::start_server("0.0.0.0:8000", move |_req| {
        let name = "world";
        // let data = format!(r#"{{ "hello": {name:?} }}"#);
        let greeting = Greeting {
            hello: name.to_string(),
        };
        Response::json(&greeting)
    });
}
