# nickel-bootstrap

A seed implementation of nickel/bootstrap

## props

https://github.com/bootstrap-ruby/sinatra-bootstrap

## notes taken during implementation

* would be helpful if docs listed what you need in your Cargo.toml
* need to list where resources go. Some kind of default for static files?
 - like sinatra has "public"
 - templates could do with a default too, "views" or "templates" perhaps
* deployment strategies? Far future item
* utilize stuff is a bit janky. Would ```use``` be a better word?
 - e.g.
   ```
   nickel.use(routes![
     ...
   ]);
   ```
 - logging etc can be pulled out of "utilize" into something else then so no need to "utilize" everything
* using ```Ok(Halt(response.render...))``` in the ```router!``` macro confuses compiler. Error as follows: ```error: unable to infer enough type information about `_`; type annotations required [E0282]```
  - I think its asking for extra information, but its a closure. Not sure how that works.

* mustache partials don't appear to work. They may not be implemented.
* images get garbled for some reason when served. Mime type wrong?
  - Nope thats not it, i think the mime type is wrong, but only for non existent images e.g. /images/adjsfasdfasdfas.jpg (doesn't exist)
