use std::collections::HashSet;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn kmeridentity(pathvec: Vec<String>, window: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let pathkmer = pathvec;
    let mut hashkmer: HashSet<Vec<&str>> = HashSet::new();
    for i in pathkmer.iter() {
        let sequencekmer = i
            .as_bytes()
            .windows(window.parse::<usize>().unwrap())
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        hashkmer.insert(sequencekmer);
    }
    let finalkmers = hashkmer
        .iter()
        .flatten()
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

    /*
      sequence content of the entire sequence
    */

    let mut sequencecontent: Vec<usize> = Vec::new();
    for i in pathkmer.iter() {
        let mut content_a: usize = 0usize;
        let mut content_g: usize = 0usize;
        let mut content_t: usize = 0usize;
        let mut content_c: usize = 0usize;
        let seq: Vec<char> = i.chars().collect::<Vec<char>>();
        for i in seq.iter() {
            match i {
                'A' => content_a += 1usize,
                'T' => content_t += 1usize,
                'G' => content_g += 1usize,
                'C' => content_c += 1usize,
                _ => continue,
            }
        }
        sequencecontent
            .push((content_g + content_c) / (content_a + content_t + content_g + content_c));
    }

    /*
    kmer count of the seq
    */

    let mut kmercount: Vec<usize> = Vec::new();
    for i in pathkmer.iter() {
        for val in finalkmers.iter() {
            let mut countadd: usize = 0usize;
            let seqkmer = i
                .as_bytes()
                .windows(window.parse::<usize>().unwrap())
                .map(|x| str::from_utf8(x).unwrap())
                .collect::<Vec<_>>();
            for seqkmerval in seqkmer.iter() {
                if seqkmerval == val {
                    countadd += 1usize;
                }
            }
            kmercount.push(countadd);
        }
    }

    /*
    sequence reconstructed from the unique kmer present in the seq
    */

    let mut uniquekmerseq: Vec<String> = Vec::new();
    for i in pathkmer.iter() {
        for val in finalkmers.iter() {
            let mut seqseq: HashSet<String> = HashSet::new();
            let seqkmer = i
                .as_bytes()
                .windows(window.parse::<usize>().unwrap())
                .map(|x| str::from_utf8(x).unwrap())
                .collect::<Vec<_>>();
            for seqkmerval in seqkmer.iter() {
                if seqkmerval == val {
                    seqseq.insert(seqkmerval.to_string());
                }
            }
            uniquekmerseq.push(
                seqseq
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .concat(),
            );
        }
    }

    /*
    sequence content of the unique kmer constructed seq
    */

    let mut uniquesequencecontent: Vec<usize> = Vec::new();
    for i in uniquekmerseq.iter() {
        let mut content_a: usize = 0usize;
        let mut content_g: usize = 0usize;
        let mut content_t: usize = 0usize;
        let mut content_c: usize = 0usize;
        let seq: Vec<char> = i.chars().collect::<Vec<char>>();
        for i in seq.iter() {
            match i {
                'A' => content_a += 1usize,
                'T' => content_t += 1usize,
                'G' => content_g += 1usize,
                'C' => content_c += 1usize,
                _ => continue,
            }
        }
        uniquesequencecontent
            .push((content_g + content_c) / (content_a + content_t + content_g + content_c));
    }

    let mut finalretrunvec: Vec<Vec<usize>> = Vec::new();
    for i in sequencecontent.iter() {
        for val in kmercount.iter() {
            for finalseq in uniquesequencecontent.iter() {
                finalretrunvec.push(vec![*i, *val, *finalseq]);
            }
        }
    }

    Ok(finalretrunvec)
}
