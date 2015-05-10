#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

use std::collections::HashMap;
use nickel::*;


fn main() {

   let mut nickel = Nickel::new();

   /*fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
     let mut data = HashMap::<&str, &str>::new();
     data.insert("title", "nickel-bootstrap");
     data.insert("message", "Welcome to");
     data.insert("message_line_two", "You can edit this to get started...");

     res.render("templates/all_for_one.mustache", &data);
   }*/

   fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
     let mut data = HashMap::<&str, &str>::new();
     data.insert("title", "nickel-bootstrap");
     data.insert("message", "Welcome to");
     data.insert("message_line_two", "You can edit this to get started...");
     res.render("templates/all_for_one.mustache", &data)
   }

   nickel.get("**", handler);

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
   });

   nickel.listen("127.0.0.1:6767");
}
