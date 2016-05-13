#![feature(plugin, custom_derive)]
#![plugin(phf_macros, serde_macros)]

extern crate aster;
extern crate hyper;
extern crate phf;
extern crate phf_codegen;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use hyper::Client;
use hyper::header::Connection;
use serde::{Deserialize, Deserializer};
use shared::*;


#[path = "src/shared.rs"]
mod shared;


fn main() {
    let client = Client::new();
    let head = get_git_head(&client);
    let data = get_data(&client, &head);
    print_data(data);
}

fn print_data(d: Data) {
    use std::fmt::Write;

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("datas.rs");
    let mut f = File::create(&dest_path).unwrap();
    let mut features = phf_codegen::Map::<&str>::new();


    // print `pub const FEATURE_{var}`
    for (key, feat) in &d.data {
        let var_name = key.replace("-", "_").to_uppercase();
        let mut stats = phf_codegen::Map::new();
        for (br, bs) in &feat.stats {
            // Version:support map
            let mut vmap = phf_codegen::Map::<&str>::new();

            for (ver, support) in bs {
                vmap.entry(ver, &format!("\"{}\"", support));
            }

            stats.entry(*br, &build_phf_map(vmap));
        }


        write!(f,
               r#"
///
/// http://caniuse.com/#feat={id}
///
///
pub const FEATURE_{var}: Feature = Feature{{
                id: "{id}",
    title: "{title}",
    parent_id: "{parent}",
    status: {status:?},
	stats: {stats},
}};
"#,
               var = var_name,
               id = key,
               title = feat.title,
               parent = feat.parent,
               status = feat.status,
               stats = &build_phf_map(stats))
            .unwrap();
        features.entry(key, &format!("FEATURE_{}", var_name));
    }


    // print id:Feature map
    f.write_all(b"\n\npub static FEATURES: phf::Map<&'static str, Feature> = ").unwrap();
    features.build(&mut f).unwrap();
    f.write_all(b";\n\n").unwrap();
}

#[derive(Debug, Deserialize)]
struct Data {
    // eras: HashMap<String, String>,
    /// not used, but enum is verified
    statuses: HashMap<Status, String>,
    /// enum is verified
    agents: HashMap<Browser, Agent>,
    data: HashMap<String, Feature>,
}

#[derive(Debug, Deserialize)]
struct Feature {
    title: String,
    description: String,
    spec: String,
    parent: String,
    status: Status,
    stats: HashMap<Browser, HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Agent {
    /// Title of browser
    browser: String,
    abbr: String,
    prefix: Prefix,
    #[serde(rename="type")]
    typ: String,
    versions: Vec<Option<String>>,
    prefix_exceptions: Option<HashMap<String, Prefix>>,
}

#[derive(Debug, Deserialize)]
struct NpmResponse {
    #[serde(rename="gitHead")]
    git_head: String,
}

fn get_data(client: &Client, head: &str) -> Data {
    let u = format!("https://raw.githubusercontent.com/Fyrd/caniuse/{}/data.json",
                    head);

    get_json::<Data>(&client, &format!("data_{}", head), &u)
}

fn get_git_head(client: &Client) -> String {
    let ver = env::var("CARGO_PKG_VERSION").unwrap();
    println!("Version: {}", ver);

    let u = format!("https://registry.npmjs.org/caniuse-db?version={}", &ver);
    let r = get_json::<NpmResponse>(&client, &format!("git_head_{}", ver), &u);
    println!("Head: {}", r.git_head);
    r.git_head
}

// cache_file_name should include hash.
// get_json just read cache if it exists.
fn get_json<T: Deserialize>(client: &Client, cache_file_name: &str, url: &str) -> T {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(cache_file_name);

    // use cache if exists
    if let Ok(f) = File::open(&dest_path) {
        return serde_json::from_reader::<_, T>(f).expect("failed to deserialize cache");
    }
    // Creating an outgoing request.
    let mut res = client.get(url)
                        .header(Connection::close())
                        .send()
                        .unwrap();

    // Read the Response.
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let decoded: T = serde_json::from_str(&body).expect("failed to deserialize response");

    let mut f = File::create(dest_path).expect("failed to create cache");
    f.write_all(&body.into_bytes()).expect("failed to write cache");

    decoded
}

fn build_phf_map<T>(m: phf_codegen::Map<T>) -> String
    where T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug + phf::PhfHash
{
    let mut buf: Vec<u8> = Vec::new();
    m.build(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
