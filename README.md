# POPL-Project-Group-11
## 1. Problem Statement (Original Statement):
Migration of C/C++ Cryptographic Algorithms to Rust, Demonstrating Memory Safety
POPL Angle:
The Principles of Programming Languages (POPL) angle in our project lies in the exploration of memory safety aspects when migrating cryptographic algorithms from C/C++ to Rust. POPL focuses on the design and analysis of programming languages, and in this context, our project aims to demonstrate how Rust, with its emphasis on memory safety, can provide a more secure alternative to traditional languages like C/C++ for implementing cryptographic algorithms.
Previous Solutions:
Cryptographic algorithms have been implemented in various programming languages, including C/C++ for their efficiency. However, these languages are notorious for memory-related vulnerabilities such as buffer overflows and data corruption.
Differences in Our Solution:
Our solution involves migrating the RC4 and RSA cryptographic algorithms from C/C++ to Rust, a language known for its focus on memory safety. By doing so, we aim to address and mitigate common vulnerabilities associated with C/C++ implementations. This migration allows for a comparative analysis of the memory safety aspects between the two languages. It's not just about implementing the algorithms; it's about leveraging Rust's features to enhance the security of cryptographic code.

## 2. Software Architecture Overview:

### Components:
RSA Implementation:
RSA cryptographic algorithm implemented in both Rust and C.
RC4 Implementation:
RC4 cryptographic algorithm implemented in both Rust and C.
Performance Measurement and Analysis:
Using Vtune for analyzing the performance of both C and Rust code.

### Architecture:
Dual Implementation:
Rust and C implementations coexist for both RSA and RC4 cryptographic algorithms.
Rust Implementation:
Leverages Rust's memory safety features.
C Implementation:
Represents the traditional approach.
Performance Analysis Component:
Vtune is utilized for performance analysis, allowing for a detailed examination of both Rust and C implementations to identify bottlenecks and optimize code.

### Testing Component:
Local Testing:
Testing components are primarily placed on the local machine.
Various test cases and inputs are provided to assess the correctness and performance of cryptographic algorithms in both Rust and C.
Vtune is used locally to analyze and profile the performance of the code.

### Reuse vs. Development:
Developed Components:
Both RSA and RC4 implementations in Rust and C are developed specifically for this project.
Performance measurement and analysis methods using Vtune are tailored for this project.

### Reused Components:
No use of external libraries or frameworks or any available implementations of the algorithms.

### Visual Representation:
```
+-----------------------------------------------------+
|                 RSA & RC4 Implementations            |
|    +----------------------+   +------------------+   |
|    |   Rust Implementation|   |   C Implementation|  |
|    |  (Memory Safety)     |   |   (Traditional)   |  |
|    +----------------------+   +------------------+   |
|                                                      |
|             +-------------------------------------+  |
|             | Performance Measurement and Analysis | |
|             |             (Using Vtune)            | |
|             +-------------------------------------+  |
|                                                      |
|                         Local Testing                |
|                                                      |
+-----------------------------------------------------+

```

### Client-Server Architecture:
Not Applicable

### Database:
Not Applicable:

This architecture allows for local testing and analysis of both Rust and C implementations, providing insights into performance differences and memory safety aspects. The use of Vtune aids in in-depth performance analysis.

## 3. POPL Aspects in RC4 Implementation:

### Memory Safety with Indexing:

In the ksa and pgra functions, array indexing is used (s[i] and key[i % key_len]) to access elements safely within bounds.
Avoiding Buffer Overflows:

The use of safe indexing and swapping in the state vector (s.swap(i, j)) helps prevent buffer overflows and ensures memory safety.
Static Typing in Rust:

Types are explicitly declared (s: &mut [u8; 256], key: [u8; 9]), utilizing Rust's static typing for compile-time type checking.
Lifetime Annotations:

Lifetime annotations are not explicitly used in these snippets, but the borrow checker ensures proper referencing and borrowing, contributing to memory safety.
POPL Aspects in RSA Implementation:

### Memory Safety with Vec<u8>:

The use of Vec<u8> for data storage ensures dynamic memory allocation and deallocation, contributing to memory safety.
Error Handling with Result Type:

The expect method is used for error handling after encrypting and decrypting data, ensuring that any errors during these operations are handled explicitly.
Static Typing and Explicit Type Conversion:

Types are explicitly declared (bits: usize, string: String), and there's explicit type conversion (string.as_bytes()) to ensure type safety.
Use of External Libraries (rsa, rand, hex):

