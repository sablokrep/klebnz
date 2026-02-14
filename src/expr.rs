use crate::datstruct::FastaRecord;
use crate::datstruct::PathStruct;
use crate::kmerreg::kmeridentity;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
 */

impl PathStruct {
    pub fn expressionnormalize(&self) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
        let expressionopen = File::open(self.pathexpression.clone()).expect("file not found");
        let expressionread = BufReader::new(expressionopen);
        let mut expressionvec: Vec<Vec<i32>> = Vec::new();
        for i in expressionread.lines() {
            let line = i.expect("line not present");
            let linevec = line
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            expressionvec.push(vec![linevec[0], linevec[1]]);
        }
        Ok(expressionvec)
    }

    pub fn fastavec(&self) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
        let fastaunpack = read_fasta(&self.pathfasta).unwrap();
        let finalvec: Vec<&FastaRecord> = fastaunpack.iter().map(|x| x.1).collect();
        let finalvecseq = finalvec
            .iter()
            .map(|x| x.sequence.clone())
            .collect::<Vec<_>>();
        let returnvec = kmeridentity(finalvecseq, &self.windowsize).unwrap();
        Ok(returnvec)
    }

    /*
    normalization of the expression with respect to the depth

    expression*depth /mean(sequencecount + kmercount + uniquekmercountcontent)
    */

    pub fn finalvecrelease(&self) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
        let expressionunpack = self.expressionnormalize().unwrap();
        let fastaunpack = self.fastavec().unwrap();
        let mut fastaconvert: Vec<Vec<i32>> = Vec::new();
        for i in fastaunpack.iter() {
            let valueinsert = i
                .iter()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            fastaconvert.push(valueinsert)
        }

        let mut finalrelease: Vec<Vec<f64>> = Vec::new();
        for i in expressionunpack.iter() {
            for val in fastaconvert.iter() {
                let estimate: i32 = (i[0] * i[1])
                    / (val.iter().sum::<i32>() / val.len().to_string().parse::<i32>().unwrap());
                let finalinsert: Vec<f64> = vec![
                    i[0].to_string().parse::<f64>().unwrap(),
                    i[1].to_string().parse::<f64>().unwrap(),
                    val[0].to_string().parse::<f64>().unwrap(),
                    val[1].to_string().parse::<f64>().unwrap(),
                    val[2].to_string().parse::<f64>().unwrap(),
                    estimate.to_string().parse::<f64>().unwrap(),
                ];
                finalrelease.push(finalinsert);
            }
        }
        Ok(finalrelease)
    }
}

pub fn read_fasta<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<HashMap<String, FastaRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = HashMap::new();
    let mut current_id = String::new();
    let mut current_sequence = String::new();
    for line in reader.lines() {
        let line = line.expect("line not present");
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.insert(
                    current_id.clone(),
                    FastaRecord {
                        id: current_id.clone(),
                        sequence: current_sequence.clone(),
                    },
                );
                current_sequence.clear();
            }
            current_id = line[1..].to_string();
        } else {
            current_sequence.push_str(&line);
        }
    }

    if !current_id.is_empty() {
        records.insert(
            current_id.clone(),
            FastaRecord {
                id: current_id,
                sequence: current_sequence,
            },
        );
    }

    Ok(records)
}
