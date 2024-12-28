# Unsafe Rust: Modifying Vector via Raw Pointer
This repository demonstrates a potential pitfall when using raw pointers in Rust to modify a vector's contents. Directly manipulating memory using raw pointers can lead to undefined behavior, especially when the vector's internal structure is changed.
The `bug.rs` file showcases the problem.  Modifying the vector through its raw pointer can corrupt the vector's internal data structures, which can result in unexpected results or runtime panics. The solution (`bugSolution.rs`) illustrates how to avoid such issues.