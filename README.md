# klebnz

- giant kelp machine learning approach. 
- to normalize the temperature variation, implemented a distance calculation as Depth * Temperature / Temperature - Depth.
- it implements the logistic and decision classifier, a grid search random forest classifier and a knn classifier. 
- added a expressionseq to classify the decay of the kelp based on the expression values and kmer content and depth.
- The math of the normalization is given with in the file.

```
cargo build

```

```
_      _          _       _   _   _____
| | __ | |   ___  | |__   | \ | | |__  /
| |/ / | |  / _ \ | '_ \  |  \| |   / / 
|   <  | | |  __/ | |_) | | |\  |  / /_ 
|_|\_\ |_|  \___| |_.__/  |_| \_| /____|
                                       

Machine learning classifier for Kleb
     ************************************************
     Gaurav Sablok,
     Email: codeprog@icloud.com
    ************************************************

Usage: klebnz <COMMAND>

Commands:
klebseq        classify according to the logistic and decision classifier
random-seq     classify according to the Random forest
knn-classify   classify according to the KNN classifier
expressionseq  sequence to expression regressor
help           Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version


```

Gaurav Sablok \
codeprog@icloud.com
