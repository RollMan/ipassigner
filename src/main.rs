extern crate regex;
extern crate nickel;
extern crate postgres;

use nickel::Nickel;
use nickel::{Request, Response, MiddlewareResult, HttpRouter};
use regex::Regex;
use postgres::{Connection, SslMode};

struct Address {
    address: String,
    user_name: String,
    user_history: Vec<String>,
}

struct User {
    id: i32,
    name: String,
    student_id: String,
}

fn status(addr_db: &Connection, user_db: &Connection) -> for<'r, 'mw, 'conn> fn (&mut Request<'mw, 'conn>, Response<'mw>) -> MiddlewareResult<'mw>{

    fn status_<'mw, 'conn>(request: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw>{
        let username = request.param("username").unwrap();
        let operation = request.param("operation").unwrap();

        res.send( format!("{} requested {}.", username, operation) )
    }

    let res = status_;
    return res;
}

fn main(){
    let addr_db = Connection::connect("postgres://username:passwd@host:port/database", SslMode::None).unwrap();
    let user_db = Connection::connect("postgres://username:passwd@host:port/database", SslMode::None).unwrap();


    let mut server = Nickel::new();

    server.get(Regex::new("/api/v1/status/(?P<username>[a-zA-Z0-9]+)/(?P<operation>(request|list|return))").unwrap(), (status(&addr_db, &user_db)));
    server.listen("127.0.0.1:8080");
}
