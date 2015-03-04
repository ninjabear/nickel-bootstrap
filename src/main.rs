#![feature(net)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;

use std::net::IpAddr;
use nickel::*;

fn main() {

   let mut nickel = Nickel::new();

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
     });

   nickel.utilize(router!(
      get "**" => |request, response| {
        "hello world"
      }
     ));

   println!("Nickel is running.");
   nickel.listen(IpAddr::new_v4(127,0,0,1), 6767);
}
