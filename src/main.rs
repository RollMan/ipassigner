extern crate regex;
extern crate nickel;
extern crate postgres;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use nickel::Nickel;
use nickel::{Request, Response, MiddlewareResult, HttpRouter, JsonBody, MediaType};
use nickel::status::StatusCOde;
use regex::Regex;
use postgres::{Connection, SslMode};
use rustc_serialize::json::{Json, ToJson};

struct Address {
    address: String,
    user_id: i32,
    user_history: Vec<i32>,
}

impl ToJson for Address {
    fn to_json(&self) -> Json{
        let mut map = BTreeMap::new();
        map.insert("address".to_string(), self.address.to_json());
        map.insert("user_name".to_string(), self.user_name.to_json());
        map.insert("user_history".to_string(), self.user_history.to_json());
        Json::Object(map)
    }
}

struct User {
    id: i32,
    name: String,
    student_id: String,
}

impl ToJson for User {
    fn to_json(&self) -> Json{
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("name".to_string(), self.name.to_json());
        map.insert("student_id".to_string(), self.student_id.to_json());
        Json::Object(map)
    }
}

struct StatusResult{
    success: bool,
    operation: String,
    address: String,
}

impl ToJson for StatusResult {
    fn to_json(&self) -> Json{
        let mut map = BTreeMap::new();
        map.insert("success".to_string, self.success.to_json());
        map.insert("operation".to_string, self.operation.to_json());
        map.insert("address".to_string, self.address.to_json());
        JSON::Object(map)
    }
}

fn status(addr_db: &Connection, user_db: &Connection) -> for<'r, 'mw, 'conn> fn (&mut Request<'mw, 'conn>, Response<'mw>) -> MiddlewareResult<'mw>{

    fn status_<'mw, 'conn>(request: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw>{
        let userid = request.param("username").unwrap();
        let operation = request.param("operation").unwrap();
        let mut res_status: StatusResult;

        if(operation == "request"){
            let available_addresses = addr_db.query("SELECT * FROM addresses WHERE user_id IS NULL", &[]).unwrap();
            if(available_addresses.is_epmry()){
                return res.send("{success: false}");
            }
            let mut entry: postgres::Row = available_addresses[0];
            let address = entry.get(0);
            let user_id = entry.get(1);
            addr_db.execute("UPDATE addresses SET column = $1 WHERE address = $2", &[&use_id, &address]);

            res_status = StatusResult(true, operation, address);
        }else if(operation == "list"){

        }else if(operation == "return"){

        }else{
            return res.error(StatusCOde::BadRequest, "No such operation");
        }
        res_status.to_json();
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
