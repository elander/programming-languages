extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::io;
use std::vec::Vec;
use std::error::Error;
use std::iter::repeat;

use rusty_machine as rm;
// use rm::linalg::{Matrix, BaseMatrix};
use rm::linalg::Matrix;
use rm::learning::k_means::{KMeansClassifier, Forgy, RandomPartition, KPlusPlus};
use rm::learning::gmm::{CovOption, GaussianMixtureModel};
use rm::learning::dbscan::DBSCAN;
use rm::learning::pca::PCA;
use rm::learning::UnSupModel;
use csv;
use rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize)]
struct Flower {
    sepal_length: f64, // everything needs to be f64, other types wont do in rusty machine
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
}

impl Flower {
    fn into_feature_vector(&self) -> Vec<f64> {
        vec![self.sepal_length, self.sepal_width, self.sepal_length, self.petal_width]
    }

    fn into_labels(&self) -> Vec<f64> {
        match self.species.as_str() {
            "setosa" => vec![1., 0., 0.],
            "versicolor" => vec![0., 1., 0.],
            "virginica" => vec![0., 0., 1.],
            l => panic!("Not able to parse the label. Some other label got passed. {:?}", l),
        }
    }
}

fn output_separator() {
    let repeat_string = repeat("*********").take(10).collect::<String>();
    println!("{}", repeat_string);
    println!("");
}

fn main() -> Result<(), Box<Error>> {
    // Get all the data
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let r: Flower = result?;
        data.push(r); // data contains all the records
    }

    // shuffle the data.
    data.shuffle(&mut thread_rng());

    // separate out to train and test datasets.
    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    // differentiate the features and the labels.
    let flower_x_train: Vec<f64> = train_data.iter().flat_map(|r| r.into_feature_vector()).collect();
    // let flower_y_train: Vec<f64> = train_data.iter().flat_map(|r| r.into_labels()).collect();

    let flower_x_test: Vec<f64> = test_data.iter().flat_map(|r| r.into_feature_vector()).collect();
    // let flower_y_test: Vec<f64> = test_data.iter().flat_map(|r| r.into_labels()).collect();

    // COnvert the data into matrices for rusty machine
    let flower_x_train = Matrix::new(train_size, 4, flower_x_train);
    // let flower_y_train = flower_y_train.chunks(3).collect();
    let flower_x_test = Matrix::new(test_size, 4, flower_x_test);
    // let flower_y_test = Matrix::new(test_size, 3, flower_y_test);
    // let flower_y_test = flower_y_test.chunks(3).collect();

    const features_num: usize = 4;
    const clusters: usize = 3;

    // Create a Kmeans model with 3 clusters
    let model_type = "Kmeans";
    let mut model = KMeansClassifier::new(clusters);

    //Train the model
    println!("Training the {} model", model_type);
    model.train(&flower_x_train)?;

    let centroids = model.centroids().as_ref().unwrap();
    println!("Model Centroids:\n{:.3}", centroids);

    // Predict the classes and partition into
    println!("Predicting the samples...");
    let classes = model.predict(&flower_x_test).unwrap();
    println!("number of classes from kmeans: {:?}", classes.data().len());
    // println!("{:?}", classes.data().len());
    // println!("{:?}", flower_y_test);
    let repeat_string = repeat("*********").take(10).collect::<String>();
    println!("{}", repeat_string);
    println!("");

    // using a different initialising method.
    let mut model = KMeansClassifier::new_specified(3, 100, Forgy); // can use the RandomPartition

    //Train the model
    println!("Training the {} model", model_type);
    model.train(&flower_x_train)?;

    let centroids = model.centroids().as_ref().unwrap();
    println!("Model Centroids:\n{:.3}", centroids);

    // Predict the classes and partition into
    println!("Predicting the samples...");
    let classes = model.predict(&flower_x_test).unwrap();
    println!("number of classes from kmeans: {:?}", classes.data().len());
    // println!("{:?}", classes.data().len());
    // println!("{:?}", flower_y_test);
    let repeat_string = repeat("*********").take(10).collect::<String>();
    println!("{}", repeat_string);
    println!("");

    // Bring in Gaussian mixture models
    // Create gmm with k(=2) classes.
    let model_type = "Mixture model";
    let mut model = GaussianMixtureModel::new(2);
    model.set_max_iters(1000);
    model.cov_option = CovOption::Diagonal;

    //Train the model
    println!("Training the {} model", model_type);
    model.train(&flower_x_train)?;

    // Print the means and covariances of the GMM
    println!("{:?}", model.means());
    println!("{:?}", model.covariances());

    // Predict the classes and partition into
    println!("Predicting the samples...");
    let classes = model.predict(&flower_x_test).unwrap();
    println!("number of classes from GMM: {:?}", classes.data().len());
    // println!("{:?}", classes.data().len());
    // println!("{:?}", flower_y_test);

    // Probabilities that each point comes from each Gaussian.
    println!("number of Probablities from GMM: {:?}", classes.data().len());
    let repeat_string = repeat("*********").take(10).collect::<String>();
    println!("{}", repeat_string);
    println!("");

    // DBscan slagorithm
    // eps = 0.3 and min_samples = 10
    let model_type = "DBScan";
    let mut model = DBSCAN::new(0.3, 10);
    // let mut model = DBSCAN::default(); //the default is DBSCAN { eps: 0.5, min_points: 5, clusters: None, predictive: false, _visited: [], _cluster_data: None }
    model.set_predictive(true);

    //Train the model
    println!("Training the {} model", model_type);
    model.train(&flower_x_train)?;

    // clusters
    let clustering = model.clusters().unwrap();

    // Predict the classes and partition into
    println!("Predicting the samples...");
    let classes = model.predict(&flower_x_test).unwrap();

    output_separator();

    println!("Dimensionality reduction using PCA");
    let mut model = PCA::default();
    println!("{:?}", model);
    let mut model = PCA::new(2, true);
    model.train(&flower_x_train)?;

    println!("{:?}", model.predict(&flower_x_test)?);
    println!("{:?}", model.components());

    output_separator();

    Ok(())
}