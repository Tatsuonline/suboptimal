extern crate lapp;
extern crate md5;
extern crate reqwest;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::SeekFrom;
use std::io::Read;
use reqwest::StatusCode;
use std::collections::HashMap;

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

    digest
}

#[tokio::main]
async fn check_subdb(hash: md5::Digest) -> Result<(), Box<dyn std::error::Error>> {

    let hash_string = format!("{:x}", hash); // String coversion.
    let uri = format!("http://api.thesubdb.com/?action=download&hash={}&language=pt,en", hash_string);
    
    let client = reqwest::Client::builder()
	.user_agent("SubDB/1.0 (suboptimal/0.1; https://github.com/Tatsuonline/suboptimal.git)")
	.build()?;
    
    let res = client.get(&uri).send().await?;

    println!("\n\n");
    
    match res.status() {
	StatusCode::OK => {
	    println!("200: The subtitles exist!");

	    match &res.headers().get("content-disposition") {
		Some(srt_file) => {
		    println!("\nFile: {:#?}\n", srt_file);

		    // TODO: Download the subtitles.
		},
		_ => println!("Somehow, you managed to screw this up."),
	    };
	},
	StatusCode::NOT_FOUND => println!("404: The subtitles unfortunately don't exist."),
	StatusCode::BAD_REQUEST => println!("400: Ya done goofed!"),
	_ => println!("Something else is messed up."),
    }

    println!("\nFull response: {:#?}\n", res);

    Ok(())
}
