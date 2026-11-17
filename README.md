# Rust Ownership vs. Java Garbage Collection (COMP 211)
I explored how Rust manages memory without a  garbage collector and how its approach compares with Java, C, as well as the topics we have learned about in class. I learned in class that in C, programmers are responsible for freeing dynamically allocated memory, as we learned about malloc and free. I wanted to understand next how Rust handles cleanup without requiring the programmer to call free, and how that also differs from garbage collection in Java.

The main topics that I learned about were:
- Ownership, scope, and automatic cleanup in Rust
- Moving a String and borrowing it without transferring ownership
- Java references, reachability, and eligibility for garbage collection
- Trade-offs between compile-time restrictions and runtime garbage collection

Next I will go over some of my main discoveries throughout this learning experience. I learned that Rust checks ownership rules at compile time, and that assigning a String to another variable moves ownership. The original variable can then no longer access that original value, as it becomes invalid. It is important to note that moving ownership does not clone a String's heap contents. Borrowing is a different functionality that lets code access a value without becoming its owner. This is similar to passing by pointer in C. On the topic of cleanup, it still happens at runtime. For the ordinary owned String examples here, leaving the owner's scope drops the value and releases its heap buffer. Rust does not free every allocation immediately after its last use. Ownership checking happens during compilation, while allocation and cleanup happen during the time a program runs. Java is different from Rust in that it tracks reachability instead of a single owner. There can be multiple references that point to the same object, and removing one reference does not make the object into garbage if another reachable reference still leads to it. An object that is no longer reachable from GC roots can be collected by the garbage collector, but that does not guarantee that it is immediately reclaimed.

## Comparison:  

|     | C | Rust | Java | 
| --- | --- | --- | --- |
| Who handles heap cleanup? | Programmer calls free | Owner's drop behavior releases its buffer |	Runtime garbage collector |
| When is cleanup done? | When free is called | Usually when the owner leaves scope | When the collector reclaims an unreachable object | 
| What is the main responsibility? | Manage allocation and deallocation correctly | Follow ownership and borrowing rules | Avoid retaining references to unneeded objects
| What trade-off is made? | Direct control with risk of memory errors | Predictable drop points with restrictions on sharing | Flexible sharing with runtime collection costs 

### Overall Comparison
Neither approach is always faster. The speed of performance depends on the workload as well as its implementation. Rust can still leak memory. An example of this is through reference-counted ownership cycles. Java can retain unwanted objects if they remain reachable, which utilizes a lot of memeory. Releasing an allocation also does not mean physical memory is returned to the operating system right away.

## Code Examples
### 1.
In this first example, I wanted to test what I learned by predicting which lines would result in a compiler error.

```
fn show(text: &String) {
    println!("{}", text);
}

fn main() {
    let first = String::from("coral");

    show(&first);                    // A
    println!("{}", first);           // B

    let second = first;              // C

    println!("{}", first);           // D
    show(&second);                   // E

    {
        let third = String::from("reef");
        show(&third);                // F
    }

    println!("{}", third);           // G
}
```
I identified two errors here: attempting to access a value after moving ownership and trying to access a variable outside its scope. I made the table below to explain whether each of the lines are valid or not:

| Line |Error (Y/N) | Explanation |
| --- | --- | --- |
| A | Yes | `show` borrows `first` without taking ownership. |
| B | Yes | `first` still owns the String after the borrow. |
| C | Yes | Ownership moves from `first` to `second`. |
| D | No | `first` can no longer access the moved value. |
| E | Yes | `second` owns the String and can lend a reference. |
| F | Yes | `third` is valid in the inner scope. |
| G | No | `third` is outside its scope. |

### 2. 

This next example is a similar exercise. 

```
fn borrow_text(text: &String) {
    println!("Borrowed: {}", text);
}

fn take_text(text: String) {
    println!("Taken: {}", text);
} // The owned String is dropped here.

fn main() {
    let ocean = String::from("ocean");

    borrow_text(&ocean);              // A
    println!("{}", ocean);           // B

    let reef = ocean;                // C
    borrow_text(&reef);              // D

    println!("{}", ocean);           // E

    take_text(reef);                  // F
    println!("{}", reef);            // G
}
```

