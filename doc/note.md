Development Note
================

A good resource: https://www.fuzzingbook.org/

## Milestone
- [x] provide input and output.
- [x] Single thread code runner
- [x] Fault detection.
- [x] fuzzle input
- [x] put it together Prove it works fine
- [x] Multi thread

## What should the interface look like?

```bash
./cfm -i cfm_in/ -o cfm_out/ ./flaw
```

Directory tree:
- cfm
- cfm_input/
    - buggy_input
    - good_input
- cfm_output/
- flaw

## 3 Problems
- Case generation
- Effective test case running
- Error tracing
    - call stack backtrace
    - faulting address
    - https://afl-1.readthedocs.io/en/latest/about_afl.html#how-afl-works
    
My only error tracing choice: libgdb, written in c.

I should write a standalone c program and then try integrating it with rust - call binary or integrate it.

Error tracer interface:
```bash
$ tracer {binary} {core file}
{faulting address}

{backtrace...}
{backtrace...}
{backtrace...}
{backtrace...}
```

Another choice: gdb/mi?: https://sourceware.org/gdb/onlinedocs/gdb/GDB_002fMI.html

GDB C API -- does such a thing exist?: https://gdb.sourceware.narkive.com/y6F9LdEM/gdb-c-api-does-such-a-thing-exist

Someone does it. https://github.com/lipk/rust-gdb/tree/master/src
