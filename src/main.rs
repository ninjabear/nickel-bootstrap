#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

use std::collections::HashMap;
use nickel::*;

fn main() {
   let mut nickel = Nickel::new();

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
   });

   nickel.get("/", middleware! { |_req, res|
      let mut data = HashMap::<&str, &str>::new();
      data.insert("title", "nickel-bootstrap");
      data.insert("message", "Welcome to");
      data.insert("message_line_two", "You can edit this to get started...");
      return res.render("templates/index.mustache", &data)
   });

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.listen("127.0.0.1:6767");
}
