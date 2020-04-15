extern crate lapp;
extern crate md5;
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
    
    hash_brown(file); // Get the hash of the video.
    
}

fn hash_brown(file: String) -> io::Result<()> {

    let metadata = fs::metadata(&file)?;
    let mut video_file = File::open(file)?;
    let mut first_buffer = vec![0; 65536]; // 64 x 1024 for 64kb.
    let mut second_buffer = vec![0; 65536]; // 64 x 1024 for 64kb.
    
    video_file.read(&mut first_buffer)?;
    video_file.seek(SeekFrom::Start(metadata.len() - 65536))?; // Sets to the last 64kb.
    video_file.read(&mut second_buffer)?;

    first_buffer.extend(&mut second_buffer.iter().cloned()); // Not very efficient.
    
    let digest = md5::compute(&mut first_buffer);

    println!("Computed hash: {:x}", digest);

    Ok(())

}

//fn check_subdb(hash: ) -> () { 

   //let user_agent = "SubDB/1.0 (suboptimal/0.1; https://github.com/Tatsuonline/suboptimal.git)"

//}
