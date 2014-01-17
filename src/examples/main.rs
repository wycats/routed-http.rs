#[crate_id = "routed_http_example#0.1"];
#[crate_type = "bin"];

extern mod http;
extern mod routed_http;

use routed_http::RoutedServer;
use http::headers::content_type::MediaType;

fn main() {
    do RoutedServer::serve |router, config| {
        config.listen("127.0.0.1", 1337);

        router.add("/hello", |_, response| {
            do response.with_response_writer |rw| {
                rw.write_content_auto(MediaType {
                    type_: ~"text",
                    subtype: ~"html",
                    parameters: ~[]
                }, ~"<p>Hello world!</p>");
            }
        });

        router.add("/posts/:id", |request, response| {
            response.write(format!("hello {}", request.params["id"]))
        });

        println!("Go to http://localhost:1337/hello");
        println!("or http://localhost:1337/posts/..., where ... can be anything");
    }
}
