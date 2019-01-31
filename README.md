# samXbam  (UNDER CONSTRUCTION)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/RobertBakaric/samXbam/blob/master/LICENSE)

Simple sam <--> bam convertor designed to work as a standalone tool

## Installation

To install samXbam, first install Rust. samXbam is currently tested on Rust 1.32.0, but it is likely to work on other versions as well.

To install samXbam itself: 

```
cargo install samXbam
```

## Usage
```

./samXbam -h.

./samXbam -i in.bam -d b2s -o out.sam

./samXbam -i out.sam -d s2b -o in.bam


```


## License

The software is licensed under the  [MIT license](http://opensource.org/licenses/MIT).

