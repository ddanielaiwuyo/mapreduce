#![allow(unused)]
use crate::worker;
use std::sync::mpsc;

#[derive(PartialEq, Debug)]
enum State {
    Idle,
    InProgress,
    Done,
}
#[derive(Debug)]
pub struct Task {
    pub file_name: String,
    state: State,
}

impl Task {
    pub fn new(file_name: String) -> Task {
        let state = State::Idle;
        Task { file_name, state }
    }
}

pub fn coordinator(
    tasks: Vec<Task>,
    map_fn: worker::MapFn,
    reduce_fn: worker::ReduceFn,
) -> Result<(), Box<dyn std::error::Error>> {
    let (worker_send_ch, recv_ch) = mpsc::channel::<Option<Vec<worker::MKeyValue>>>();
    let mut worker_handles = vec![];
    for mut task in tasks {
        let file_path = task.file_name;
        let send_ch = worker_send_ch.clone();
        let instruction = worker::WorkerInstruction {
            send_ch,
            file_path,
            map_fn,
            reduce_fn,
        };

        let handle = worker::thread_worker(instruction);
        worker_handles.push(handle);

        task.state = State::InProgress;
    }

    // TODO(next_thing)
    // 1. Make sure that as each thread reports back, they report with the id
    // so we can update the task to State::Done. We could also consider changing
    // the tasks to a `Stack` so we can pop tasks that have already be done, or better still a hashmap
    // 2. Decide whether workers save their output to a dest file or the coordinator does
    let mut results = vec![];
    drop(worker_send_ch);
    for result in recv_ch {
        results.push(result)
    }

    println!("[coordinator] all workers done");
    for result in results {
        match result {
            Some(value) => println!("{:?}", value.last()),

            None => (),
        }
    }

    for handle in worker_handles {
        match handle.join() {
            Ok(_) => (),
            Err(err) => println!("err: {:?}", err),
        };
    }
    Ok(())
}
