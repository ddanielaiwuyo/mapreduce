mod worker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is a stub, that we'll refactor to RPC's while been
    // multithreaded. This is supposed to simulate a worker
    // asking the co-ordinator main for a task, and when they
    // also report they're done w a task. But for now, we're asking for
    // a specific task
    let task_file_path = get_task("tasks/the_hemingway.txt", false);
    {
        let _rpc_response = worker::ask_task();
    }

    worker::worker(&task_file_path, custom_map, custom_reduce)?;
    Ok(())
}

#[derive(PartialEq, Debug)]
enum State {
    Idle,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    file_name: String,
    state: State,
}

fn get_task(task_name: &str, ask: bool) -> String {
    println!("[CO-ORDINATOR] Getting task");
    let mut current_tasks = vec![
        Task {
            file_name: String::from("tasks/the_hemingway.txt"),
            state: State::Idle,
        },
        Task {
            file_name: String::from("tasks/carvan.txt"),
            state: State::Idle,
        },
        Task {
            file_name: String::from("tasks/poem.txt"),
            state: State::Idle,
        },
    ];

    for task in &mut current_tasks {
        if task.file_name == task_name && ask && task.state == State::InProgress {
            task.state = State::Done;
            break;
        }
    }

    for task in &mut current_tasks {
        if task.state == State::Idle {
            task.state = State::InProgress;
            println!("[CO-ORDINATOR] Assigning task: {:?}", task);
            return task.file_name.clone();
        }
    }

    String::from("")
}

fn custom_map(key: &String, value: &str) -> Vec<worker::MKeyValue> {
    println!("[USER_MAP]: Starting implementation for {}", key);

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
