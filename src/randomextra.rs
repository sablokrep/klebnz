use crate::klennz::load_data;
use crate::klennz::predict;
use smartcore::api::SupervisedEstimator;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifierParameters;
use smartcore::ensemble::random_forest_classifier::{
    RandomForestClassifier, RandomForestClassifierSearchParameters,
};
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::model_selection::KFold;
use smartcore::model_selection::cross_validate;
use smartcore::model_selection::train_test_split;
use smartcore::tree::decision_tree_classifier::SplitCriterion;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn trainradom_agrresive(pathfile: &str, loadfile: &str) -> Result<String, Box<dyn Error>> {
    let loaddata = load_data(pathfile).unwrap();
    let predictfile = predict(loadfile).unwrap();
    let predictmatrix = DenseMatrix::from_2d_vec(&predictfile).unwrap();
    let search_params = RandomForestClassifierSearchParameters {
        criterion: vec![SplitCriterion::Gini, SplitCriterion::Entropy],
        max_depth: vec![Some(5), Some(10), None],
        min_samples_split: vec![2, 5, 10],
        n_trees: vec![50, 100, 200],
        ..Default::default()
    };
    // Perform grid search with cross-validation
    let mut best_score = 0.0;
    let mut best_params = Default::default();
    for params in search_params {
        let cv_result = cross_validate(
            RandomForestClassifier::new(),
            &loaddata.0,
            &loaddata.1,
            Default::default(),
            &KFold::default().with_n_splits(5),
            &accuracy,
        )
        .unwrap();
        let mean_score = cv_result.mean_test_score();
        if mean_score > best_score {
            best_score = mean_score;
            best_params = params;
        }
    }
    // Train the final model with the best parameters
    let best_rf = RandomForestClassifier::fit(&loaddata.0, &loaddata.1, best_params).unwrap();
    let modelpredict = best_rf.predict(&predictmatrix).unwrap();
    let mut filewrite = File::create("predictvalue.txt").expect("file not present");
    for i in modelpredict.iter() {
        writeln!(filewrite, "{}", i).expect("file not present");
    }

    Ok("model has been trained".to_string())
}

pub fn randomextra(
    pathfile: &str,
    loadfile: &str,
    treesinput: &str,
    depthinput: &str,
    samplesplit: &str,
) -> Result<String, Box<dyn Error>> {
    let loaddata = load_data(pathfile).unwrap();
    let predictfile = predict(loadfile).unwrap();
    let predictmatrix = DenseMatrix::from_2d_vec(&predictfile).unwrap();
    let (x_train, x_test, y_train, y_test) =
        train_test_split(&loaddata.0, &loaddata.1, 0.2, true, Some(10));
    let rf_params = RandomForestClassifierParameters::default()
        .with_n_trees(treesinput.parse::<u16>().unwrap())
        .with_max_depth(depthinput.parse::<u16>().unwrap())
        .with_min_samples_split(samplesplit.parse::<usize>().unwrap());
    let rf_classifier = RandomForestClassifier::fit(&x_train, &y_train, rf_params).unwrap();
    let y_hat = rf_classifier.predict(&predictmatrix).unwrap();
    let ypredict = rf_classifier.predict(&x_test).unwrap();
    let accuracymodel = accuracy(&y_test, &ypredict);
    let mut filewrite = File::create("output.txt").expect("file not present");
    for i in y_hat.iter() {
        writeln!(filewrite, "{:?}", i).expect("file not present");
    }
    println!("The accuracy of the model is {}", accuracymodel);
    Ok("file has been written".to_string())
}
