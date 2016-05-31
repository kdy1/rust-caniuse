#![feature(plugin, custom_derive)]
#![plugin(phf_macros, serde_macros)]

extern crate inflect;
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
use serde::Deserialize;
use shared::*;


#[path = "src/shared/mod.rs"]
mod shared;


fn main() {
    let client = Client::new();
    let head = get_git_head(&client);
    let data = Data::get(&client, &head).patch();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("datas.rs");
    DataPrinter {
        data: data,
        w: File::create(&dest_path).unwrap(),
    }
    .print();
}

struct DataPrinter<W: Write> {
    data: Data,
    w: W,
}

impl<W> DataPrinter<W>
    where W: Write
{
    fn print(&mut self) {
        // start: pub enum Fature
        self.p(b"#[derive(Debug,Copy,Clone)]
    pub enum Feature {\n");
        self.print_each(|f| {
            format!("/// http://caniuse.com/#feat={id}
/// {desc}
{var},\n",
                    id = f.id,
                    var = f.var_name,
                    desc = &f.description)
        });
        self.p(b"}\n");
        // end: pub enum Feature


        self.print_feature_impl();

        // // print id:Feature map
        self.p(b"\n\npub static FEATURES: phf::Map<&'static str, Feature> = ");
        {
            let mut features = phf_codegen::Map::<&str>::new();
            for (_, f) in &self.data.data {
                features.entry(&f.id, &format!("Feature::{}", f.var_name));
            }
            features.build(&mut self.w).unwrap();
        }
        self.p(b";\n\n");
    }

    fn print_feature_impl(&mut self) {
        self.p(b"impl Feature {");

        // start: fn id()
        self.p(b"\n/// ID of the feature.
pub fn id(self) -> &'static str {");
        self.print_match(|f| format!("\"{}\"", f.id));
        self.p(b"}\n");

        // start: fn parent_id()
        self.p(b"\n/// ID of the parent feature, or empty string.
pub fn parent_id(self) -> &'static str {");
        self.print_match(|f| format!("\"{}\"", f.id));
        self.p(b"}\n");

        // start: fn status()
        self.p(b"\n/// Specification status.
pub fn status(self) -> Status {");
        self.print_match(|f| format!("Status::{}", f.status));
        self.p(b"}\n");

        // start: fn title()
        self.p(b"\n
pub fn title(self) -> &'static str {");
        self.print_match(|f| format!("\"{}\"", f.title));
        self.p(b"}\n");

        // start: fn stats()
        self.p(b"\n
pub fn stats(self) -> &'static Stats {");
        self.print_match(|f| format!("&STATS_{}", f.const_name));
        self.p(b"}");

        self.p(b"\n\n}");

        self.print_each(|f| {
            use std::fmt::Write;

            let mut stats = String::new();
            stats.push('[');

            for (browser, stat) in &f.stats {
                let mut stat_map = phf_codegen::Map::<&str>::new();
                // stat: map[version]support
                for (ver, support) in stat {
                    stat_map.entry(&ver, &format!("Support::{:?}", support));
                }
                write!(stats,
                       "(Browser::{}, {}),",
                       browser,
                       &Self::build_map(stat_map))
                    .unwrap();
            }
            stats.push(']');
            format!("static STATS_{}: Stats = {};\n", f.const_name, &stats)
        });
    }

    fn print_each<F>(&mut self, expr: F)
        where F: Fn(&Feature) -> String
    {
        for (_, feature) in &self.data.data {
            write!(self.w, "{}", expr(&feature)).unwrap();
        }
    }

    fn print_match<F>(&mut self, expr: F)
        where F: Fn(&Feature) -> String
    {
        self.p(b"\n  match self {\n");
        self.print_each(|f| {
            format!("    Feature::{var} => {expr},\n",
                    var = f.var_name,
                    expr = expr(&f))
        });
        self.p(b"  }\n");
    }

    fn p(&mut self, b: &[u8]) {
        self.w.write_all(b).unwrap();
    }

    fn build_map<T>(map: phf_codegen::Map<T>) -> String
        where T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug + phf::PhfHash
    {
        let mut buf: Vec<u8> = Vec::new();
        map.build(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }
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

impl Data {
    fn get(client: &Client, head: &str) -> Self {
        let u = format!("https://raw.githubusercontent.com/Fyrd/caniuse/{}/data.json",
                        head);
        get_json::<Data>(&client, &format!("data_{}", head), &u)
    }


    fn patch(mut self) -> Self {
        use inflect::CaseFormat;

        for (id, ref mut feature) in &mut self.data {
            feature.id = id.clone();
            feature.var_name = inflect::UpperCamel::convert_to(id);
            feature.const_name = feature.var_name.to_uppercase();
        }

        self
    }
}

#[derive(Debug, Deserialize)]
struct Feature {
    #[serde(skip_deserializing)]
    id: String,
    /// enum variant name
    #[serde(skip_deserializing)]
    var_name: String,
    /// UPPER_CAMEL
    #[serde(skip_deserializing)]
    const_name: String,

    title: String,
    description: String,
    spec: String,
    parent: String,
    status: Status,
    stats: HashMap<Browser, HashMap<String, Support>>,
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

    let dec = match serde_json::from_str::<T>(&body) {
        Ok(dec) => dec,
        Err(e) => {
            let mut f = File::create(dest_path).expect("failed to create cache");
            f.write_all(&body.into_bytes()).expect("failed to write cache");
            panic!(format!("failed to deserialize response: {}, file: {}",
                           e,
                           cache_file_name));
        }
    };

    let mut f = File::create(dest_path).expect("failed to create cache");
    f.write_all(&body.into_bytes()).expect("failed to write cache");

    dec
}
