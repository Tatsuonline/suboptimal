extern crate lapp;
extern crate md5;
extern crate reqwest;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::SeekFrom;
use std::io::Read;
use reqwest::StatusCode;
use std::path::{Path, PathBuf};

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

    let file_path = Path::new(&file);
    let metadata = fs::metadata(&file).unwrap();
    
    let hash = hash_brown(&file, metadata); // Get the hash of the video.
    check_subdb(hash, &file_path, language);
}

fn hash_brown(file: &String, metadata: std::fs::Metadata) -> md5::Digest {

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
async fn check_subdb(hash: md5::Digest, file_path: &std::path::Path, language: String) -> Result<(), Box<dyn std::error::Error>> {

    let hash_string = format!("{:x}", hash); // String coversion.
    let uri = format!("http://api.thesubdb.com/?action=download&hash={}&language={},en", hash_string, language);
    
    let client = reqwest::Client::builder()
	.user_agent("SubDB/1.0 (suboptimal/0.1; https://github.com/Tatsuonline/suboptimal.git)")
	.build()?;
    
    let mut res = client.get(&uri).send().await?;

    match res.status() {
	StatusCode::OK => {
	    println!("200: The subtitles exist!");

	    let subtitles_file_name = file_path.file_stem().unwrap();
	    let mut subtitles_file_format = PathBuf::from(subtitles_file_name);
	    subtitles_file_format.set_extension("srt");
	    let subtitles_file = file_path.parent().unwrap().join(subtitles_file_format);

	    let mut file = File::create(&subtitles_file).unwrap();
	    println!("Downloading to {:#?}...", subtitles_file);
	    
	    while let Some(chunk) = res.chunk().await.unwrap() {
	    	file.write_all(&chunk);
	    }

	    println!("Download complete.");
	},
	StatusCode::NOT_FOUND => println!("404: The subtitles unfortunately don't exist."),
	StatusCode::BAD_REQUEST => println!("400: Ya done goofed!"),
	_ => println!("Something else is messed up."),
    }

    Ok(())
}
