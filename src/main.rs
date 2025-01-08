/*
    Copyright (c) 2025 MydriaTech AB

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to
    deal in the Software without restriction, including without limitation the
    rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
    sell copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
    IN THE SOFTWARE.
*/

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;

/// Application main entry point
fn main() {
    unpack_xz("app.xz", "app");
    let argv = &std::env::args().collect::<Vec<_>>();
    execcv("app", argv);
}

/// Unpack LMZA2 (XZ) `archive` into `target`
fn unpack_xz(archive: &str, target: &str) {
    let mut input = BufReader::new(File::open(archive).unwrap());
    let mut output = File::create(target).unwrap();
    lzma_rs::xz_decompress(&mut input, &mut output).unwrap();
}

/// Hand over execution from this process to `executable`.
fn execcv(executable: &str, argv: &[String]) {
    let path = &CString::new(executable).unwrap();
    let argv = &argv
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect::<Vec<_>>();
    nix::unistd::execv(path, argv).unwrap();
}
