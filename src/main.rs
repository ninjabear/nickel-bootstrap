#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::net::IpAddr;
use nickel::*;
use rustc_serialize::Encodable;


fn main() {

   let mut nickel = Nickel::new();
   let mut router = Nickel::router();

   router.get("**", middleware! { |req, mut resp|

     let mut data = HashMap::<&str, &str>::new();

     data.insert("title", "nickel-bootstrap");
     data.insert("message", "Welcome to");
     data.insert("message_line_two", "You can edit this to get started...");

     resp.render("templates/all_for_one.mustache", data);

  });

   nickel.utilize(router);

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
   });

   nickel.listen("127.0.0.1:6767");
}
