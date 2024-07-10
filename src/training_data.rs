use ndarray::array;
use ndarray::Array2;

// Training data
pub fn get_training_data() -> (Array2<f32>, Vec<&'static str>) {
    let training_data: Array2<f32> = array!(
        [0., 1., 150., 0., 0.], // Less social media, has a pet dog, moderate Rust LOC, didn't eat meat, Sad
        [1., 0., 50., 1., 1.],  // Active on social media, no pet dog, few Rust LOC, ate meat, Neutral
        [0., 1., 300., 1., 2.], // Less social media, has a pet dog, many Rust LOC, ate meat, Happy
        [1., 0., 100., 0., 0.], // Active on social media, no pet dog, some Rust LOC, didn't eat meat, Sad
        [0., 1., 200., 1., 1.], // Less social media, has a pet dog, moderate Rust LOC, ate meat, Neutral
        [1., 0., 0., 0., 0.],   // Active on social media, no pet dog, no Rust LOC, didn't eat meat, Sad
        [0., 1., 400., 1., 2.], // Less social media, has a pet dog, a lot of Rust LOC, ate meat, Happy
        [1., 0., 150., 1., 1.], // Active on social media, no pet dog, moderate Rust LOC, ate meat, Neutral
        [0., 1., 50., 0., 0.],  // Less social media, has a pet dog, few Rust LOC, didn't eat meat, Sad
        [1., 0., 250., 1., 2.], // Active on social media, no pet dog, good amount of Rust LOC, ate meat, Happy
        [0., 1., 350., 0., 1.], // Less social media, has a pet dog, many Rust LOC, didn't eat meat, Neutral
        [0., 1., 100., 0., 0.], // Less social media, has a pet dog, some Rust LOC, didn't eat meat, Sad
        [1., 0., 75., 1., 1.],  // Active on social media, no pet dog, few Rust LOC, ate meat, Neutral
        [0., 1., 320., 1., 2.], // Less social media, has a pet dog, many Rust LOC, ate meat, Happy
        [1., 0., 180., 0., 0.], // Active on social media, no pet dog, moderate Rust LOC, didn't eat meat, Sad
        [0., 1., 210., 1., 1.], // Less social media, has a pet dog, moderate Rust LOC, ate meat, Neutral
        [1., 0., 10., 0., 0.],  // Active on social media, no pet dog, few Rust LOC, didn't eat meat, Sad
        [0., 1., 450., 1., 2.], // Less social media, has a pet dog, a lot of Rust LOC, ate meat, Happy
        [1., 0., 160., 1., 1.], // Active on social media, no pet dog, moderate Rust LOC, ate meat, Neutral
        [0., 1., 70., 0., 0.],  // Less social media, has a pet dog, few Rust LOC, didn't eat meat, Sad
        [1., 0., 270., 1., 2.], // Active on social media, no pet dog, good amount of Rust LOC, ate meat, Happy
        [0., 1., 370., 0., 1.], // Less social media, has a pet dog, many Rust LOC, didn't eat meat, Neutral
        [1., 0., 90., 0., 0.],  // Active on social media, no pet dog, some Rust LOC, didn't eat meat, Sad
        [0., 1., 220., 1., 1.], // Less social media, has a pet dog, moderate Rust LOC, ate meat, Neutral
        [1., 0., 20., 0., 0.],  // Active on social media, no pet dog, few Rust LOC, didn't eat meat, Sad
        [0., 1., 480., 1., 2.], // Less social media, has a pet dog, a lot of Rust LOC, ate meat, Happy
        [1., 0., 130., 1., 1.], // Active on social media, no pet dog, moderate Rust LOC, ate meat, Neutral
        [0., 1., 60., 0., 0.],  // Less social media, has a pet dog, few Rust LOC, didn't eat meat, Sad
        [1., 0., 230., 1., 2.], // Active on social media, no pet dog, good amount of Rust LOC, ate meat, Happy
        [0., 1., 330., 0., 1.], // Less social media, has a pet dog, many Rust LOC, didn't eat meat, Neutral
        [1., 0., 110., 0., 0.], // Active on social media, no pet dog, some Rust LOC, didn't eat meat, Sad
        [0., 1., 190., 1., 1.], // Less social media, has a pet dog, moderate Rust LOC, ate meat, Neutral
        [1., 0., 30., 0., 0.],  // Active on social media, no pet dog, few Rust LOC, didn't eat meat, Sad
        [0., 1., 420., 1., 2.], // Less social media, has a pet dog, a lot of Rust LOC, ate meat, Happy
        [1., 0., 140., 1., 1.], // Active on social media, no pet dog, moderate Rust LOC, ate meat, Neutral
        [0., 1., 40., 0., 0.]   // Less social media, has a pet dog, few Rust LOC, didn't eat meat, Sad
    );

    let feature_names = vec![
        "Social Media", "Pet Dog", "Rust Lines of Code", "Ate Meat"
    ];

    (training_data, feature_names)
}