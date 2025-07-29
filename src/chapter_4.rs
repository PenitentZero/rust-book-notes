pub fn ownership() {
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("Memory address of s: {:p}", s.as_ptr());
    let s1 = s;
    println!("Memory address of s1: {:p}", s1.as_ptr());
    let value: String = take_ownership(s1);
    println!("Memory address of value: {:p}", value.as_ptr());
    //s1 is not available anymore
    println!("{}", value);

    let s1 = String::from("hello");
    println!("Memory address of s1: {:p}", &s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut string_2 = String::from("Hello");
    mut_string(&mut string_2);
    println!("string_2 mut_string memory address in out of function: {:p}",string_2.as_ptr());
    println!("mutable string: {}", string_2);

    let test_string: String = String::from("Hello baby!");
    let test_string_len = string_len(&test_string);
    println!("test string length: {}", test_string_len);

    let take_string: String = return_string();
    println!("Memory address of taken_string: {:p}", take_string.as_ptr());
    println!("taken string: {}", take_string);
}

fn calculate_length(string: &String) -> usize {
    println!("Memory address of string: {:p}", string);
    string.len()
}
fn mut_string(string: &mut String) {
    println!("mut_string Function: {:p}", string.as_ptr());
    string.push_str(", World!");
}

fn take_ownership(some_string: String) -> String {
    some_string
}

fn string_len(string: &String) -> usize {
    string.len()
}

fn return_string() -> String {
    let string = String::from("hello, darling");
    println!("Memory address of return_string: {:p}", string.as_ptr());
    string
}
/*
Ownership, Garbage collector, explicit allocate and free memory
Heap and stack.
Stack, push and pop , Fast because of the way it stores the data, stack doesn't have to search for a place to put new data or get data, fixed size
Heap, unknown size or a size that might change get stored in heap, Heap is less organized, when you put some data on the heap you ask for some amount of space
marks it being in use, return a pointer, which is the address of that location
you can store the pointer on the stack, but when you
want the actual data, you have to follow the pointer.
Accessing data in the heap is slower than accessing data on the stack
because you have to follow a pointer to get there.
but knowing that managing heap data is why
ownership exists can help explain why it works the way it does.

Ownership rules:
Each value in Rust has a variable that’s called its owner.
There can be only one owner at a time.
When the owner goes out of scope, the value will be dropped.

New type
The String Type
The types covered
previously are all stored on the stack and popped off the stack when their
scope is over, but we want to look at data that is stored on the heap and
explore how Rust knows when to clean up that data.

We’ll use String as the example here and concentrate on the parts of
String that relate to ownership.
This type is allocated
on the heap and as such is able to store an amount of text that is unknown
to us at compile time. You can create a String from a string literal using the
"from" function, like so:

let s = String::from("Hello, world!");

How to push data:
s.push_str("W");
How to print:
println!("{}", s);

let me tell you the difference here
when you edit String, you actually change the value in the memory
huge but, if you edit &str, the last thing still exist on the memory, just your variable is pointing to a new place in memory!
fuck

The memory must be requested from the operating system at runtime.
We need a way of returning this memory to the operating system when
we’re done with our String.

what does rust do?
once you are out of scope, rust will automatically return the memory and make it free

Ownership and Functions
&mut string_name
ownership and borrowing
mut is for editing!
you can use
*/