| Line | Error(Y\N) | Explanation |
| --- | --- | --- |
| A | Yes | `borrow_text` borrows `ocean` without taking ownership. |
| B | Yes | The borrow has ended, and `ocean` still owns the String. |
| C | Yes | Ownership moves from `ocean` to `reef`. |
| D | Yes | `borrow_text` borrows `reef` while ownership stays with `reef`. |
| E | No | `ocean` can no longer access the String after ownership moved at C. |
| F | Yes | Ownership moves from `reef` into `take_text` |
| G | No | `reef` can't access the String after ownership was moved at F. |


### 3.

This final example helped me see how dynamically allocated data is stored differently by C, Rust, and Java.   

In C, the programmer determines cleanup:
```
#include <stdlib.h>

int main(void) {
    int *first = malloc(sizeof(int));
    if (first == NULL) return 1;

    *first = 10;
    int *second = first; // Both point to the same allocation.

    free(first);        // Frees the allocation.

    // *second would now access freed memory!
    // free(second) would be a double free.
}
```
In Rust, ownership determines clean-up:
```
fn main() {
    let first = Box::new(10);
    let second = first; // Ownership moves to second.

    // println!("{}", first); // Compile error: first was moved.
    println!("{}", second);
} // second is dropped; the heap allocation is freed.
```
In Java, if an object cannot be reached anymore it is eligible for collection: 
```
class Data {
    int value = 10;
}

public class Example {
    public static void main(String[] args) {
        Data first = new Data();
        Data second = first; // Both reference the same object.

        first = null;
        System.out.println(second.value); // Still accessible.

        second = null; // No references remain in this example.
    }
}
```

### Running the exercises

Install Rust, a C compiler, and a JDK.  

**Exercises 1 and 2** 
```
rustc exercise1.rs
rustc exercise2.rs
```

These examples will first fail because both have compiler errors.  

Comment out the invalid lines noted above, then compile and run again:  

```sh
rustc ownership_scope.rs -o ownership_scope
./ownership_scope

rustc borrow_take.rs -o borrow_take
./borrow_take
```

**Exercise 3**  
C:

```
cc manual_memory.c -o manual_memory
./manual_memory
```
Rust:

```
rustc box_cleanup.rs -o box_cleanup
./box_cleanup
```

Java:

```
javac Example.java
java Example
```


## Written Reflection
I approached learning about this topic by using AI as a tutor and teacher. Initially, I started with what I already knew from COMP 211 and wanted to see how I could build off of my knowledge of C as I started learning about ownership in Rust. I don't have any prior knowledge on Rust, so I had a few questions already based on the topic that I knew I wanted to address. My exploration then developed more in the direction of memory management as I continued to ask more questions. One prompt that I used was what I started with: "I know of C pointers, malloc, free from COMP 211, but I haven't used Rust. What problem does Rust's ownership system solve?" I thought that this was a good way of very quickly explaining what I already know and starting with the most basic question on the topic so that I may learn about what ownership in Rust actually is. I then asked a follow-up question of "So when Rust prevents us from using p after ownership can been transferred, does that make p null?" because it was only said that p is prevented from being used, and I was curious as to what particularly happens to it. I engaged with AI in many ways. First I asked a broad question, and then continued the conversation by asking follow-up questions to clarify parts of its answers. I also asked for examples that I could work through, send in an answer, and have it checked. This allowed me to test what I learned and ask more questions. I think getting exercises to do worked well in testing what I learned, as well as trying to explain what I grasped in my own words. I realized that my wording was off and I was able to correct what I had thought was right. In the future, I will try to do that more often, so that I can also challenge the AI tutor and ensure that the information I am receiving is valid.

[AI conversation permalink](https://chatgpt.com/share/6afa6f6a-4dfc-83e9-97bc-f98c465d1604)  
[Slideshow](https://canva.link/j6s1xz3wxt4fwn9)   
