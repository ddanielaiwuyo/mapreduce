use mapreduce::worker;
use mapreduce::{custom_map, custom_reduce};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn write_excerpt(filename: &str, content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = PathBuf::from(worker::RESULT_DIR).join(filename);
    let mut fd = fs::File::create(&full_path)?;
    fd.write_all(content.as_bytes())?;
    let s = full_path.to_string_lossy();
    Ok(s.to_string())
}

fn shuffle_phase(list_kv_pairs: Vec<worker::MKeyValue>) -> String {
    let mut tally: BTreeMap<String, worker::HashCounter> = BTreeMap::new();
    for kv in list_kv_pairs.iter() {
        if !tally.contains_key(&kv.key) {
            tally.insert(
                kv.key.clone(),
                worker::HashCounter {
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

    let mut full_content = String::from("");
    for (k, v) in tally {
        let out = mapreduce::custom_reduce(k, v.values);
        full_content.push_str(out.as_str());
    }

    full_content
}

#[test]
fn worker_produces_correct_map_output() -> Result<(), Box<dyn std::error::Error>> {
    let excerpt = "
        The result of this worker with this content should be the following:
        1. Content is saved to results/mr-out-filename
        2. `the` should appear with a count of 2
        3. `this` should appear with a count of 3
        4 `should` with a count of 4
        5 `with`  a count of 5
        6 `a`   count of 5
        7 `count`   of 6
        8 `of`   8
        ";

    let filename = "worker-test";
    let full_path = write_excerpt(filename, excerpt)?;
    let mut expected_list_kv_pairs = custom_map(&String::from("test_key"), excerpt);
    expected_list_kv_pairs.sort_by_key(|k| k.key.clone());
    let actual_list_kv_pairs_result = worker::worker(&full_path, custom_map, custom_reduce)?;
    assert_eq!(expected_list_kv_pairs, actual_list_kv_pairs_result);
    let expected_result = shuffle_phase(expected_list_kv_pairs);

    // Read from outfile
    let out_file = format!("{}-{}", worker::FILE_PREFIX, filename);
    let out_file = PathBuf::from(worker::RESULT_DIR).join(out_file);
    let final_result = fs::read_to_string(&out_file)?;
    final_result.contains(expected_result.as_str());

    fs::remove_file(full_path)?;
    fs::remove_file(out_file)?;
    Ok(())
}
