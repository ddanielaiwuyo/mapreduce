mod coordinator;
mod worker;

// Source: http://nil.csail.mit.edu/6.5840/2025/labs/lab-mr.html
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is a stub, that we'll refactor to RPC's while been
    // multithreaded. This is supposed to simulate a worker
    // asking the co-ordinator main for a task, and when they also report they're done w a task. But for now, we're asking for
    // a specific task
    // let task_file_path = get_task("tasks/the_hemingway.txt", false);
    {
        let _rpc_response = worker::ask_task();
    }

    // worker::worker(&task_file_path, custom_map, custom_reduce)?;
    let current_tasks = vec![
        coordinator::Task::new(String::from("tasks/the_hemingway.txt")),
        coordinator::Task::new(String::from("tasks/carvan.txt")),
        coordinator::Task::new(String::from("tasks/poem.txt")),
    ];

    let _ = coordinator::coordinator(current_tasks, custom_map, custom_reduce);
    Ok(())
}

fn custom_map(key: &String, value: &str) -> Vec<worker::MKeyValue> {
    println!("[map]: {}", key);

    let mut cleaned_content: String = String::with_capacity(value.len());

    for ch in value.chars() {
        if ch.is_whitespace() || ch.is_alphabetic() {
            let us = format!("{}", ch);
            cleaned_content += &us;
        }
    }

    let value = cleaned_content.to_ascii_lowercase();

    let words = value.split_ascii_whitespace();
    let mut kv_pairs = vec![];
    for word in words {
        kv_pairs.push(worker::MKeyValue {
            key: String::from(word),
            value: String::from("1"),
        });
    }

    kv_pairs
}

fn custom_reduce(key: String, values: Vec<String>) -> String {
    let mut curr_count: u32 = 0;
    for val in values {
        let s = val.parse::<u32>().unwrap_or_default();
        curr_count += s;
    }

    format!("{} -> {}", key, curr_count)
}
