use jars::{jar, JarOptionBuilder};

fn main() {
    let jar = jars::jar(
        "/Users/aitozi/ant/work/code/ant/ant-flink/flink-connectors/flink-connector-lindorm/target/flink-connector-lindorm-1.15-antflink-1.0.0-SNAPSHOT-jar-with-dependencies.jar",
        JarOptionBuilder::default(),
    ).unwrap();

    for (file_path, content) in jar.files {
        println!("{}", file_path)
        // ...
    }
}
