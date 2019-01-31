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
use rust_htslib::bam::*;



/// A SAM reader.
#[derive(Debug)]
pub struct Reader {
    bgzf: *mut htslib::samFile,
    header: HeaderView,
}

unsafe impl Send for Reader {}



impl Reader {
    /// Create a new Reader from path.
    ///
    /// # Arguments
    ///
    /// * `path` - the path to open.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ReaderPathError> {
        match path.as_ref().to_str() {
            Some(p) if path.as_ref().exists() => Ok(try!(Self::new(p.as_bytes()))),
            _ => Err(ReaderPathError::InvalidPath),
        }
    }

    /// Create a new Reader from STDIN.
    pub fn from_stdin() -> Result<Self, BGZFError> {
        Self::new(b"-")
    }
 extern "C" fn pileup_read(
        data: *mut ::std::os::raw::c_void,
        record: *mut htslib::bam1_t,
    ) {
        let _self = unsafe { &*(data as *mut Self) };
        //unsafe { htslib::sam_read1(_self.bgzf, record) }
    }
    /// Create a new Reader from URL.
   // pub fn from_url(url: &Url) -> Result<Self, BGZFError> {
   //     Self::new(url.as_str().as_bytes())
   // }

    /// Create a new Reader.
    ///
    /// # Arguments
    ///
    /// * `path` - the path to open. Use "-" for stdin.
    fn new(path: &[u8]) -> Result<Self, BGZFError> {
        let bgzf = try!(htslib::hts_open(&ffi::CString::new(path).unwrap(), b"r"));
        let header = unsafe { htslib::sam_hdr_read(bgzf) };
        Ok(Reader {
            bgzf: bgzf,
            header: HeaderView::new(header),
        })
    }
/*
    extern "C" fn pileup_read(
        data: *mut ::std::os::raw::c_void,
        record: *mut htslib::bam1_t,
    ) -> i32 {
        let _self = unsafe { &*(data as *mut Self) };
        unsafe { htslib::sam_read1(_self.bgzf, record) }
    }
*/
    /// Iterator over the records between the (optional) virtual offsets `start` and `end`
    ///
    /// # Arguments
    ///
    /// * `start` - Optional starting virtual offset to seek to. Throws an error if it is not
    /// a valid virtual offset.
    ///
    /// * `end` - Read until the virtual offset is less than `end`
    pub fn iter_chunk(&mut self, start: Option<i64>, end: Option<i64>) -> ChunkIterator<Self> {
        if let Some(pos) = start {
            self.seek(pos)
                .expect("Failed to seek to the starting position");
        };

        ChunkIterator {
            reader: self,
            end: end,
        }
    }
}


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

    fn pileup(&mut self) -> pileup::Pileups<Self> {
        let _self = self as *const Self;
        let itr = unsafe {
            htslib::bam_plp_init(
                Some(Reader::pileup_read),
                _self as *mut ::std::os::raw::c_void,
            )
        };
        pileup::Pileups::new(self, itr)
    }

    fn bgzf(&self) -> *mut htslib::samFile {
        self.bgzf
    }

    fn header(&self) -> &HeaderView {
        &self.header
    }
}








impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            htslib::hts_close(self.bgzf);
        }
    }
}

pub fn print(){
    println!("{}" , 8);
}