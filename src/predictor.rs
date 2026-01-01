use std::path::PathBuf;
use ort::{session::Session, value::Value, inputs};
use std::sync::Mutex;
use crate::models::{BatchInferenceResponse, InferenceResponse};
// Required for thread-safe interior mutability

pub struct ModelManager {
    // We wrap the Session in a Mutex so we can borrow it mutably
    // even when ModelManager itself is immutable (&self)
    session: Mutex<Session>,
}

impl ModelManager {
    pub fn new(model_path: &PathBuf) -> Self {
        let session = Session::builder()
            .expect("Failed to create builder")
            .commit_from_file(model_path)
            .expect("Failed to load model");

        Self {
            session: Mutex::new(session),
        }
    }

    // This stays &self (immutable) to satisfy Axum/Handlers
    pub fn predict(&self, input_data: Vec<f32>) -> (String, f32) {
        // 1. Create the input tensor
        let input_tensor = Value::from_array((vec![1, 4], input_data))
            .expect("Failed to create input tensor");

        let mut session_guard = self.session.lock().unwrap();
        let input_name = session_guard.inputs[0].name.clone();

        // 2. Execute inference - Note: No '?' inside the inputs macro
        let outputs = session_guard
            .run(inputs![input_name.as_str() => &input_tensor])
            .expect("Inference failed");

        // 3. Tell the compiler exactly what's inside the sequence: Maps with i64 keys and f32 values
        let output_seq = outputs[1]
            .try_extract_sequence::<ort::value::MapValueType<i64, f32>>(&Default::default())
            .expect("Failed to extract sequence at index 1");

        // 4. Get the first map in the sequence
        let first_map = &output_seq[0];

        // 5. Extract the data from that map
        let map_data = first_map
            .try_extract_map::<i64, f32>()
            .expect("Failed to extract Map from sequence");

        // 6. Find the highest probability (same as before)
        let (max_class_idx, max_prob) = map_data
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&k, &v)| (k, v))
            .expect("Probability map was empty");

        // 7. Map to labels
        let labels = ["setosa", "versicolor", "virginica"];
        let species = labels.get(max_class_idx as usize)
            .unwrap_or(&"unknown")
            .to_string();

        (species, max_prob)
    }

    pub fn predict_batch(&self, batch_data: Vec<Vec<f32>>) -> BatchInferenceResponse {
        let batch_size = batch_data.len();
        // Flatten the 2D Vec into a 1D Vec for the tensor [batch_size, 4]
        let flattened_data: Vec<f32> = batch_data.into_iter().flatten().collect();

        // 1. Create tensor with dynamic batch size [N, 4]
        let input_tensor = Value::from_array((vec![batch_size as i64, 4], flattened_data))
            .expect("Failed to create input tensor");

        let mut session_guard = self.session.lock().unwrap();
        let input_name = session_guard.inputs[0].name.clone();

        let outputs = session_guard
            .run(inputs![input_name.as_str() => &input_tensor])
            .expect("Inference failed");

        // 2. Extract the Sequence of Maps
        // Now the sequence length will be 'batch_size' instead of 1
        let output_seq = outputs[1]
            .try_extract_sequence::<ort::value::MapValueType<i64, f32>>(&Default::default())
            .expect("Failed to extract sequence");

        let mut results: Vec<InferenceResponse> = Vec::with_capacity(batch_size);
        let labels = ["setosa", "versicolor", "virginica"];

        // 3. Iterate through each map in the sequence (one per row)
        let results: Vec<InferenceResponse> = output_seq.iter().map(|map_val| {
            let map_data = map_val.try_extract_map::<i64, f32>().unwrap();
            let (&idx, &prob) = map_data.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();

            InferenceResponse {
                species: labels[idx as usize].to_string(),
                probability: prob,
            }
        }).collect();

        // Wrap the results in our explicit response struct
        BatchInferenceResponse {
            count: results.len(),
            results: results,
            model_name: "iris-svc-v1".to_string(),
        }
    }
}