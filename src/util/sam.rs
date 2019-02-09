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

use std::ffi;
use std::path::Path;
use std::ptr;
use std::slice;



use rust_htslib::*;
use rust_htslib::sam::*;
use rust_htslib::bam::*;
use rust_htslib::bam::header::*;

use std::fs::File;
use std::io::{BufReader,BufRead,Result};
use regex::Regex;






// design interface
/*
pub fn sam2bam (samfile: &str, bamfile: &str)-> Result<()>{

    let mut header = Header::new();

    {     // fn read header

        let file = File::open(samfile)?;
        let re = Regex::new(r"@(\w+?)\tSN:(.*?)\tLN:(\d+)").unwrap();  // simplyfy the rexeq

*/

/*
match iteratively

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let text = "2012-03-14, 2013-01-01 and 2014-07-05";
for cap in re.captures_iter(text) {
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
}
*/


/*
Outsourcing lines()

 let mut f = BufReader::new(file);
    let mut count = 0;
    let mut line = String::new();
    if let Ok(mut bytes) = f.read_line(&mut line) {
        while bytes > 0 {
            // Do something with the line
            count += 1;
            line.clear();
            match f.read_line(&mut line) {
                Ok(b) => bytes = b,
                Err(_) => {},
            }
        }
    }

pub struct Lines<B> {
    buf: B,
}

lines iterator , combine with the above one 

#[stable(feature = "rust1", since = "1.0.0")]
impl<B: BufRead> Iterator for Lines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}




        for line in BufReader::new(file).lines() {
            let r = line.unwrap();
            match re.captures(&r) {
                Some(_caps) => {                               // match interface here 
                    header.push_record(
                    HeaderRecord::new(&(_caps[1].as_bytes()))   // only one push for multiple records depending on a recod
                    .push_tag(b"SN", &(_caps[2].to_string()))
                    .push_tag(b"LN", &(_caps[3].to_string().parse::<usize>().unwrap())),   // fn str_slice to int (multiple)
                    );
                }
                None => {
                    break;
                }

            }
      
        }

    }


        
        let file = File::open(samfile)?;

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
            Some(_caps) => {

                continue;
            }
            None => {
              let mut l  = Record::from_sam(&_header, r.as_bytes()).unwrap();
              bam_printer.write(&l).expect("Line not formated properly !");
            }

        }
    }
    Ok(())
}


*/





/*  To read a header directly !!!

fn header(path: &[u8]) -> Result<()> {
        let file = try!(bgzf_open(&ffi::CString::new(path).unwrap(), b"r"));
        let header = unsafe { htslib::sam_hdr_read(file) };

        Ok(())
//        Ok(Reader {
//            bgzf: file,
//            header: HeaderView::new(header),
//       }) 
}


*/

// Define traits for the sam object


// A SAM reader.

pub struct Lines<B> {
    buf: B,
}


#[derive(Debug)]
pub struct reader {
    line_count: usize,
    samfile: String,
    header: HeaderView,
}


pub trait Getters {
	fn get_filename(&self)->&str;
	fn header(&self)->&HeaderView;
}


pub trait Parsers {
	fn parse_header(file: &str)->HeaderView;
}







impl reader {
    pub fn new(file: &str)->  Self{
            reader{
                line_count: 0,
                samfile: file.to_string(),
                header: reader::parse_header(file),
            }
        }


}



impl<B: BufRead> Iterator for Lines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }
    }
}


impl Getters for reader {
	fn get_filename (&self) -> &str {
		&self.samfile
	}
	fn header(&self)-> &HeaderView{
		&self.header
	}
}




impl Parsers for reader {
	fn parse_header (file: &str) -> HeaderView {
		let mut header = Header::new();
		let fh = File::open(file).expect("File not sam !");
		
		let re_begin = Regex::new(r"^@(\w{2})").unwrap();
        let re_iter  = Regex::new(r"\t(\w{2}):(\w+)").unwrap();  // simplyfy the rexeq
        
        
        for line in BufReader::new(fh).lines() {
			let uline = line.unwrap();
			println!("{}", uline);
			match re_begin.captures(&uline) {
                Some(_caps) => { 
					for mt in re_iter.captures_iter(&uline) { 
						println!("{}: {} - {}", &_caps[1], &mt[1], &mt[2]);
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
		HeaderView::from_header(&header)
	}
}



/*
impl Read for Reader {
    fn read(&mut self, record: &mut record::Record) -> Result<(), ReadError> {
        let header = unsafe { htslib::sam_hdr_read(self.bgzf) };
        match unsafe { htslib::sam_read1(self.bgzf, header, record.inner) } {
            -1 => Err(ReadError::NoMoreRecord),
            -2 => Err(ReadError::Truncated),
            -4 => Err(ReadError::Invalid),
            _ => Ok(()),
        }
    }

    /// Iterator over the records of the fetched region.
    /// Note that, while being convenient, this is less efficient than pre-allocating a
    /// `Record` and reading into it with the `read` method, since every iteration involves
    /// the allocation of a new `Record`.
    fn records(&mut self) -> Records<Self> {
        Records { reader: self }
    }


    fn bgzf(&self) -> *mut htslib::samFile {
        self.bgzf
    }



    fn header(&self) -> &HeaderView {
        &self.header
    }
}

*/


