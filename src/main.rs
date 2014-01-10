#[crate_id = "routed_http"];

extern mod http;
extern mod route_recognizer;
extern mod extra;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{RequestUri,AbsolutePath};
use route_recognizer::{Router,Params};

type Handler = fn(&ServerRequest, &mut ResponseWriter);

pub struct ServerRequest<'a> {
  request: &'a Request,
  params: Params
}

#[deriving(Clone)]
pub struct RoutedServer {
  router: Router<Handler>
}

impl RoutedServer {
  pub fn new(router: Router<Handler>) -> RoutedServer {
    RoutedServer{ router: router }
  }
}

impl Server for RoutedServer {
  fn get_config(&self) -> Config {
    Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
  }

  fn handle_request(&self, request: &Request, response: &mut ResponseWriter) {
    let url = request_uri_to_path(&request.request_uri);
    let matched = self.router.recognize(url).unwrap();
    let &handler = matched.handler;
    handler(&ServerRequest{ request: request, params: matched.params }, response);
  }
}

fn request_uri_to_path(uri: &RequestUri) -> ~str {
  match *uri {
    AbsolutePath(ref str) => str.clone(),
    _ => fail!("TODO")
  }
}

fn main() {
  let mut router = Router::new();
  router.add("/hello", hello);
  router.add("/posts/:id", posts);

  fn hello(_: &ServerRequest, response: &mut ResponseWriter) {
    response.write(bytes!("hello world"));
  }

  fn posts(request: &ServerRequest, response: &mut ResponseWriter) {
    let id = request.params["id"].unwrap();
    let output = "hello " + id.as_slice();
    response.write(output.as_bytes());

  }

  RoutedServer::new(router).serve_forever();
}
