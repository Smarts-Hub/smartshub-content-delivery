# SmartsHub Content Delivery
A microservice that serves files from a local directory to users throught a HTTP API built on Rust.

## To deploy locally
1. Clone the repository
2. Move into the folder
3. Rename `.env.example` to `.env`
4. Run `cargo run`

## How to use
```bash
# Get the content of a file
~$ localhost:3030/file/example.txt
This is an example file

# Get the SHA1 checksum of the file
~$ localhost:3030/file/example.txt/sha1
5a5d714f8cf66a9bc94cc5748302e094abb85fe6

# Get the size in bytes of the file
~$ localhost:3030/file/example.txt/size
23
```