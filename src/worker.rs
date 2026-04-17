#![allow(dead_code, unused)]
use std::fs;

#[derive(Debug)]
pub struct MKeyValue {
    pub key: String,
    pub value: String,
}

pub type MapFn = fn(key: &String, value: &str) -> Vec<MKeyValue>;
pub type ReduceFn = fn(key: String, values: Vec<String>) -> String;

// Question: How do we get our data source from?
// The formal spec says this is determined by the master node, where they
// split the data into different partitions
// Looking at the src code, it seems its' done via RPC?, but you wouldn't
// want to send data via RPC, So instead we should use files instead to communicate
// via RPC. I still dont get why we need to communicate via RPC, but I have not
// use RPC in go neither am i experienced in rust, So if we just read from `source`
pub fn ask_task() -> String {
    String::new()
}

pub fn worker(
    task_file_name: &String,
    map: MapFn,
    reduce: ReduceFn,
) -> Result<Vec<MKeyValue>, Box<dyn std::error::Error>> {
    println!("[WORKER] Woker starting");
    let content = match fs::read_to_string(task_file_name) {
        Ok(v) => v,
        Err(err) => {
            let err_info = format!("[WORKER] could not open task_file: {}", task_file_name);
            return Err(err_info.into());
        }
    };

    // According to the task spec, we need to break a stream of text into words
    // Now I'm confused. Here is the thing, the mapFn(key, value) -> Vec<KV>
    // So what are we feeding it? OHH, all this we've done here is supposed to be the user_impl
    // All we could give it the name of the file and the content
    // I'm not sure yet, but I think I can wayne on the second one, since it almost
    // looks like the function signature described in the GO code
    let mut words = content.split_ascii_whitespace();
    let mut list_kv_pairs = map(task_file_name, &content);
    println!("list of kv_pairs from map_func() n");

    for kv in list_kv_pairs.iter() {
        println!("{:?}", kv);
    }

    Ok(list_kv_pairs)
}
