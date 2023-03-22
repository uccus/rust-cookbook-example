use std::fs::File;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;

pub fn test(){
    let path = "./test.tar.gz";
    let tar_gz = File::create(path).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("src", "./src").unwrap();
    
    let tar_gz = File::open(path).unwrap();
    let dec = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(dec);
    archive.unpack("test").unwrap();
}