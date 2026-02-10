use crate::klennz::load_data;
use crate::klennz::predict;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::neighbors::knn_classifier::KNNClassifier;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn knnclassifier(pathfile: &str, loadfile: &str) -> Result<String, Box<dyn Error>> {
    let data = load_data(pathfile).unwrap();
    let matrix = predict(loadfile).unwrap();
    let matrixdense = DenseMatrix::from_2d_vec(&matrix).unwrap();
    let fitvalue = KNNClassifier::fit(&data.0, &data.1, Default::default()).unwrap();
    let predictvalue = fitvalue.predict(&matrixdense).unwrap();
    let mut filewrite = File::create("predictedtext.txt").expect("file not present");
    for i in predictvalue.iter() {
        writeln!(filewrite, "{}", i).expect("file not present");
    }
    Ok("The data has been written".to_string())
}
