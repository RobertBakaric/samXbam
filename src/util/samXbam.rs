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

use rust_htslib::prelude::*;
use rust_htslib::bam;
use rust_htslib::sam;

use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;


use rust_htslib::*;
use rust_htslib::bam::*;
use rust_htslib::bam::header::*;





// design interface

pub fn sam2bam (samfile: &str, bamfile: &str){

    let mut header = Header::new();

    {     // fn read header

        let file = File::open(samfile).unwrap();
        let re = Regex::new(r"@(\w+?)\tSN:(.*?)\tLN:(\d+)").unwrap();  // simplyfy the rexeq


        for line in BufReader::new(file).lines() {
            let r = line.unwrap();
            match re.captures(&r) {
                Some(caps) => {                               // match interface here 
                    header.push_record(
                    HeaderRecord::new(&(caps[1].as_bytes()))   // only one push for multiple records depending on a recod
                    .push_tag(b"SN", &(caps[2].to_string()))
                    .push_tag(b"LN", &(caps[3].to_string().parse::<usize>().unwrap())),   // fn str_slice to int (multiple)
                    );
                }
                None => {
                    break;
                }

            }
      
        }

    }


        
        let file = File::open(samfile).unwrap();

        let mut bam_printer =   if bamfile != "stdout" { 
                                    bam::Writer::from_path(bamfile, &header).unwrap()
                                }else{
                                    bam::Writer::from_stdout(&header).unwrap()
                                };

        let _header = HeaderView::from_header(&header);
        let re = Regex::new(r"^@.*").unwrap();

        for line in BufReader::new(file).lines() {
        let r = line.unwrap();
    
        match re.captures(&r) {
            Some(caps) => {

                continue;
            }
            None => {
              let mut l  = Record::from_sam(&_header, r.as_bytes()).unwrap();
              bam_printer.write(&l);
            }

        }
    }
}



pub fn bam2sam(bamfile: &str, samfile: &str){

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
        sam_printer.write(&r);
    }

 
}