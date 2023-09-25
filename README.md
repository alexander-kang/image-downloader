## What is this

An image downloading tool written in Rust that's pretty fast when it works properly.

It sends GET requests to URLs the user specifies and downloads images from there.
  
The reliability and usability is a WIP but the speed is pretty good already.

### Prerequisites

Verify that [Rust](https://www.rust-lang.org/tools/install) is installed
```sh
rustc --version
```

### Installation

1. Clone the repo & navigate to the directory
   ```sh
   git clone https://github.com/alexander-kang/image-downloader.git
   cd image-downloader/
   ```
2. Install dependencies
   ```sh
   cargo build
   ```

## Usage

Make a file in `image-downloader/` called `urls.txt` and put URLs you'd like to download from on each new line. Save this and then run the code.
```sh
cargo run --release
```
The downloaded files will be in numbered directories within `out/`.

## Work in progress

Reliability:
- Stop panicking anytime an error occurs
- If operation fails, keep trying up to N times  


Usability:
- Simplify populating `urls.txt` (e.g., Python script auto-populates it for user)
- Let user specify name for directories in `out/`
- Report which operations failed (after N attempts)
- Progress updates
- Profile code to make smaller and faster
