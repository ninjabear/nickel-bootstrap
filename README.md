# nickel-bootstrap

A seed implementation of nickel/bootstrap


## Notes

* would be helpful if docs listed what you need in your Cargo.toml
* need to list where resources go. Some kind of default for static files?
 - like sinatra has "public"
 - templates could do with a default too, "views" or "templates" perhaps
* deployment strategies? Far future item
* utilize stuff is a bit janky. Would ```use``` be a better word?
 - e.g.
   ```{rust}
   nickel.use(routes![
     ...
   ]);
   ```
 - logging etc can be pulled out of "utilize" into something else then so no need to "utilize" everything
* using ```{rust} Ok(Halt(response.render...))``` in the ```{rust} router!``` macro confuses compiler. Error as follows: ```error: unable to infer enough type information about `_`; type annotations required [E0282]```
  - I think its asking for extra information, but its a closure. Not sure how that works.
