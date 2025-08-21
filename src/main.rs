use abootimg_oxide::{BufReader, Header};
use std::fs::File;

fn main() {
    let mut r = BufReader::new(File::open("vendor_boot.img").unwrap());
    let hdr = Header::parse(&mut r).unwrap();
    println!("{hdr:#?}");

    // // Extract the kernel
    // use std::io::{self, BufWriter, Read, Seek, SeekFrom};
    //
    // let mut w = BufWriter::new(File::create("boot_a_kernel").unwrap());
    // let r = r.get_mut();
    // r.seek(SeekFrom::Start(hdr.kernel_position() as u64))
    //     .unwrap();
    // io::copy(&mut r.take(hdr.kernel_size().into()), w.get_mut()).unwrap();
}