External libraries like rsa, rand, and hex are utilized, and their use aligns with memory safety principles as they are expected to be implemented with those considerations.
Difficulties and Experiences:

### Transition from C to Rust:

Moving from the C-style manual memory management to Rust's ownership model might pose challenges. Ensuring lifetimes and borrowing are appropriately managed could be an initial difficulty.
Understanding External Libraries:

Incorporating and understanding external libraries, such as rsa and hex, might require familiarity with their documentation and potentially adjusting code to match their conventions.
Error Handling Paradigm Shift:

Shifting from C-style error handling to Rust's Result type might be challenging initially. Ensuring comprehensive error checking is crucial for robust code.
Ensuring Memory Safety in Cryptographic Operations:

Given the sensitivity of cryptographic operations, thorough testing and validation are essential to ensure memory safety, avoiding potential vulnerabilities.

## 4. Results

To compare performance, reliability, and memory safety, we used various cutting-edge benchmark tools like Vtune, Perf, ZChampSim, etc., and tested on various platforms. Our in-depth analysis shows that Rust programs perform better in terms of CPU parallelization, cache memory safety, and average run time compared to C programs. Here are some snapshots that provide a summary of the tests performed.



Performance Analysis of rsa.c :  
![cimage-1](/images/rsa_c.jpg)

Performance Analysis of rc4.c :  
![cimage-1](/images/rc4_c.jpg)

Performance Analysis of rsa.rs : 
![cimage-1](/images/rsa_rust.jpg)

Performance Analysis of rc4.rs
![cimage-2](/images/rc4_rust.jpg)

. Memory Safety and Bounds Checking:
C Code:

```
int main() {
    int arr[5] = {1, 2, 3, 4, 5};
    int index = 10; // Out of bounds access
    printf("%d\n", arr[index]); // Undefined behavior in C
    return 0;
}
```
Rust Code:

```
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let index = 10; // Out of bounds access is a compile-time error
    println!("{}", arr[index]); // Compiler ensures bounds checking
}
```
Explanation:

In the C example, there's an out-of-bounds access to the array, which leads to undefined behavior.
The Rust example uses a Vec, and the attempt to access an element out of bounds is caught at compile time, preventing runtime errors.
Why Rust is Better:
Rust's ownership model and borrowing system allow it to perform static analysis and catch issues like array bounds violations at compile time, providing stronger guarantees for memory safety.
2. Ownership and Borrowing:
C Code:

```
void modify_array(int *arr, int length) {
    arr[0] = 10;
    // No way to express ownership or borrow semantics in C
}

int main() {
    int arr[5] = {1, 2, 3, 4, 5};
    modify_array(arr, 5);
    printf("%d\n", arr[0]); // Value changed in the function
    return 0;
}
```
Rust Code:

```
fn modify_vector(vec: &mut Vec<i32>) {
    vec[0] = 10;
    // Ownership and borrowing semantics in Rust
}

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    modify_vector(&mut vec);
    println!("{}", vec[0]); // Value changed in the function
}
```
Explanation:

In C, there's no clear way to express ownership or borrowing semantics, making it challenging to manage mutable references safely.
Rust's borrowing system allows the creation of mutable references, and the compiler enforces rules to prevent data races and ensure memory safety.
Why Rust is Better:
Rust's ownership model helps prevent common programming errors related to mutable access and provides a clear and safe way to handle mutable data.
3. String Ownership and Lifetimes:
C Code:

```
char* create_string() {
    char str[] = "Hello, C!";
    return str; // Returning a pointer to a local variable is unsafe
}

int main() {
    char* c_str = create_string();
    printf("%s\n", c_str); // Undefined behavior in C
    return 0;
}
```
Rust Code:

```
fn create_string() -> &'static str {
    let s: &'static str = "Hello, Rust!";
    s // No ownership issues, string has a static lifetime
}

fn main() {
    let rust_str = create_string();
    println!("{}", rust_str);
}
```
Explanation:

The C code returns a pointer to a local array, leading to undefined behavior when accessed outside the function.
Rust uses static lifetimes to indicate that the string has a lifetime that lasts for the entire program, preventing issues related to returning references to local variables.
Why Rust is Better:
Rust's ownership and lifetime system prevents dangling references and ensures that references are valid for their intended duration, enhancing memory safety.
4. Option and Result Types:
C Code:

```
int divide(int a, int b, int* result) {
    if (b == 0) {
        return -1; // Error code for division by zero
    }
    *result = a / b;
    return 0;
}

int main() {
    int result;
    if (divide(10, 0, &result) == 0) {
        printf("Result: %d\n", result);
    } else {
        printf("Error: Division by zero\n");
    }
    return 0;
}
```
Rust Code:

