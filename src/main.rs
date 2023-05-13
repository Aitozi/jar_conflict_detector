use std::{env, path};
use std::cmp::min;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Error;
use std::path::Path;
use std::rc::Rc;
use std::string::String;

use clap::builder::Str;
use clap::Parser;
use zip::ZipArchive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    jar_list: String,

    #[arg(short, long, action = clap::ArgAction::Append)]
    excludes: Vec<String>,

    #[arg(short = 'c')]
    #[arg(long = "prefix_count")]
    #[arg(default_value_t = 3)]
    #[arg()]
    group_prefix_count: usize,
}


fn main() {
    let args = Args::parse();
    let paths: Vec<_> = args.jar_list.split(";").collect();
    if paths.len() < 2 {
        println!("No conflict class found");
        return;
    }

    let mut all: BTreeMap<Rc<String>, Vec<Rc<String>>> = BTreeMap::new();

    // build all class to jar mapping
    for x in paths {
        let jar_name = Rc::new(get_jar_name(x));
        insert_all(&mut all, extract_class_filenames_from_jar(&x, jar_name, &args.excludes).unwrap());
    }

    // cut the key length for more readability
    let mut result: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    all.iter()
        .filter(|&(k, v)| v.len() >= 2)
        .for_each(|(key, value)| {
            let v: Vec<&str> = key.split("/").collect();
            let len = min(v.len(), args.group_prefix_count);
            let cut_key = v[0..len].join("/");
            let v = result.entry(cut_key).or_insert(HashSet::new());
            value.iter().for_each(|m| {v.insert(m.to_string());});
        });

    for (name, jar) in result {
        println!("{:?}, {:?}", name, jar)
    }
}

fn insert_all(all: &mut BTreeMap<Rc<String>, Vec<Rc<String>>>, another: HashMap<Rc<String>, Vec<Rc<String>>>) {
    for (name, jar) in another {
        match all.get_mut(&name) {
            Some(x) => x.extend(jar),
            None => {
                all.insert(name, jar);
            }
        }
    }
}

fn get_jar_name(path: &str) -> String {
    let path = Path::new(path);
    match path.file_name() {
        Some(file_name) => file_name.to_os_string().into_string().unwrap(),
        None => panic!("Not a valid jar path {:?}", path)
    }
}

fn extract_class_filenames_from_jar(path: &str, jar_name: Rc<String>, excludes: &Vec<String>) -> Result<HashMap<Rc<String>, Vec<Rc<String>>>, Error> {
    let mut classes: HashMap<Rc<String>, Vec<Rc<String>>> = HashMap::new();
    let mut jar = File::open(path)?;
    let mut jar = ZipArchive::new(jar)?;

    for name in jar.file_names() {
        if filter(name, excludes) {
            match classes.get_mut(&name.to_string()) {
                Some(x) => x.push(jar_name.clone()),
                None => {
                    let mut v = Vec::new();
                    v.push(jar_name.clone());
                    classes.insert(Rc::new(name.to_string()), v);
                }
            }
        }
    }
    Ok(classes)
}

fn filter(name: &str, excludes: &Vec<String>) -> bool {
    if !name.ends_with(".class") {
        return false;
    }
    if name.starts_with("META-INF") {
        return false;
    }

    for exclude in excludes {
        if name.starts_with(exclude) {
            return false;
        }
    }
    return true;
}
