extern crate rand;
extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;
use router::Router;
use std::io::Read;

#[derive(RustcDecodable)]
struct JsonRequest {
  name: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
  response: String,
  success: bool,
  error_message: String
}

impl JsonResponse {
  fn success(response: String) -> Self {
    JsonResponse { response: response, success: true, error_message: "".to_string() }
  }

  fn error(msg: String) -> Self {
    JsonResponse { response: "".to_string(), success: false, error_message: msg }
  }
}

fn pick_response(name: String) -> String {
  let num = rand::thread_rng().gen_range(1, 4);

  let response = match num {
    1 => format!("Hello {}!", name),
    2 => format!("Did you see that ludicrous display last night, {}?", name),
    3 => format!("Nice weather for ducks, isn't it {}", name),
    _ => format!("")     // match is exhaustive
  };

  response.to_string()
}

fn handler(req: &mut Request) -> IronResult<Response> {
  let response = JsonResponse::success(response: pick_response("Brian".to_string()));
  let out = json::encode(&response).expect("Failed to encode response");

  let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
  Ok(Response::with((content_type, status::Ok, out)))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
  let mut payload = String::new();
  req.body.read_to_string(&mut payload).expect("Failed to read request body");

  // let incoming: JsonResponse = json::decode(&payload).ok().expect("Invalid JSON in POST body");
  let out = match json::decode(&payload) {
    Err(e) => {
      let response = JsonResponse::error(format!("Error parsing JSON: {:?}", e));
      json::encode(&response).ok().expect("Error encoding response")
    },
    Ok(incoming) => {
      let converted: JsonRequest = incoming;
      let response = JsonResponse::success(get_name(converted.name));
      json::encode(&response).expect("Error encoding response")
    }
  };

  let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
  Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
  let mut router = Router::new();
  router.get("/", handler, "index");
  router.post("/", post_handler, "post_name");

  println!("Listening on localhost:3009");
  Iron::new(router).http("localhost:3009").ok();
}