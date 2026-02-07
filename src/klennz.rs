use smartcore::linalg::basic::matrix::DenseMatrix;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

type STRUCTTYPE = (f32, f32, f32, String);

pub fn load_data<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<(DenseMatrix<f64>, Vec<i32>), Box<dyn Error>> {
    let file = File::open(path).expect("file not present");
    let mut names: Vec<String> = Vec::new();
    let mut temp: Vec<f32> = Vec::new();
    let mut depth: Vec<usize> = Vec::new();
    let mut addon: Vec<f32> = Vec::new();
    let mut class: Vec<String> = Vec::new();
    let fileread = BufReader::new(file);
    for i in fileread.lines() {
        let line = i.expect("line not present");
        let linevec = line.split(";").collect::<Vec<_>>();
        names.push(linevec[0].to_string());
        temp.push(linevec[1].parse::<f32>().unwrap());
        depth.push(linevec[2].parse::<usize>().unwrap());
        class.push(linevec[3].to_string());
    }

    /*
    to normalize the depth and temperature a equation d*t/d-t
    */

    for i in temp.iter() {
        for val in depth.iter() {
            let value = i * *val as f32 / (*val as f32 - i);
            addon.push(value);
        }
    }

    let mut vectype: Vec<STRUCTTYPE> = Vec::new();
    for i in 0..temp.len() {
        vectype.push((temp[i], depth[i] as f32, addon[i], class[i].clone()));
    }

    let mut densematrix: Vec<Vec<f64>> = Vec::new();
    let mut class: Vec<i32> = Vec::new();
    for i in vectype.iter() {
        if i.3 == "Y" {
            densematrix.push(vec![i.0 as f64, i.1 as f64, i.2 as f64]);
            class.push(1);
        } else if i.3 == "N" {
            densematrix.push(vec![i.0 as f64, i.1 as f64, i.2 as f64]);
            class.push(0);
        } else {
            continue;
        }
    }

    let densematrixreturn: DenseMatrix<f64> = DenseMatrix::from_2d_vec(&densematrix).unwrap();

    Ok((densematrixreturn, class))
}

pub fn predict<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut returnvec: Vec<Vec<f64>> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        let linevec = line.split(";").collect::<Vec<_>>();
        returnvec.push(vec![linevec[0].parse::<f64>().unwrap()]);
    }
    Ok(returnvec)
}
