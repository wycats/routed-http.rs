#[crate_id = "routed_http"];

extern mod http;
extern mod route_recognizer;
extern mod extra;

use std::io::net::ip::{SocketAddr, Ipv4Addr, IpAddr};
use http::server::{Config, Server, Request, ResponseWriter};
use http::server::request::{RequestUri,AbsolutePath};
use route_recognizer::{Router,Params,Match};

type Handler = 'static |&ServerRequest, &mut ServerResponse|: Send;

impl Clone for Handler {
  fn clone(&self) -> Handler {
    unsafe { std::cast::transmute_copy(self) }
  }
}

pub struct ServerRequest<'a> {
  request: &'a Request,
  params: Params
}

pub struct ServerResponse<'a, 'b> {
  response: &'b mut ResponseWriter<'a>
}

impl<'a, 'b> ServerResponse<'a, 'b> {
  pub fn write(&mut self, string: &str) {
    self.response.write(string.as_bytes());
  }
}

#[deriving(Clone)]
pub struct ServerConfig {
  ip: IpAddr,
  port: u16
}

impl ServerConfig {
  pub fn new() -> ServerConfig {
    ServerConfig{ ip: Ipv4Addr(127, 0, 0, 1), port: 8888 }
  }

  pub fn to_config(&self) -> Config {
    Config{ bind_address: SocketAddr{ ip: self.ip, port: self.port } }
  }

  pub fn host(&mut self, string: &'static str) {
    let parts = string.split('.').to_owned_vec();

    let a = from_str(parts[0]).unwrap();
    let b = from_str(parts[1]).unwrap();
    let c = from_str(parts[2]).unwrap();
    let d = from_str(parts[3]).unwrap();

    self.ip = Ipv4Addr(a, b, c, d);
  }

  pub fn port(&mut self, port: u16) {
    self.port = port;
  }

  pub fn listen(&mut self, host: &'static str, port: u16) {
    self.host(host);
    self.port(port);
  }
}

#[deriving(Clone)]
pub struct RoutedServer {
  router: Router<Handler>,
  config: ServerConfig
}

impl RoutedServer {
  pub fn new(router: Router<Handler>) -> RoutedServer {
    RoutedServer{ router: router, config: ServerConfig::new() }
  }

  pub fn serve(configuration: proc(router: &mut Router<Handler>, config: &mut ServerConfig)) {
    let mut server = RoutedServer::new(Router::new());
    configuration(&mut server.router, &mut server.config);
    server.serve_forever();
  }
}

impl Server for RoutedServer {
  fn get_config(&self) -> Config {
    self.config.to_config()
  }

  fn handle_request(&self, request: &Request, response: &mut ResponseWriter) {
    let url = request_uri_to_path(&request.request_uri);
    let Match{ params, handler } = self.router.recognize(url).unwrap();

    let server_request = &ServerRequest{ request: request, params: params };
    let server_response = &mut ServerResponse{ response: response };
    (*handler)(server_request, server_response);
  }
}

fn request_uri_to_path(uri: &RequestUri) -> ~str {
  match *uri {
    AbsolutePath(ref str) => str.clone(),
    _ => fail!("TODO")
  }
}

fn main() {
  do RoutedServer::serve |router, config| {
    config.listen("127.0.0.1", 1337);

    router.add("/hello", |_, response| {
      response.write("hello world");
    });

    router.add("/posts/:id", |request, response| {
      response.write(format!("hello {}", request.params["id"]))
    });
  }
}
