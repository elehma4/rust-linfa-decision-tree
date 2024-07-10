# Decision Tree Model with linfa

## Overview

This Rust program creates a decision tree model using the `linfa` and `linfa_trees` crates. The model is trained on a dataset with features related to social media activity, pet ownership, Rust lines of code, and dietary habits. The goal is to predict the human condition (Happy, Neutral, Sad) based on these features and provide a CLI response explaining the prediction for each data point. The program also exports the trained decision tree to a TikZ format file (`dt.tex`), which can be used for LaTeX document rendering.

## Dataset

### Training Data

The training dataset consists of 37 samples with 4 features and 1 label. Each row in the dataset represents a sample with the following features:
- Social Media Activity (0: Less, 1: Active)
- Pet Dog Ownership (0: No, 1: Yes)
- Rust Lines of Code (Numeric value)
- Ate Meat (0: No, 1: Yes)
- Mood (Label: 0: Sad, 1: Neutral, 2: Happy)

### Test Data

The test dataset consists of 10 samples with 4 features. Each row in the dataset represents a sample with the following features:
- Social Media Activity (0: Less, 1: Active)
- Pet Dog Ownership (0: No, 1: Yes)
- Rust Lines of Code (Numeric value)
- Ate Meat (0: No, 1: Yes)

### Example Test Data

| Social Media | Pet Dog | Rust Lines of Code | Ate Meat |
|--------------|---------|--------------------|----------|
| 0            | 1       | 200                | 0        |
| 1            | 0       | 80                 | 1        |
| 0            | 1       | 250                | 1        |
| 1            | 0       | 120                | 0        |
| 0            | 1       | 180                | 1        |
| 1            | 0       | 150                | 0        |
| 0            | 1       | 300                | 1        |
| 1            | 0       | 60                 | 1        |
| 0            | 1       | 400                | 0        |
| 1            | 0       | 100                | 0        |

## Program Flow

1. **Data Preparation**: The training data is stored in a separate file (`training_data.rs`) and loaded into the main program.
2. **Dataset Creation**: A linfa `Dataset` is created with features and labels from the training data.
3. **Model Training**: A decision tree model is created using the Gini split quality criterion and trained on the training dataset.
4. **Predictions**: The model predicts the labels for the test dataset.
5. **CLI Response**: A CLI text response is generated that explains the prediction for each data point by mapping the predicted label to human conditions (Sad, Neutral, Happy).
6. **Model Export**: The trained model is exported to TikZ format and saved to a file (`dt.tex`).

## Running the Program

To run the program, ensure you have Rust installed and the necessary dependencies (`linfa`, `linfa_trees`, `ndarray`) added to your `Cargo.toml` file.

### Cargo.toml

```toml
[dependencies]
linfa = "0.5.1"
linfa-trees = "0.5.1"
ndarray = "0.15.3"