```
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None // Representing division by zero with Option
    } else {
        Some(a / b)
    }
}

fn main() {
    if let Some(result) = divide(10, 0) {
        println!("Result: {}", result);
    } else {
        println!("Error: Division by zero");
    }
}
```
Explanation:

The C code returns an error code for division by zero, and the caller needs to check the return value for errors.
Rust uses the Option type to explicitly handle the possibility of division by zero, providing a cleaner and safer way to represent errors.
Why Rust is Better:
Rust's use of algebraic data types like Option and Result allows for more expressive error handling, reducing the likelihood of unchecked errors.
5. Safe Concurrency with Ownership:
C Code:

```
#include <stdio.h>
#include <pthread.h>

int global_var = 0;

void* increment(void* arg) {
    int* local_var = (int*)arg;
    for (int i = 0; i < 1000000; i++) {
        (*local_var)++;
    }
    return NULL;
}

int main() {
    pthread_t thread1, thread2;
    pthread_create(&thread1, NULL, increment, (void*)&global_var);
    pthread_create(&thread2, NULL, increment, (void*)&global_var);
    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);
    printf("Global Variable: %d\n", global_var); // Unsafe concurrent access
    return 0;
}
```
Rust Code:

```
use std::thread;

fn increment(local_var: &mut i32) {
    for _ in 0..1000000 {
        *local_var += 1;
    }
}

fn main() {
    let mut global_var = 0;
    let mut handles = vec![];

    for _ in 0..2 {
        let handle = thread::spawn(move || {
            increment(&mut global_var);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Global Variable: {}", global_var); // Safe concurrent access
}
```
Explanation:

The C code uses threads to increment a global variable without any synchronization mechanisms, leading to data races.
Rust ensures safe concurrency by using ownership and borrowing to control access to shared data, preventing data races.
Why Rust is Better:
Rust's ownership model provides a safe

## 5. Potential for future work
Given more time, there are several potential areas for future work and additional considerations related to Principles of Programming Languages (POPL) aspects. Here are some possibilities:

### - Formal Verification:
Explore formal methods and tools for verifying the correctness and security properties of the cryptographic algorithms implemented in Rust. This involves using formal specifications and proofs to ensure that the code adheres to its intended behavior.

### - Concurrency and Parallelism:
Investigate the potential for concurrent or parallel implementations of cryptographic algorithms in Rust. Explore how Rust's ownership and borrowing system can be leveraged to design concurrent algorithms without introducing data races or other concurrency-related issues.

### - Dynamic Analysis Tools:
Implement runtime analysis tools to dynamically assess memory safety during the execution of cryptographic algorithms. Tools such as runtime checkers or sanitizers can help identify issues that may not be apparent during static analysis.

### - Cross-Language Compatibility:
Consider how Rust cryptographic implementations can interact with existing C/C++ codebases. Explore methods for seamless integration or interoperability between Rust and other languages, addressing real-world scenarios where systems may be composed of multiple languages.

### - Exploration of Other Memory Safety Features:
Investigate additional memory safety features in Rust, such as the borrow checker, and explore how they contribute to preventing common programming errors. Examine how these features could be applied or extended in the context of cryptographic algorithm implementations.

### - Benchmarking and Performance Optimization:
Conduct more extensive benchmarking to evaluate the performance of Rust implementations in various scenarios. Identify opportunities for performance optimization while maintaining memory safety, potentially through the use of advanced language features like unsafe blocks.

### - Extended Cryptographic Algorithm Set:
Expand the scope of cryptographic algorithms covered in the project. Implement and migrate additional algorithms, each highlighting different aspects of memory safety and Rust's features.

### - Security Auditing:
Conduct a comprehensive security audit of the Rust implementations. Engage in code reviews, penetration testing, or collaborate with security experts to identify potential vulnerabilities or areas for improvement in terms of memory safety.

### - Documentation and Educational Resources:
Create comprehensive documentation and educational resources highlighting the memory safety features in Rust and their application to cryptographic algorithm implementations. Share insights gained during the project to contribute to the broader programming community.

Integration with Formal Specification Languages:
Explore integration with formal specification languages like TLA+ or Alloy to formally describe cryptographic algorithm properties and behavior. Verify that the Rust implementations conform to these specifications.
Continued exploration in these areas would contribute to a more comprehensive understanding of how Rust, as a language designed with memory safety in mind, can be effectively utilized in the domain of cryptographic algorithms.
