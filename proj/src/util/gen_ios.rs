use std::collections::HashMap;

const  GRADLE_CMD: &str = "gradlew";

pub fn gradle_gen(task: &str, property: HashMap<&str, &str>) -> String{
    let mut str_pp = String::new();
    for (key, value) in property {
        str_pp += &*format!(" -P{key} {value} ", key = key, value = value);
    }
    let out = format!("{gradle} {task} {property}", gradle = GRADLE_CMD, task = task, property = str_pp);
    return out;
}