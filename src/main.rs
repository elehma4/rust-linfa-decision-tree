use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::{array, s, Array2, Axis};
use std::error::Error;

mod training_data;

fn main() -> Result<(), Box<dyn Error>> {
    // Load training data from external file
    let (training_data, feature_names) = training_data::get_training_data();

    // Separate features and labels for training
    let num_features = training_data.len_of(Axis(1)) - 1;
    let train_features = training_data.slice(s![.., 0..num_features]).to_owned();
    let train_labels = training_data.column(num_features).to_owned().map(|&x| x as usize);

    // Create dataset from features and labels
    let train_dataset = Dataset::new(train_features, train_labels)
        .with_feature_names(feature_names.clone());

    // Create a decision tree model using the Gini split quality
    let model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .fit(&train_dataset)?;

    // Test data (features only, without labels)
    let test_data: Array2<f32> = array!(
        [0., 1., 200., 0.], // Less social media, has a pet dog, moderate Rust LOC, didn't eat meat
        [1., 0., 80., 1.],  // Active on social media, no pet dog, few Rust LOC, ate meat
        [0., 1., 250., 1.], // Less social media, has a pet dog, many Rust LOC, ate meat
        [1., 0., 120., 0.], // Active on social media, no pet dog, some Rust LOC, didn't eat meat
        [0., 1., 180., 1.], // Less social media, has a pet dog, moderate Rust LOC, ate meat
        [1., 0., 150., 0.], // Active on social media, no pet dog, moderate Rust LOC, didn't eat meat
        [0., 1., 300., 1.], // Less social media, has a pet dog, many Rust LOC, ate meat
        [1., 0., 60., 1.],  // Active on social media, no pet dog, few Rust LOC, ate meat
        [0., 1., 400., 0.], // Less social media, has a pet dog, a lot of Rust LOC, didn't eat meat
        [1., 0., 100., 0.]  // Active on social media, no pet dog, some Rust LOC, didn't eat meat
    );

    // Predict the labels for the test data
    let predictions = model.predict(&test_data);

    // Generate a CLI text response explaining the prediction for each data point
    for (i, (input, &prediction)) in test_data.outer_iter().zip(predictions.iter()).enumerate() {
        let human_condition = match prediction {
            0 => "Sad",
            1 => "Neutral",
            2 => "Happy",
            _ => "Unknown",
        };

        println!(
            "Data Point {}: Social Media: {}, Pet Dog: {}, Rust LOC: {}, Ate Meat: {} -> Predicted Condition: {}",
            i + 1,
            input[0],
            input[1],
            input[2],
            input[3],
            human_condition
        );
    }

    Ok(())
}