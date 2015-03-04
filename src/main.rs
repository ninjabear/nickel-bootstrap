#![feature(net)]
#![feature(path)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;
extern crate mustache;

use std::collections::HashMap;
use std::net::IpAddr;
use nickel::*;
use std::path::PathBuf;
use mustache::*;

fn main() {

   let mut nickel = Nickel::new();

   nickel.utilize(StaticFilesHandler::new("public/"));

   nickel.utilize(middleware! { |request|
     println!("Request: {:?}", request.origin.uri);
     });

   nickel.utilize(router!(

   get "**" => |request, response| {
      let mut data = HashMap::new();
      data.insert("title".to_string(), StrVal("nickel-bootstrap".to_string()));
      data.insert("message".to_string(), StrVal("hello from nickel bootstrap!".to_string()));

      let template = Context::new(PathBuf::new("mustache"))
          .compile_path(PathBuf::new("base"))
          .ok()
          .unwrap();

      let mut resp = vec![];
      template.render_data(&mut resp, &Map(data));
      String::from_utf8(resp).unwrap()
   }

   ));

   nickel.listen(IpAddr::new_v4(127,0,0,1), 6767);
}
