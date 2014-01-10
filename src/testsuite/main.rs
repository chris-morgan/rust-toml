// TOML test suite for [1]
//
// [1]: https://github.com/BurntSushi/toml-test

extern mod extra;
extern mod toml = "toml#0.1";

use extra::json::{Json,String,List,Object};
use extra::treemap::TreeMap;
use toml::toml::{Unsigned,Signed,Float}; // XXX

fn to_json_type(typ: ~str, val: Json) -> Json {
    let mut tree = ~TreeMap::new();
    tree.insert(~"type", String(typ));
    tree.insert(~"value", val);
    Object(tree)
}

fn to_json(v: &toml::Value) -> Json {
    match v {
        &toml::NoValue => { fail!("Invalid toml document"); }
        &toml::Table(ref map) => {
            let mut tree = ~TreeMap::new();
            for (k, v) in map.iter() {
                tree.insert(k.clone(), to_json(v));
            }
            Object(tree)
        }
        &toml::TableArray(ref arr) => {
            List(arr.map(|i| to_json(i)))
        }
        &toml::Array(ref arr) => {
            let list = arr.map(|i| to_json(i)); 
            to_json_type(~"array", List(list))
        }
        &toml::Boolean(true) => { to_json_type(~"bool", String(~"true")) }
        &toml::Boolean(false) => { to_json_type(~"bool", String(~"false")) }
        &Unsigned(n) => { to_json_type(~"integer", String(n.to_str())) }
        &Signed(n) => { to_json_type(~"integer", String("-" + n.to_str())) }
        &Float(n) => { to_json_type(~"float", String(std::f64::to_str(n))) }
        &toml::String(ref str) => { to_json_type(~"string", String(str.clone())) }
        &toml::Datetime(y,m,d,h,mi,s) => {
            let s = format!("{:04u}-{:02u}-{:02u}T{:02u}:{:02u}:{:02u}Z", y,m,d,h,mi,s);
            to_json_type(~"datetime", String(s))
        }
    }
}

fn main() {
    let toml = toml::parse_from_bytes(std::io::stdin().read_to_end());
    let json = to_json(&toml);
    println!("{:s}", json.to_pretty_str());
}
