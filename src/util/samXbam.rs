/*
* Copyright (c) <2019> <Robert Bakaric : rbakaric@exaltum.eu>
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use rust_htslib::bam;
use rust_htslib::sam;

use rust_htslib::bam::*;
use std::io::{Result};
use util::sam::*;





pub fn sam2bam(samfile: &str, bamfile: &str)-> Result<()>{

    let sam_reader = ReadSam::new(samfile);
    let header = sam_reader.header();

    let mut bam_printer =   if bamfile != "stdout" { 
                                bam::Writer::from_path(bamfile, &header ).unwrap()
                            }else{
                                bam::Writer::from_stdout( &header).unwrap()
                            };

// copy reads to new BAM file
    for record in sam_reader.records() {
       let mut l  = Record::from_sam(&(HeaderView::from_header(&header)), record.as_bytes()).unwrap();
       bam_printer.write(&l).expect("Line not formated properly !");
    }

 Ok(())
}



pub fn bam2sam(bamfile: &str, samfile: &str)-> Result<()>{

    let mut bam_reader = bam::Reader::from_path(bamfile).unwrap();

    let header = bam::Header::from_template(bam_reader.header());

    let mut sam_printer =   if samfile != "stdout" { 
                                sam::Writer::from_path(samfile, &header).unwrap()
                            }else{
                                sam::Writer::from_stdout(&header).unwrap()
                            };

// copy reads to new SAM file
    for record in bam_reader.records() {
        let r = record.unwrap();
        sam_printer.write(&r).expect("Line not formated properly !");
    }

 Ok(())
}
