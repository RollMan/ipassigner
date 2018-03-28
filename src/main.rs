extern crate regex;
extern crate nickel;

use nickel::Nickel;
use nickel::{Request, Response, MiddlewareResult, HttpRouter};
use regex::Regex;

fn status<'mw, 'conn>(request: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw>{
    let username = request.param("username").unwrap();
    let operation = request.param("operation").unwrap();

    res.send( format!("{} requested {}.", username, operation) )
}

fn main(){
    let mut server = Nickel::new();

    server.get(Regex::new("/api/v1/status/(?P<username>[a-zA-Z0-9]+)/(?P<operation>(request|list|return))").unwrap(), status);
    server.listen("127.0.0.1:8080");
}
