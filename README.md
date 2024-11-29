# cuffney cli

> CLI tool for interacting with the cuffney platform

![primary](https://github.com/jcuffney/cli/actions/workflows/primary.yml/badge.svg)

## Usage

`cargo run -- -h`

## About

This project is a CLI tool for interacting with the cuffney platform. It is written in Rust and uses the `clap` library for parsing command line arguments.

## Creating a release

Releases are triggered when a new tag is pushed. To create a new release, run the following commands:

```bash
git tag -a vx.x.x -m "<message>"
git push origin vx.x.x
```