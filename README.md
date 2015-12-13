# link-checker
A simple link checking tool written in Rust 1.3

[![Build Status](https://travis-ci.org/antonyh/link-checker.svg)](https://travis-ci.org/antonyh/link-checker)

## Building the link-checker

Use 'cargo build'.

## Running the link-checker

User 'cargo run input.txt'

## Creating your own input file

You can name it anything you want, and all it really needs is to be a list of URLs, one per line. 

## Known limitations

It depends on 'hyper' v0.6.15 for making HTTP calls, so whatever it can do this tool should also do. I've not yet tried it with HTTPS or with querystrings. All requests are GET, so no POST, HEAD, or other methods. 

