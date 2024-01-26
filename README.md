# Rust request searcher

Rust request searcher it's a program which can sort your file or database by lines you need (By keyword / regular expression in line). 
It can sort 15GB file in 22 seconds (Tested on Macbook Air M1 2020).

# Building with cargo:

`cargo build --release` in project folder.
Then you need file in rust-request-searcher/target/release/request_searcher.exe or your OS file descriptor.

# How to use:

Rust request searcher uses several arguments:

```
-F (-F1, -F2 ... -F5): your one-word request. Up to five, if it's only, then use -F.
-R (-R1, -R2 ... -R5): your regular expression request.
-P: path to file which needs to be sorted.
-C: sort lines from type url:login:pass to login:pass.
-r: Use register accounting.
```

New files will be located in the ./result folder.
