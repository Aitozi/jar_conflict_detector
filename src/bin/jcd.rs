use std::cell::RefCell;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Error;
use std::path::Path;
use std::rc::Rc;
use std::string::String;
use std::{env, path};

use clap::builder::Str;
use clap::Parser;
use zip::read::ZipFile;
use zip::ZipArchive;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long = "jars",
        required = true,
        help = "The jar list joined by semicolon"
    )]
    jar_list: String,

    #[arg(short, long)]
    #[clap(value_enum, default_value_t = DistinctFrom::SIZE)]
    check: DistinctFrom,

    #[arg(short, long, action = clap::ArgAction::Append, help = "The exclude package prefix")]
    exclude: Vec<String>,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
enum DistinctFrom {
    CRC,
    SIZE,
    NONE,
}

const DISTINCT_FROM_NONE: u64 = 0;

fn main() {
    let mut args = Args::parse();

    let paths: Vec<_> = args.jar_list.split(";").collect();
    if paths.len() < 2 {
        println!("No conflict class found");
        return;
    }

    // <class, <crc32, jar-list>>
    let mut name_to_sources: BTreeMap<Rc<String>, HashMap<u64, Vec<Rc<String>>>> = BTreeMap::new();

    // build all class to jar mapping
    for x in paths {
        let jar_name = Rc::new(get_jar_name(x));
        extract_class_filenames_from_jar(&x, &mut name_to_sources, jar_name, &args);
    }

    let mut result: BTreeMap<Rc<String>, HashMap<u64, Vec<Rc<String>>>> = name_to_sources
        .into_iter()
        .filter(|(k, v)| match args.check {
            DistinctFrom::NONE => v.get(&DISTINCT_FROM_NONE).unwrap().len() >= 2,
            _ => v.len() >= 2,
        })
        .collect();

    for (name, jar) in result {
        println!("{:?}, {:?}", name, jar)
    }
}

fn get_jar_name(path: &str) -> String {
    let path = Path::new(path);
    match path.file_name() {
        Some(file_name) => file_name.to_os_string().into_string().unwrap(),
        None => panic!("Not a valid jar path {:?}", path),
    }
}

fn extract_class_filenames_from_jar(
    path: &str,
    name_to_sources: &mut BTreeMap<Rc<String>, HashMap<u64, Vec<Rc<String>>>>,
    jar_name: Rc<String>,
    args: &Args,
) {
    let jar = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("path: {} err: {}", path, e);
        }
    };
    let mut zip = ZipArchive::new(jar).unwrap();

    for i in 0..zip.len() {
        let mut zip_entry = zip.by_index(i).unwrap();
        let name = zip_entry.name();
        if filter(name, &args.exclude) {
            let distinct_from = get_distinct_from(&zip_entry, args);
            match name_to_sources.get_mut(&name.to_string()) {
                Some(entries) => match entries.get_mut(&distinct_from) {
                    Some(v) => {
                        v.push(jar_name.clone());
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(jar_name.clone());
                        entries.insert(distinct_from, v);
                    }
                },
                None => {
                    let mut v = Vec::new();
                    v.push(jar_name.clone());
                    let mut entry = HashMap::new();
                    entry.insert(distinct_from, v);
                    name_to_sources.insert(Rc::new(name.to_string()), entry);
                }
            }
        }
    }
}

fn get_distinct_from(zip: &ZipFile, arg: &Args) -> u64 {
    return match arg.check {
        DistinctFrom::CRC => zip.crc32() as u64,
        DistinctFrom::SIZE => zip.size(),
        DistinctFrom::NONE => DISTINCT_FROM_NONE,
    };
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

#[test]
fn test_parse() {
    // https://stackoverflow.com/questions/74465951/how-to-parse-custom-string-with-clap-derive
    // https://github.com/clap-rs/clap/discussions/4517
    // first argument is binary name
    let args = Args::parse_from(["", "--jars", "a.jar;b.jar"]);
    assert_eq!(args.check, DistinctFrom::SIZE);
    assert!(args.exclude.is_empty());

    let args = Args::try_parse_from([""]);
    assert!(args.is_err());

    let args = Args::parse_from(["", "--jars", "a.jar", "-c", "crc"]);
    assert_eq!(args.check, DistinctFrom::CRC);

    let args = Args::parse_from(["", "--jars", "a.jar", "-c", "none"]);
    assert_eq!(args.check, DistinctFrom::NONE);

    let args = Args::try_parse_from(["", "--jars", "a.jar", "-c", "none1"]);
    assert!(args.is_err());
}
