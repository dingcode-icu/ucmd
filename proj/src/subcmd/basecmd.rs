extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};
use std::io::Read;
use self::yaml_rust::Yaml;

pub(crate) trait BaseCmd {
    fn parse_config(conf: &str) -> Yaml {
        let mut file = std::fs::File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        drop(file);
        let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
        let doc = &docs[0];
        println!("{:?}", doc);
        return doc.to_owned()
    }

    fn run(&self){}
}