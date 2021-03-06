extern crate md5;
extern crate reqwest;
extern crate walkdir;
use structopt::StructOpt;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io::SeekFrom;
use std::io::Read;
use reqwest::StatusCode;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "suboptimal", about = "Finds and downloads subtitles for videos.")]
struct Opt {

    // Add the verbose flag to print details as the program runs.
    #[structopt(short, long, help = "Prints details as the program runs")]
    verbose: bool,

    // Add the language flag to specify the language of the subtitles to download.
    #[structopt(short, long, default_value = "en", help = "Specifies the language of the subtitles to download")]
    language: String,

    // Add the file name input for which to get subtitles for.
    #[structopt(help = "The path to the file or directory of files for which to get subtitles for")]
    file: String,

}

fn main() {

    let opt = Opt::from_args();
 
    let metadata = fs::metadata(&opt.file).unwrap(); // Pull metadata of the file to determine the size.

    recursive_check(&opt.file, metadata, opt.verbose, &opt.language); // Check recursively through a folder until video files are found.
}

fn verbosity(verbose: bool, information: &str) -> () {
    
    if verbose {
	println!("+ verbose: {}", information);
    }
}

fn database_file_search(file: &String, metadata: std::fs::Metadata, verbose: bool, language: &String) -> () {

    let file_path = Path::new(file);
    let hash = hash_brown(&file, metadata, verbose); // Get the hash of the video.
    check_subdb(hash, &file_path, language, verbose); // Check if the subtitles exist on SubDB.

}

fn recursive_check(file: &String, metadata: std::fs::Metadata, verbose: bool, language: &String) -> () {

    // In case the argument is a file.
    if metadata.is_file() {
	database_file_search(file, metadata, verbose, language);
    } else if metadata.is_dir() { 

	// If the argument is a directory, recursively go through it and process only files.
	for entry in WalkDir::new(file) {
	    
	    let entry = entry.unwrap();
	    let entry_path = &entry.path().display().to_string();
	    let new_metadata = fs::metadata(&entry.path().display().to_string()).unwrap();

	    if new_metadata.is_file() {
		database_file_search(entry_path, new_metadata, verbose, language);
	    }	
	}
    }

}

fn hash_brown(file: &String, metadata: std::fs::Metadata, verbose: bool) -> md5::Digest {

    verbosity(verbose, "Opening the video file.");
    let mut video_file = File::open(file).unwrap();
    // Here we create two buffers to hold 64kb of data from the start and end of the video:
    verbosity(verbose, "Creating the 64kb buffers of the video.");
    let mut first_buffer = vec![0; 65536]; // 64 x 1024 for 64kb.
    let mut second_buffer = vec![0; 65536];
    
    video_file.read(&mut first_buffer).unwrap();
    video_file.seek(SeekFrom::Start(metadata.len() - 65536)).unwrap(); // Sets the buffer to pull from the last 64kb.
    video_file.read(&mut second_buffer).unwrap();

    verbosity(verbose, "Combining the buffers.");
    first_buffer.extend(&mut second_buffer.iter().cloned()); // Combining the two buffers. This process is not very efficient and can be done better.
    verbosity(verbose, "Computing the hash.");
    let digest = md5::compute(&mut first_buffer); // Here we actually compute the hash.

    digest
}

#[tokio::main]
async fn check_subdb(hash: md5::Digest, file_path: &std::path::Path, language: &String, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {

    let hash_string = format!("{:x}", hash); // String coversion of the hash.
    let uri = format!("http://api.thesubdb.com/?action=download&hash={}&language={},en", hash_string, language);

    verbosity(verbose, "Building the request.");
    let client = reqwest::Client::builder()
	.user_agent("SubDB/1.0 (suboptimal/0.1; https://github.com/Tatsuonline/suboptimal.git)")
	.build()?;

    verbosity(verbose, "Sending the GET request.");
    let mut res = client.get(&uri).send().await?; // The GET request is sent.

    match res.status() {
	StatusCode::OK => {
	    verbosity(verbose, "Return code of 200 was received (subtitles exist).");
	    // A return code of 200 was received and indicates that the subtitles exist in the database.

	    // Here we create the subtitles file path that we will write the data to.
	    // The name should be the same as the video file and in the same location.
	    verbosity(verbose, "Setting up subtitle file path.");
	    let subtitles_file_name = file_path.file_stem().unwrap();
	    let mut subtitles_file_format = PathBuf::from(subtitles_file_name);
	    subtitles_file_format.set_extension("srt");
	    let subtitles_file = file_path.parent().unwrap().join(subtitles_file_format);

	    verbosity(verbose, "Creating the subtitles file.");
	    let mut file = File::create(&subtitles_file).unwrap();
	    println!("Downloading to {:#?}...", subtitles_file);

	    verbosity(verbose, "Writing the data to the subtitles file.");
	    // Now we download the data in chunks and write it to the file.
	    while let Some(chunk) = res.chunk().await.unwrap() {
	    	file.write_all(&chunk);
	    }

	    println!("Download complete.");
	},
	StatusCode::NOT_FOUND => println!("A return code of 404 was received. This indicates that the subtitles unfortunately don't exist in the SubDB database."),
	StatusCode::BAD_REQUEST => println!("Error: A return code of 400 was received. This indicates that a bad request was made."),
	_ => println!("Error: A failing return code was received."),
    }

    Ok(())
}
