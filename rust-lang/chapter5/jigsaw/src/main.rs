extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::io;
use std::vec::Vec;
use std::error::Error;

use csv;
use rand;
use rand::thread_rng;
use rand::seq::SliceRandom;
use vtext::vectorize::CountVectorizer;
use sprs::{CsMatBase, assign_to_dense};
use ndarray::{ArrayViewMut2, Array};

// use ml_utils;
// use ml_utils::sup_metrics::{accuracy, logloss_score};
// use ml_utils::datasets::Flower;

/// Multi class version of Logarithmic Loss metric.
///
/// # Arguments
/// * actual - Ground truth (correct) labels for n_samples samples.
/// * predicted - Predicted probabilities, as returned by a classifier’s predict method. If predicted.shape = (n_samples,) the probabilities provided are assumed to be that of the positive class. Keep in mind that the dimensions of actual and predicted should be the same.
/// * eps - Log loss is undefined for p=0 or p=1, so probabilities are clipped to max(eps, min(1 - eps, p)).
///
/// # Examples
///
/// ```
/// use jigsaw::multiclass_logloss;
/// let loss = multiclass_logloss() // complete this
/// ```
fn multiclass_logloss(actual: Vec<f32>, predicted: Vec<f32>, eps: f32) -> f32 {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
pub struct SpookyAuthor {
    id: String,
    text: String,
    author: String
}

impl SpookyAuthor {
    pub fn into_labels(&self) -> f32 {
        match self.author.as_str() {
            "EAP" => 0.,
            "HPL" => 1.,
            "MWS" => 2.,
            l => panic!("Not able to parse the target. Some other target got passed. {:?}", l),
        }
    }
}

fn build_vocabulary(data: &Vec<SpookyAuthor>) -> CountVectorizer {
    let mut cv = CountVectorizer::new();
    let mut all_text = vec![];
    for spooky_author in data {
        all_text.push(spooky_author.text.clone());
    }
    cv.fit(&all_text[..]);
    // println!("{:?}", abc);
    // // let new_text = vec!["I market"];
    // let s = String::from("I market");
    // let new_text = vec![s];
    // let transformed = cv.transform(&new_text[..]);
    // println!("{:?}", transformed);
    cv
}

fn get_feature_vectors(data: &Vec<SpookyAuthor>, bow_model: &mut CountVectorizer) -> CsMatBase<i32, usize, std::vec::Vec<usize>, std::vec::Vec<usize>, std::vec::Vec<i32>> {
    let mut all_text = vec![];
    for spooky_author in data {
        all_text.push(spooky_author.text.clone());
    }
    bow_model.transform(&all_text[..])
}

pub fn main() -> Result<(), Box<Error>> {
    // Get all the data
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let r: SpookyAuthor = result?;
        data.push(r); // data contains all the records
        // break;
    }
    // println!("{:?}", data);
    let mut bow_model = build_vocabulary(&data);
    let feature_vectors = get_feature_vectors(&data, &mut bow_model);
    let feature_vectors_dense = feature_vectors.to_dense();

    let y_train: Vec<f32> = data.iter().map(|r| r.into_labels()).collect();
    let y_train = Array::from(y_train);
    println!("{:?}", y_train.shape());
    println!("{:?}", feature_vectors_dense.as_slice().unwrap().len());

    // now we can probably load them to different vectors.

    // check out the rust repl https://github.com/google/evcxr

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use csv;

    #[test]
    fn test_spooky_author() {
        let data = "\"id\",\"text\",\"author\"\n\"id26305\",\"This process, however, afforded me no means of ascertaining the dimensions of my dungeon; as I might make its circuit, and return to the point whence I set out, without being aware of the fact; so perfectly uniform seemed the wall.\",\"EAP\"\n\"id17569\",\"It never once occurred to me that the fumbling might be a mere mistake.\",\"HPL\"";
        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        let mut data = Vec::new();
        for result in rdr.deserialize() {
            let r: SpookyAuthor = result.unwrap();
            data.push(r); // data contains all the records
        }
        assert_eq!(data[0].author, "EAP");
    }

    #[test]
    fn test_spooky_author_into_label_vector() {
        let data = "\"id\",\"text\",\"author\"\n\"id26305\",\"This process, however, afforded me no means of ascertaining the dimensions of my dungeon; as I might make its circuit, and return to the point whence I set out, without being aware of the fact; so perfectly uniform seemed the wall.\",\"EAP\"\n\"id17569\",\"It never once occurred to me that the fumbling might be a mere mistake.\",\"HPL\"";
        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        let mut data = Vec::new();
        for result in rdr.deserialize() {
            let r: SpookyAuthor = result.unwrap();
            data.push(r); // data contains all the records
        }
        assert_eq!(data[0].into_labels(), 0.);
    }

}