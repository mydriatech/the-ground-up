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
use std::io::Seek;
use std::io::SeekFrom;

/// Application main entry point
fn main() {
    let argv = &std::env::args().collect::<Vec<_>>();
    let executable_name = argv.first().map(String::as_str).unwrap_or("app");
    let archive_name = format!("{executable_name}.xz");
    let target_executable_name = format!("{executable_name}-bin");
    unpack_xz(&archive_name, &target_executable_name);
    execcv(&target_executable_name, argv);
}

/// Unpack LMZA2 (XZ) `archive` into `target`
fn unpack_xz(archive: &str, target: &str) {
    let mut input = BufReader::new(
        File::open(archive)
            .map_err(|e| {
                println!("Unable to open archive '{archive}': {e}");
            })
            .unwrap(),
    );
    let mut output = File::options()
        .append(true)
        .open(target)
        .map_err(|e| {
            println!("Unable to target executable to write to '{target}': {e}");
        })
        .unwrap();
    let output_size = output.seek(SeekFrom::End(0)).unwrap();
    if output_size == 0 {
        // This is still the placeholder -> Uncompress to this file
        lzma_rs::xz_decompress(&mut input, &mut output).unwrap();
    }
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
