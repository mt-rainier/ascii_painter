ASCII Painter
=============

It's work-in-progress. Inspired by asciiflow.com, this tool converts call graph:

```
ClassA::func_1
  ClassB::func_2
    ClassD::func_4
  ClassC::func_3
```

to UML sequence:

```
    ┌──────┐    ┌──────┐    ┌──────┐    ┌──────┐
    │ClassA│    │ClassB│    │ClassD│    │ClassC│
    └──────┘    └──────┘    └──────┘    └──────┘
 func  │           │           │           │
 _1    │           │           │           │
──────►│           │           │           │
       │           │           │           │
       │ func_2    │           │           │
       │──────────►│           │           │
       │           │           │           │
       │           │ func_4    │           │
       │           │──────────►│           │
       │           │           │           │
       │ func_3    │           │           │
       │──────────────────────────────────►│
       │           │           │           │
```

## Usage

```bash
cat ./painter/test/callgraph.txt | cargo run
cargo build --release
cat ./painter/test/callgraph.txt | ./target/release/ascii_painter
```
