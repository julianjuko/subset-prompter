use csv;
use rayon::prelude::*;
use serde_json::{StreamDeserializer, Value};
use std::collections::HashSet;
use std::io::{self, stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    let mut input = String::new();

    println!("Please enter your filepath: ");
    io::stdin().read_line(&mut input).unwrap();
    let filepath: String = input.trim().to_string();

    println!("Please enter your data path: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let start_time = Instant::now();
    let data_path: Vec<&str> = input.trim().split(&['.'][..]).collect();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filepath)
        .unwrap();

    // Determine which column contains the JSON blob.
    let blob_column_index = reader
        .headers()
        .unwrap()
        .iter()
        .position(|header| header.contains("blob"))
        .expect("Could not find a column named 'blob'");

    // Use Arc (Atomic Reference Counter) and Mutex (Mutual Exclusion) for thread-safe shared state.
    let result_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    // Create a counter for processed blobs.
    let processed_blobs_counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    // Convert records into a parallel iterator and process them in parallel.
    reader.records().par_bridge().for_each(|result| {
        // Ensure no error occurred.
        let record = result.unwrap();

        // Create a stream deserializer
        let stream_deserializer: StreamDeserializer<_, Value> =
            serde_json::Deserializer::from_str(&record[blob_column_index]).into_iter::<Value>();

        // Navigate into the blob based on provided data path.
        for value in stream_deserializer {
            match value {
                Ok(v) => navigate_blob(&v, &data_path, &result_set),
                Err(_) => continue,
            }
        }

        // Increase processed blobs counter and print progress.
        *processed_blobs_counter.lock().unwrap() += 1;
        print!(
            "\rBlobs processed: {}. Unique values collected: {}",
            *processed_blobs_counter.lock().unwrap(),
            result_set.lock().unwrap().len()
        );
        stdout().flush().unwrap(); // Flush stdout to update on same line
    });

    let binding = result_set.lock().unwrap();
    let mut result_vec: Vec<_> = binding.iter().collect();

    result_vec.sort();

    println!("\nDone.");
    println!("\nSorted unique values:");

    for value in result_vec {
        println!("{}", value);
    }

    let duration = start_time.elapsed();

    println!(
        "\nTime elapsed is: {} minutes {:.2} seconds",
        duration.as_secs() / 60,
        duration.as_secs_f64() % 60.0
    );
}

fn navigate_blob(blob: &Value, data_path: &[&str], set_arc: &Arc<Mutex<HashSet<String>>>) {
    if data_path.is_empty() {
        return;
    }

    match blob {
        Value::Object(map) => {
            if data_path[0] == "" {
                // This is an abstraction level.
                for (_, value) in map.iter() {
                    navigate_blob(value, &data_path[1..], set_arc);
                }
            } else if let Some(value) = map.get(data_path[0]) {
                if data_path.len() == 1 {
                    // We've reached the last item in our data path
                    if let Value::String(s) = value {
                        set_arc.lock().unwrap().insert(s.clone());
                    }
                } else {
                    // Continue navigating deeper into the blob
                    navigate_blob(value, &data_path[1..], set_arc);
                }
            }
        }
        Value::Array(arr) => {
            for value in arr.iter() {
                navigate_blob(value, &data_path[..], set_arc);
            }
        }
        _ => {}
    }
}
