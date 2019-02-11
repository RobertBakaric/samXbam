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


use rust_htslib::bam::*;
use rust_htslib::bam::header::*;

use std::fs::File;
use std::io::{BufReader,Read,BufRead,Result};
use regex::Regex;




#[derive(Debug)]
pub struct ReadSam{
    samfile: String,
    header: Header,
    fh: File,
    
}





pub trait Getters {
	fn get_filename(self)->String;
	fn header(self)->Header;
}


trait Parsers {
	fn parse_header(file: &str)->Header;
}



impl  ReadSam {
    pub fn new(file: &str)-> ReadSam{
            ReadSam{
                samfile: file.to_string(),
                header: ReadSam::parse_header(file),
                fh: File::open(file).unwrap(),
            }
        }
	 pub fn records(self) -> impl Iterator<Item = String> {
        BufReader::new(self.fh)
            .lines()
            .map(Result::unwrap)
            .filter(|s| !Regex::new(r"^@").unwrap().is_match(s))
    }

}



impl Getters for ReadSam{

	fn get_filename (self) -> String {
		self.samfile
	}
	
	fn header(self)-> Header{
		self.header
	}

}





impl Parsers for ReadSam {
	fn parse_header (file: &str) -> Header {
		let mut header = Header::new();
		let fh = File::open(file).expect("File not SAM file !");
		
		let re_begin = Regex::new(r"^@(\w{2})").unwrap();
        let re_iter  = Regex::new(r"\t(\w{2}):(\w+)").unwrap();
        
        
        for line in BufReader::new(fh).lines() {
			let uline = line.unwrap();
			//println!("{}", uline);
			match re_begin.captures(&uline) {
                Some(_caps) => { 
					for mt in re_iter.captures_iter(&uline) { 
						//println!("{}: {} - {}", &_caps[1], &mt[1], &mt[2]);
						header.push_record(
							HeaderRecord::new(&(_caps[1].as_bytes()))
							.push_tag(&(mt[1].as_bytes()), &(mt[2].to_string()))
							);
					}
                }
                None => {
                    break;
                }

            }
		}
		//HeaderView::from_header(&header)
		header
	}
}


