use crate::worker;
use std::sync::mpsc;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Idle,
    InProgress,
    Done,
}
#[derive(Debug, Clone, PartialEq)]
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
    tasks: &mut Vec<Task>,
    map_fn: worker::MapFn,
    reduce_fn: worker::ReduceFn,
) -> Result<(), Box<dyn std::error::Error>> {
    let (send_response_ch, recv_ch) = mpsc::channel::<worker::Response>();
    // let (worker_send_ch, recv_ch) = mpsc::channel::<Option<Vec<worker::MKeyValue>>>();
    let mut worker_handles = vec![];
    for mut task in tasks.clone() {
        let file_path = task.file_name;

        let send_response = send_response_ch.clone();

        let instruction = worker::WorkerInstruction {
            send_response,
            file_path,
            map_fn,
            reduce_fn,
        };

        let handle = worker::thread_worker(instruction);
        worker_handles.push(handle);

        task.state = State::InProgress;
    }

    let mut responses = vec![];
    drop(send_response_ch);

    for response in recv_ch {
        responses.push(response)
    }

    for response in responses {
        let worker_id = response.id;
        match response.value {
            Some(_result) => {
                let mut iter = tasks.into_iter();
                while let Some(task) = iter.next() {
                    if task.file_name == worker_id {
                        task.state = State::Done;
                        break;
                    }
                }
            }

            None => {}
        };
    }

    println!(
        "
        ========================== ==========================
                    [coordinator-report] failed tasks
        ========================== ==========================
        "
    );
    for unfinished_task in tasks.iter().filter(|task| task.state.eq(&State::Idle)) {
        println!("  {unfinished_task:#?}");
    }

    for handle in worker_handles {
        match handle.join() {
            Ok(_) => (),
            Err(err) => println!("err: {:?}", err),
        };
    }
    Ok(())
}
