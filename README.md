# kvs
A simple key/value store for practicing rust-lang.

## How to use

1. compile
```
cargo build
cd target/release/
```

2. run the server and the client

```
$ ./kvs-server --help

kvs-server 0.1.0
TangliziGit <tanglizimail@foxmail.com>
the server for the key value store

USAGE:
    kvs-server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Print the version

OPTIONS:
    -e, --engine <ENGINE-NAME>    the key-value store engine name [default: kvs]  [possible values: kvs, sled]
    -a, --addr <IP-PORT>          a v4 or v6 IP address with a port number [default: 127.0.0.1:4000]
```

```
$ ./kvs-client

kvs-client 0.1.0
TangliziGit <tanglizimail@foxmail.com>
a client for the key value store

USAGE:
    kvs-client [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Print the version

SUBCOMMANDS:
    get    Get the string value of a given string key
    rm     Remove a given key
    set    Set the value of a string key to a string
```

## Feature

1. friendly CLI 
2. error handling with `Result`
3. log-structured k/v store  
    logging with compaction
4. simple & readable protocol  
    like redis `RESP`
5. shared store engine for multi-threads  
    unique shared writer and cloneable reader, based on reference counting and locks.
    next step is to use wait-free data structures.

## Benchmark

This benchmark compares it with [sled](https://github.com/spacejam/sled) (A modern embedded database).  
`kvs` engine has lower spaces and faster speed, because it has very basic function like setting, getting, removing k/v pairs and log compaction.  
The benchmark result is showed below.  

![full_operation_benchmark](https://raw.githubusercontent.com/TangliziGit/kvs/master/doc/full_operation_benchmark.svg)
 
For example, when set, get and remove 2^16 k/v pairs, `kvs` uses 350ms and 4.05MB while `sled` uses 2176ms and 7.45MB.

![full_density_avg](https://raw.githubusercontent.com/TangliziGit/kvs/master/doc/full_density_avg.png)

## Reference

- [Write a Good CLI Program](https://qiita.com/tigercosmos/items/678f39b1209e60843cc3)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [rust-lang-nursery](https://github.com/rust-lang-nursery)
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)
- [The Design and Implementation of a Log-Structured File System](https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf)
- [Bitcask: A Log-Structured Hash Table for Fast Key/Value Data](https://github.com/basho/bitcask/blob/develop/doc/bitcask-intro.pdf)
- [Redis Protocol specification](https://redis.io/topics/protocol): the redis client-server communication protocol
- [Statistically Rigorous Java Performance Evaluation](https://dri.es/files/oopsla07-georges.pdf): a good example of the kind of thinking necessary to create effective benchmarks
- [Rust: A unique perspective](https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html)
- [Lock-free vs. wait-free concurrency](https://rethinkdb.com/blog/lock-free-vs-wait-free-concurrency/)
- [Lock-Free and Wait-Free, definition and examples ](http://concurrencyfreaks.blogspot.com/2013/05/lock-free-and-wait-free-definition-and.html) [[Chinese](http://ifeve.com/lock-free-and-wait-free/)]