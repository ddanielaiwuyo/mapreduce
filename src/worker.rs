use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::{fs, thread};

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

#[derive(Debug, Hash, PartialEq, Eq)]
struct HashCounter {
    key: String,
    values: Vec<String>,
}

#[derive(Debug)]
pub struct WorkerInstruction {
    pub send_ch: Sender<Option<Vec<MKeyValue>>>,
    pub file_path: String,
    pub map_fn: MapFn,
    pub reduce_fn: ReduceFn,
}

/// Worker function
pub fn worker(
    task_file_path: &String,
    map_fn: MapFn,
    reduce_fn: ReduceFn,
) -> Result<Vec<MKeyValue>, Box<dyn std::error::Error>> {
    println!("[WORKER] Woker starting");
    let content = match fs::read_to_string(task_file_path) {
        Ok(v) => v,
        Err(err) => {
            let err_info = format!(
                "[WORKER] could not open task_file: {}
                Reason: {err}",
                task_file_path
            );
            return Err(err_info.into());
        }
    };

    // According to the task spec, we need to break a stream of text into words
    // Now I'm confused. Here is the thing, the mapFn(key, value) -> Vec<KV>
    // So what are we feeding it? OHH, all this we've done here is supposed to be the user_impl
    // All we could give it the name of the file and the content
    // I'm not sure yet, but I think I can wayne on the second one, since it almost
    // looks like the function signature described in the GO code
    // let words = content.split_ascii_whitespace();
    let list_kv_pairs = map_fn(task_file_path, &content);

    let mut tally: HashMap<String, HashCounter> = HashMap::with_capacity(list_kv_pairs.len());
    for kv in list_kv_pairs.iter() {
        if !tally.contains_key(&kv.key) {
            tally.insert(
                kv.key.clone(),
                HashCounter {
                    key: kv.key.clone(),
                    values: vec![kv.value.clone()],
                },
            );

            continue;
        } else {
            match tally.get_mut(&kv.key) {
                Some(hash_counter) => {
                    hash_counter.values.push(kv.value.clone());
                }
                _ => (),
            };
        }
    }

    for (k, v) in tally {
        let _ = reduce_fn(k, v.values);
    }

    println!("[worker] done with {task_file_path}");
    Ok(list_kv_pairs)
}

pub fn thread_worker(instruction: WorkerInstruction) -> JoinHandle<()>{
    let handle = thread::spawn(move || {
        let result = worker(
            &instruction.file_path,
            instruction.map_fn,
            instruction.reduce_fn,
        );

        let _ = match result {
            Ok(success) => {
                instruction
                    .send_ch
                    .send(Some(success))
                    .expect("[worker] could not send. orphaned. Coordinator has been closed");
                drop(instruction.send_ch);
            }
            Err(err) => {
                println!("[worker-error]: {err}",);
                instruction
                    .send_ch
                    .send(None)
                    .expect("[worker] could not send. orphaned. Coordinator has been closed");
                drop(instruction.send_ch);
                return;
            }
        };
    });

    handle
}
