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


/*   --------------------  WARNING ----------------------  

                      WORK IN PROGRESS...

     ---------------------------------------------------- */



extern crate clap;
extern crate bio;
extern crate rust_htslib;
extern crate regex;

use std::io::*;



mod ui;
mod util;

use util::samXbam::*;


use ui::cli::*;


fn main()-> Result<()>{    // add security wrappers


    let options = parse_cli();
    
    match options.value_of("direction").unwrap() {
        "s2b" => {sam2bam(options.value_of("input").unwrap(), 
                          options.value_of("output").unwrap_or("stdout"))}
        "b2s" => {bam2sam(options.value_of("input").unwrap(), 
                          options.value_of("output").unwrap_or("stdout"))}
        _ => {println!("direction not set properly");Ok(())}
    };


    Ok(())
    // check return

}
