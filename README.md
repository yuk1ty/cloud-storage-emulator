# cloud-storage-emulator

⚠️ This project is still in progress.

cloud-storage-emulator is an emulator that runs in a single container on your laptop or in your CI environment. The emulator can run in two modes: in-memory (for unit tests) and disk (durable, persistence but not implemented for now).

## Install

The quickest way to use cloud-storage-emulator is by using Docker for now.

```
$ docker build . -t cloud-storage-emulator:latest
$ docker run -p 8000:8000 cloud-storage-emulator:latest
```

## Features

### Modes

- [x] In-memory mode (in progress)
- [ ] Disk mode

### Buckets Related

- [x] List buckets
- [ ] Get bucket
- [x] Create new bucket
- [ ] Update bucket
- [ ] Delete bucket

### Objects Related

TBD

## Why using Rust?

Rust is the most fluent programming language for me and just for fun!
