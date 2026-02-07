mod args;
mod klennz;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::klennz::load_data;
use crate::klennz::predict;
use clap::Parser;
use figlet_rs::FIGfont;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use smartcore::tree::decision_tree_classifier::DecisionTreeClassifier;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("klebNZ");
    println!("{}", repgenerate.unwrap());

    let args = CommandParse::parse();
    match &args.command {
        Commands::Klebseq {
            filepath,
            predictfile,
            thread,
        } => {
            let n_threads = thread.parse::<usize>().expect("thread must be a number");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let loaddata = load_data(filepath).unwrap();
                let predictfile = predict(predictfile).unwrap();
                let predictmatrix = DenseMatrix::from_2d_vec(&predictfile).unwrap();
                let logisticfit =
                    LogisticRegression::fit(&loaddata.0, &loaddata.1, Default::default()).unwrap();
                let logisticpredict = logisticfit.predict(&predictmatrix).unwrap();
                let accuracyvalue = accuracy(&loaddata.1, &logisticpredict);
                println!("The accuracy of the predicted model is {}", accuracyvalue);

                let random =
                    RandomForestClassifier::fit(&loaddata.0, &loaddata.1, Default::default())
                        .unwrap();
                let randompredict = random.predict(&predictmatrix).unwrap();
                let accuracy_random = accuracy(&loaddata.1, &randompredict);
                println!(
                    "The random forest classifier has the prediction accuracy:{}",
                    accuracy_random
                );
                let decisionclass =
                    DecisionTreeClassifier::fit(&loaddata.0, &loaddata.1, Default::default())
                        .unwrap();
                let decisionpredict = decisionclass.predict(&predictmatrix).unwrap();
                let decisionaccuracy = accuracy(&loaddata.1, &decisionpredict);
                println!(
                    "The decision classifier has the model accuracy:{}",
                    decisionaccuracy
                );
            });
        }
    }
}
