#![feature(net)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;

use std::collections::HashMap;
use std::net::IpAddr;
use nickel::*;
use std::io;

fn main() {

   let mut nickel = Nickel::new();

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
     });

   fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {

     let mut data = HashMap::<&str, &str>::new();
     data.insert("title", "nickel-bootstrap");
     data.insert("message", "hello from nickel bootstrap!");
     Ok(Halt(try!(res.render("templates/index.mustache", &data))))
     
   }

   nickel.get("**", middleware!(@handler));

   nickel.listen(IpAddr::new_v4(127,0,0,1), 6767);
}
