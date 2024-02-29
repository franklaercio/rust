# The Rust Programmer Language

Rust is safe to manager memory, because Rust do the code analysis in runtime (Boghout Check).

The compile reject program when not __convices__ that program isn't correct.

__The problems to manager memory in C are:__

- The memory is alocate in heap and __FIND__
- DOS: Denied of Service, because the unsed memory when you call the same service some times, one time this has enoghout memory
- The programmer have to remember to free memory in C, but some problems maybe __CHANGE__ like use memory in time to free
- Garbaget collector it's one alternative to manager memory, but have some issues:
    - Use more memory to manager automatic
    - Use more processing to manager
    - It's not predicted
    - And has a silent error, when not refresh in large data structure
    - The tail latency it's more in languages that using garbaget collector

