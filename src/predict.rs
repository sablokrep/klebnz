use crate::datstruct::Fasta;
use crate::datstruct::PathStruct;
use crate::kmerreg::kmeridentity;
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn predictmatrix(
    pathfasta: &str,
    expressionfile: &str,
    windowsize: &str,
) -> Result<DenseMatrix<f64>, Box<dyn Error>> {
    let fileopen = File::open(pathfasta).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut id: Vec<String> = Vec::new();
    let mut name: Vec<String> = Vec::new();
    let mut namestruct: Vec<Fasta> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with(">") {
            id.push(line.split(">").collect::<Vec<_>>()[1].to_string());
        }
        if !line.starts_with(">") {
            name.push(line)
        }
    }

    for i in 0..id.len() {
        namestruct.push(Fasta {
            id: id[1].clone(),
            name: name[i].clone(),
        })
    }

    let svec: Vec<String> = namestruct
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>();
    let matrix = creatematrix(svec.clone()).unwrap();
    let additionalkmermatrix = kmeridentity(svec.clone(), windowsize).unwrap();
    let mut finalmatrix: Vec<Vec<f64>> = Vec::new();
    for i in matrix.iter() {
        for val in additionalkmermatrix.iter() {
            let valinitial = ndarray::Array1::from_vec(i.clone());
            let valinsert = val
                .iter()
                .map(|x| x.to_string().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            let valinsertmatrix = ndarray::Array1::from_vec(valinsert.clone());
            let finalmatrixinsert = valinitial + valinsertmatrix;
            finalmatrix.push(finalmatrixinsert.to_vec());
        }
    }

    /*
     *
     * Making a densematrix from all the Vec<f64>
     */

    let pathstructunravel = PathStruct {
        pathexpression: expressionfile.to_string(),
        pathfasta: pathfasta.to_string(),
        windowsize: windowsize.to_string(),
    };

    let pathstructopen = pathstructunravel.finalvecrelease().unwrap();
    let mut densevector: Vec<Vec<f64>> = Vec::new();
    for i in finalmatrix.iter() {
        for val in pathstructopen.iter() {
            let valadd = ndarray::Array1::from_vec(i.clone());
            let valaddmatrix = ndarray::Array1::from_vec(val.clone());
            let densematix = valadd + valaddmatrix;
            densevector.push(densematix.to_vec());
        }
    }

    let finalrelease: DenseMatrix<f64> = DenseMatrix::from_2d_vec(&densevector).unwrap();
    Ok(finalrelease)
}

pub fn creatematrix(vecstring: Vec<String>) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let vecstring = choplength(vecstring).unwrap();
    let vecstore: Vec<Vec<f64>> = Vec::new();
    for i in vecstring.iter() {
        let stringchar = i.chars().collect::<Vec<_>>();
        let mut stringvec: Vec<Vec<f64>> = Vec::new();
        for i in stringchar.iter() {
            match i {
                'A' => stringvec.push(vec![1.0, 0.0, 0.0, 0.0]),
                'T' => stringvec.push(vec![0.0, 1.0, 0.0, 0.0]),
                'G' => stringvec.push(vec![0.0, 0.0, 1.0, 0.0]),
                'C' => stringvec.push(vec![0.0, 0.0, 0.0, 1.0]),
                _ => continue,
            }
        }
    }
    Ok(vecstore)
}

pub fn choplength(path: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let vecclone = path.clone();
    let mut seqlength: Vec<usize> = Vec::new();
    let mut finalstring: Vec<String> = Vec::new();
    for i in vecclone.iter() {
        seqlength.push(i.len());
    }
    let maxvalue = seqlength.iter().max().unwrap();
    for i in vecclone.iter() {
        let stringval = i.clone();
        let stringlength = stringval.len();
        if stringlength == *maxvalue {
            finalstring.push(stringval)
        } else if stringlength > *maxvalue {
            let diff = maxvalue - stringlength;
            let stringchop = stringval[0..diff].to_string();
            finalstring.push(stringchop);
        }
    }
    Ok(finalstring)
}
