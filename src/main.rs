use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::os::unix::raw::time_t;
use std::path;
use clap::error::ContextValue::String;
use jars::{Jar, JarOption, JarOptionBuilder};
use zip::ZipArchive;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    jars: String,

    // #[arg(short, long, default_value_t = "org.apache.flink")]
    // exclude: Vec<String>,

    debug: bool
}


fn main() {
    let path = "/Users/aitozi/ant/work/code/ant/ant-flink/flink-connectors/flink-connector-zdal/target/flink-connector-zdal-1.15.2-SNAPSHOT-jar-with-dependencies.jar";
    let all: HashMap<String, Vec<String>> = HashMap::new();

    for (file_path, content) in get_class_name(path) {
        println!("{}", file_path)
    }
}

/*
1. 正则过滤前缀
2. 多线程插入
3. 多种输出方式
4. 漂亮的输出格式
 */
fn get_and_insert_name(path: &str, all: HashMap<String, Vec<String>>) -> Result<(), Error> {
    let mut files = HashMap::new();
    let mut jar = File::open(path)?;
    let mut jar = jar.map(ZipArchive::new)??;

    // let start = time_t
    for name in jar.file_names() {
        if name.ends_with(".class") {
            // files.insert(name, file)
        }
    }
    Ok(files)
}
