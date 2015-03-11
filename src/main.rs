#![feature(net)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;
extern crate "rustc-serialize" as rustc_serialize;

use std::collections::HashMap;
use std::net::IpAddr;
use nickel::*;
use rustc_serialize::Encodable;


struct View<T>(&'static str, T);

impl<T: Encodable> ResponseFinalizer for View<T> {
    fn respond<'a>(self, res: Response<'a>) -> MiddlewareResult<'a> {
        let View(path, data) = self;
        let stream = try!(res.render(path, &data));

        Ok(Halt(stream))
    }
}

fn main() {
   let mut nickel = Nickel::new();

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
     });

   nickel.utilize(router!(

   get "**" => |request, response| {
      let mut data = HashMap::new();
      data.insert("title", "nickel-bootstrap");
      data.insert("message", "Welcome to");
      data.insert("message_line_two", "You can edit this to get started...");

      View("templates/all_for_one.mustache", data)
   }

   ));

   nickel.listen(IpAddr::new_v4(127,0,0,1), 6767);
}
