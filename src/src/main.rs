extern crate regex;
extern crate nickel;
extern crate postgres;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use std::error::Error;
use std::env;
use nickel::Nickel;
use nickel::{Request, Response, MiddlewareResult, HttpRouter};
use nickel::status::StatusCode;
use nickel::QueryString;
use regex::Regex;
use postgres::{Connection, TlsMode};
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
    address: Vec<String>,
    reason: String,
}

impl ToJson for StatusResult {
    fn to_json(&self) -> Json{
        let mut map = BTreeMap::new();
        map.insert("success".to_string(), self.success.to_json());
        map.insert("operation".to_string(), self.operation.to_json());
        map.insert("address".to_string(), self.address.to_json());
        map.insert("reason".to_string(), self.reason.to_json());
        Json::Object(map)
    }
}

fn status<'mw, 'conn>(request: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw>{
    //let userid = request.param("username").unwrap().parse::<i32>().unwrap();
    //let operation = request.param("operation").unwrap();
    let mut res_status: StatusResult = StatusResult{success: false, operation: String::new(), address: Vec::new(), reason: String::new()};

    //let DB_HOST = env::var("POSTGRES_SERVICE_HOST").unwrap();
    let DB_HOST = env::var("IPASSIGNER_POSTGRES_SERVICE_HOST").unwrap();
    //let DB_URI: &str = "postgres://postgres:test@127.17.0.2:5432/";
    //let DB_NAME: &str = "asl";

    let db = Connection::connect("postgres://postgres:test@".to_owned() + &DB_HOST[..] + ":5432/asl", TlsMode::None).unwrap();

    {
        let requesting_user_id = request.param("username").unwrap().parse::<i32>().unwrap();
        if db.query("SELECT * FROM users WHERE id = $1", &[&requesting_user_id]).unwrap().is_empty() {
            res_status.reason = String::from("No such user who has the specified id.");
            return res.send(res_status.to_json());
        }
    }

    if request.param("operation").unwrap() == "request" {
        // Need to check if the refered user is exist.
        let requested_user_id = request.param("username").unwrap().parse::<i32>().unwrap();

        let available_addresses = db.query("SELECT * FROM addresses WHERE user_id IS NULL", &[]).unwrap();
        if available_addresses.is_empty() {
            res_status.success = false;
            res_status.reason = String::from("No available addresses. Contact to administrator: Yohei Shimmyo <m5221148@u-aizu.ac.jp>");
            return res.send(res_status.to_json());
        }

        let entry: postgres::rows::Row = available_addresses.get(0);
        let address: String = entry.get(0);
        //let user_id: i32 = entry.get(1);

        match db.execute("UPDATE addresses SET user_id = $1 WHERE address = $2", &[&requested_user_id, &address]) {
            Ok(num_of_rows) => (),
            Err(e) => {
                res_status.reason = String::from(format!("Unexpected database error.\nUPDATE addresses SET user_id = $1 WHERE address = $2\n\n{}", e.description()));
                return res.send(res_status.to_json());
            },
        }

        res_status = StatusResult{success: true, operation: String::from("request"), address: vec![address], reason: String::new()};
    }else if request.param("operation").unwrap() == "list" {
        let requested_user_id =request.param("username").unwrap().parse::<i32>().unwrap(); 

        let list = db.query("SELECT address FROM addresses WHERE user_id = $1", &[&requested_user_id]).unwrap();
        let mut addresses: Vec<String> = Vec::new();
        for address in &list {
            addresses.push(address.get(0));
        }
        res_status = StatusResult{success: true, operation: String::from("list"), address: addresses,reason: String::new()};
    }else if request.param("operation").unwrap() == "return" {
        let requested_user_id =request.param("username").unwrap().parse::<i32>().unwrap(); 
        //let addr = request.query().get("addr");
        let addr = match request.query().get("addr") {
            Some(addr) => {
                // Check the refered address is occupied by the requesting user.
                let usr_has_addr = db.query("SELECT address FROM addresses WHERE address = $1 AND user_id = $2", &[&addr, &requested_user_id]).unwrap();
                if usr_has_addr.is_empty() {
                    res_status.reason = String::from(format!("You don't have such a address: {}", addr));
                    return res.send(res_status.to_json());
                }
                db.execute("UPDATE addresses SET user_id = NULL WHERE address = $1", &[&addr]).unwrap();
                addr
            },
            None => {
                res_status.reason = String::from("You need to specify an address that you want to return.");
                return res.send(res_status.to_json());
            },
        };
        res_status = StatusResult{success: true, operation: String::from("return"), address: vec![addr.to_string()], reason: String::new()};
    }else{
        return res.error(StatusCode::BadRequest, "No such operation");
    }
    res.send(res_status.to_json())
}

fn main(){


    let mut server = Nickel::new();

    server.get(Regex::new("/api/v1/status/(?P<username>[a-zA-Z0-9]+)/(?P<operation>(request|list|return))").unwrap(), status);
    server.listen("0.0.0.0:8080").unwrap();
}
