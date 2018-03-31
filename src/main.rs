extern crate regex;
extern crate nickel;
extern crate postgres;
extern crate rustc_serialize;
#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use nickel::Nickel;
use nickel::{Request, Response, MiddlewareResult, HttpRouter, JsonBody, MediaType};
use nickel::status::StatusCode;
use regex::Regex;
use postgres::{Connection, TlsMode};
use rustc_serialize::json::{Json, ToJson};

static DB_URI: &'static str = "postgres://postgres:test@127.17.0.2/";
static DB_NAME_ADDR: &'static str = "addresses";
static DB_NAME_USER: &'static str = "users";

struct Address {
    address: String,
    user_id: i32,
    user_history: Vec<i32>,
}

impl ToJson for Address {
    fn to_json(&self) -> Json{
        let mut map = BTreeMap::new();
        map.insert("address".to_string(), self.address.to_json());
        map.insert("user_id".to_string(), self.user_id.to_json());
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
        map.insert("success".to_string(), self.success.to_json());
        map.insert("operation".to_string(), self.operation.to_json());
        map.insert("address".to_string(), self.address.to_json());
        Json::Object(map)
    }
}

fn status<'mw, 'conn>(request: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw>{
    let userid = request.param("username").unwrap();
    let operation = request.param("operation").unwrap();
    let mut res_status: StatusResult = StatusResult{success: false, operation: String::new(), address: String::new()};

    let addr_db = Connection::connect(DB_URI.to_owned() + DB_NAME_ADDR, TlsMode::None).unwrap();
    let user_db = Connection::connect(DB_URI.to_owned() + DB_NAME_USER, TlsMode::None).unwrap();

    if operation == "request" {
        let available_addresses = addr_db.query("SELECT * FROM addresses WHERE user_id IS NULL", &[]).unwrap();
        if available_addresses.is_empty() {
            return res.send("{success: false}");
        }
        let mut entry: postgres::rows::Row = available_addresses.get(0);
        let address: String = entry.get(0);
        let user_id: i32 = entry.get(1);
        addr_db.execute("UPDATE addresses SET column = $1 WHERE address = $2", &[&user_id, &address]);

        res_status = StatusResult{success: true, operation: String::from(operation), address: address};
    }else if operation == "list" {

    }else if operation == "return" {

    }else{
        return res.error(StatusCode::BadRequest, "No such operation");
    }
    res.send(res_status.to_json())
}

fn main(){


    let mut server = Nickel::new();

    server.get(Regex::new("/api/v1/status/(?P<username>[a-zA-Z0-9]+)/(?P<operation>(request|list|return))").unwrap(), status);
    server.listen("127.0.0.1:8080");
}
