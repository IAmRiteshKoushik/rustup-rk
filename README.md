# rustup-rk

## Topic 1 : Stack and Heap
### Stack
- Stack and hepa are parts of memory available to your code to use at runtime
but thheya re structued in different ways. The wstakcs tores values in the 
order it gets them and removes the valeus in the opposite order. This is LIFO.
- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change must 
be stored on the heap instead.
### Heap
- Less organized, you request a certain amount of space. Memory allocator finds
an empty spot in the heap that is big enough and marks it as being in use and 
returns a pointer which is the address of that location. This is called 
`allocating on the heap` or simply `allocating.`
- Acessing data in the heap is slower that accessing data on the stack because 
you have to follow a pointer to get there. Contemporary processors are faster if
jump around less in memory.

---
Pushing to the stack is faster thatn allocating on the heap because the 
allocator never has to search for a place to store new data; that location 
is alays at the top of the stack. Comparitively, allocating space on the heap
requires more work because the allocator must first find a big enough space 
and then perform bookkeeping to prepare for the next allocation.

`PS: Ownership - Manages heap data`
- Keeping track of what  parts of code are using what data on the heap
- Minimizing duplicate data on the heap
- Cleaning up unused data on the heap so you do not run out of space

## Topic 2 : Ownership Rules
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will get dropped


