ASCII Painter
=============

It's work-in-progress. Inspired by asciiflow.com, this tool converts call graph:

Check out [todo](todo.md) for implementation progress and plan.

```
ClassA::func_1
  ClassB::func_2
    ClassB::func_3
    ClassB::func_4
  ClassB::func_2

ClassC
  ClassB::func_3
```

to UML sequence:

```
    ┌──────┐    ┌──────┐    ┌──────┐
    │ClassA│    │ClassB│    │ClassC│
    └──────┘    └──────┘    └──────┘
 func  │           │           │
 _1    │           │           │
──────►│           │           │
       │           │           │
       │ func_2    │           │
       │──────────►│           │
       │           │           │
       │           │────┐      │
       │           │    │      │
       │           │ func_3    │
       │           │    │      │
       │           │◄───┘      │
       │           │           │
       │           │────┐      │
       │           │    │      │
       │           │ func_4    │
       │           │    │      │
       │           │◄───┘      │
       │           │           │
       │ func_2    │           │
       │──────────►│           │
       │           │           │
       │           │  func_3   │
       │           │◄──────────│
       │           │           │
```

## Usage

```bash
cat ./painter/test/callgraph.txt | cargo run
cargo build --release
cat ./painter/test/callgraph.txt | ./target/release/ascii_painter
```
