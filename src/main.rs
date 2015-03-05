#![feature(net)]
#![feature(path)]
#![feature(fs)]
#![feature(io)]

extern crate nickel;
#[macro_use] extern crate nickel_macros;
extern crate mustache;

use std::collections::HashMap;
use std::net::IpAddr;
use nickel::*;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
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
      data.insert("message".to_string(), StrVal("Welcome to".to_string()));
      data.insert("message_line_two".to_string(), StrVal("You can edit this to get started...".to_string()));

      let template_file = "templates/all_for_one.mustache";

      let mut file = File::open(&PathBuf::new(template_file))
                          .ok()
                          .expect(&*format!("Couldn't open template: {}", template_file));

      let mut raw_template = String::new();
      file.read_to_string(&mut raw_template)
          .ok()
          .expect(&*format!("Couldn't read template file: {}", template_file));

      let template = mustache::compile_str(&*raw_template);

      let mut resp = vec![];
      template.render_data(&mut resp, &Map(data));
      String::from_utf8(resp).unwrap()
   }

   ));

   nickel.listen(IpAddr::new_v4(127,0,0,1), 6767);
}
