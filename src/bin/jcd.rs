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

    #[arg(long, help = "Disable the crc check", action = clap::ArgAction::SetTrue)]
    #[arg(default_value_t = false)]
    disable_crc: bool,

    #[arg(short, long, action = clap::ArgAction::Append, help = "The exclude package prefix")]
    exclude: Vec<String>,
}

const MOCK_CRC_NUMBER: u32 = 1;

fn main() {
    let mut args = Args::parse();

    let paths: Vec<_> = args.jar_list.split(";").collect();
    if paths.len() < 2 {
        println!("No conflict class found");
        return;
    }

    // <class, <crc32, jar-list>>
    let mut name_to_sources: BTreeMap<Rc<String>, HashMap<u32, Vec<Rc<String>>>> = BTreeMap::new();

    // build all class to jar mapping
    for x in paths {
        let jar_name = Rc::new(get_jar_name(x));
        extract_class_filenames_from_jar(
            &x,
            &mut name_to_sources,
            jar_name,
            &args.exclude,
            args.disable_crc,
        );
    }

    let mut result: BTreeMap<Rc<String>, HashMap<u32, Vec<Rc<String>>>> = name_to_sources
        .into_iter()
        .filter(|(k, v)| {
            if args.disable_crc {
                v.get(&MOCK_CRC_NUMBER).unwrap().len() >= 2
            } else {
                v.len() >= 2
            }
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
    name_to_sources: &mut BTreeMap<Rc<String>, HashMap<u32, Vec<Rc<String>>>>,
    jar_name: Rc<String>,
    excludes: &Vec<String>,
    disable_crc: bool,
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
        if filter(name, excludes) {
            let zip_file_crc32 = if !disable_crc {
                zip_entry.crc32()
            } else {
                MOCK_CRC_NUMBER
            };
            match name_to_sources.get_mut(&name.to_string()) {
                Some(entries) => match entries.get_mut(&zip_file_crc32) {
                    Some(v) => {
                        v.push(jar_name.clone());
                    }
                    None => {
                        let mut v = Vec::new();
                        v.push(jar_name.clone());
                        entries.insert(zip_file_crc32, v);
                    }
                },
                None => {
                    let mut v = Vec::new();
                    v.push(jar_name.clone());
                    let mut entry = HashMap::new();
                    entry.insert(zip_file_crc32, v);
                    name_to_sources.insert(Rc::new(name.to_string()), entry);
                }
            }
        }
    }
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
