# zoo-debugger
A simple debugger at unix by Rust.

## Usage
Clone this repository:
```
$ git clone https://github.com/jinnzoo/zoo-debugger.git
```
Build it in your crate root:
```
$ cd zoo-debugger
$ cargo build
```
Debug new process:
```
$ cd target/debug/
$ ./zoo-debugger --new <program path>
```
Or, debug existing process:
```
$ ./zoo-debugger --process <pid>
```
