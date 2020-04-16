
# Suboptimal: The Optimal Subtitle Finder

1.  [Installation](#orgbacbada)
2.  [Documentation](#org29f2f62)

---

Suboptimal is a minimalistic command line interface (CLI) subtitle finder written in Rust. It works for any video, movie or TV show you have! If the subtitles exist on a publically available database online, suboptimal will find and download it for you.

Currently, suboptimal uses the SubDB API to pull subtitles, with more APIs to added soon.


<a id="orgbacbada"></a>

# Installation

Clone this repository:

    git clone https://github.com/Tatsuonline/suboptimal.git

Build the release:

    cargo build --release

Run suboptimal:

    cd target/release
    ./suboptimal <file> <flags>


<a id="org29f2f62"></a>

# Documentation

The following flags can be passed to suboptimal.

Downloads the subtitles for supplied video.
  -h, --help
  -v, --verbose
  -l, --language (default en)
  <file> (string) input video file name

