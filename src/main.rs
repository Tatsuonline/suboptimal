extern crate lapp;
extern crate md5;
extern crate reqwest;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::SeekFrom;

fn main() {

    let args = lapp::parse_args("
Downloads the subtitles for supplied video.
  -h, --help
  -v, --verbose
  -l, --language (default en)
  <file> (string) input video file name
	");

    let help = args.get_bool("help");
    let verbose = args.get_bool("verbose");
    let language = args.get_string("language");
    let file = args.get_string("file");
    
    let hash = hash_brown(file); // Get the hash of the video.
    check_subdb(hash);
}

fn hash_brown(file: String) -> md5::Digest {

    let metadata = fs::metadata(&file).unwrap();
    let mut video_file = File::open(file).unwrap();
    let mut first_buffer = vec![0; 65536]; // 64 x 1024 for 64kb.
    let mut second_buffer = vec![0; 65536]; // 64 x 1024 for 64kb.
    
    video_file.read(&mut first_buffer).unwrap();
    video_file.seek(SeekFrom::Start(metadata.len() - 65536)).unwrap(); // Sets to the last 64kb.
    video_file.read(&mut second_buffer).unwrap();

    first_buffer.extend(&mut second_buffer.iter().cloned()); // Not very efficient.
    
    let digest = md5::compute(&mut first_buffer);

    println!("Computed hash: {:x}", digest);

    digest
}

fn check_subdb(hash: md5::Digest) -> () { 

    let hash_string = format!("{:x}", hash); // String coversion.
    //let uri_builder = format!("http://sandbox.thesubdb.com/?action=download&hash={}&language=pt,en", hash_string); // For testing.
    let uri_builder = format!("http://api.thesubdb.com/?action=download&hash={}&language=pt,en", hash_string);
    println!("URI: {}", uri_builder);

    send_request(uri_builder);
}   
    
fn send_request(uri: String) -> io::Result<()> {

    let client = reqwest::Client::new();
    let res = client.get(&uri)
	.header("User-Agent", "SubDB/1.0 (suboptimal/0.1; https://github.com/Tatsuonline/suboptimal.git)")
	.send();

    if res.status().is_success() {
	println!("Success!");
    } else if res.status().is_server_error() {
	println!("Server error!");
    } else {
	println!("Something else happened. Status: {:?}", res.status());
}

    Ok(())
}
