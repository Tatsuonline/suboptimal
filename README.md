
# Suboptimal: The Optimal Subtitle Finder

1.  [Installation](#org2f72563)
2.  [Documentation](#orga11e821)

---

Suboptimal is a minimalistic command line interface (CLI) subtitle finder for any video, movie or TV show you have! If the subtitles exist on a publically available database online, suboptimal will find and download it for you.

Currently, suboptimal uses the SubDB API to pull subtitles, with more APIs to added soon.


<a id="org2f72563"></a>

# Installation

Clone this repository:

    git clone https://github.com/Tatsuonline/suboptimal.git

Build the release:

    cargo build --release

Run suboptimal:

    cd target/release
    ./suboptimal <file> <flags>


<a id="orga11e821"></a>

# Documentation

The following flags can be passed to suboptimal.

Downloads the subtitles for supplied video.
  -h, &#x2013;help
  -v, &#x2013;verbose
  -l, &#x2013;language (default en)
  <file> (string) input video file name

