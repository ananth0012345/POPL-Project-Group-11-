# POPL-Project-Group-11
1)Problem Statement (Original Statement):
Migration of C/C++ Cryptographic Algorithms to Rust, Demonstrating Memory Safety
POPL Angle:
The Principles of Programming Languages (POPL) angle in our project lies in the exploration of memory safety aspects when migrating cryptographic algorithms from C/C++ to Rust. POPL focuses on the design and analysis of programming languages, and in this context, our project aims to demonstrate how Rust, with its emphasis on memory safety, can provide a more secure alternative to traditional languages like C/C++ for implementing cryptographic algorithms.
Previous Solutions:
Cryptographic algorithms have been implemented in various programming languages, including C/C++ for their efficiency. However, these languages are notorious for memory-related vulnerabilities such as buffer overflows and data corruption.
Differences in Our Solution:
Our solution involves migrating the RC4 and RSA cryptographic algorithms from C/C++ to Rust, a language known for its focus on memory safety. By doing so, we aim to address and mitigate common vulnerabilities associated with C/C++ implementations. This migration allows for a comparative analysis of the memory safety aspects between the two languages. It's not just about implementing the algorithms; it's about leveraging Rust's features to enhance the security of cryptographic code.


2)Software Architecture Overview:

Components:
RSA Implementation:
RSA cryptographic algorithm implemented in both Rust and C.
RC4 Implementation:
RC4 cryptographic algorithm implemented in both Rust and C.
Performance Measurement and Analysis:
Using Vtune for analyzing the performance of both C and Rust code.

Architecture:
Dual Implementation:
Rust and C implementations coexist for both RSA and RC4 cryptographic algorithms.
Rust Implementation:
Leverages Rust's memory safety features.
C Implementation:
Represents the traditional approach.
Performance Analysis Component:
Vtune is utilized for performance analysis, allowing for a detailed examination of both Rust and C implementations to identify bottlenecks and optimize code.

Testing Component:
Local Testing:
Testing components are primarily placed on the local machine.
Various test cases and inputs are provided to assess the correctness and performance of cryptographic algorithms in both Rust and C.
Vtune is used locally to analyze and profile the performance of the code.

Reuse vs. Development:
Developed Components:
Both RSA and RC4 implementations in Rust and C are developed specifically for this project.
Performance measurement and analysis methods using Vtune are tailored for this project.

Reused Components:
No use of external libraries or frameworks or any available implementations of the algorithms.

Visual Representation:
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

Client-Server Architecture:
Not Applicable

Database:
Not Applicable:

This architecture allows for local testing and analysis of both Rust and C implementations, providing insights into performance differences and memory safety aspects. The use of Vtune aids in in-depth performance analysis.



3)POPL Aspects in RC4 Implementation:

Memory Safety with Indexing:

In the ksa and pgra functions, array indexing is used (s[i] and key[i % key_len]) to access elements safely within bounds.
Avoiding Buffer Overflows:

The use of safe indexing and swapping in the state vector (s.swap(i, j)) helps prevent buffer overflows and ensures memory safety.
Static Typing in Rust:

Types are explicitly declared (s: &mut [u8; 256], key: [u8; 9]), utilizing Rust's static typing for compile-time type checking.
Lifetime Annotations:

Lifetime annotations are not explicitly used in these snippets, but the borrow checker ensures proper referencing and borrowing, contributing to memory safety.
POPL Aspects in RSA Implementation:

Memory Safety with Vec<u8>:

The use of Vec<u8> for data storage ensures dynamic memory allocation and deallocation, contributing to memory safety.
Error Handling with Result Type:

The expect method is used for error handling after encrypting and decrypting data, ensuring that any errors during these operations are handled explicitly.
Static Typing and Explicit Type Conversion:

Types are explicitly declared (bits: usize, string: String), and there's explicit type conversion (string.as_bytes()) to ensure type safety.
Use of External Libraries (rsa, rand, hex):

External libraries like rsa, rand, and hex are utilized, and their use aligns with memory safety principles as they are expected to be implemented with those considerations.
Difficulties and Experiences:

Transition from C to Rust:

Moving from the C-style manual memory management to Rust's ownership model might pose challenges. Ensuring lifetimes and borrowing are appropriately managed could be an initial difficulty.
Understanding External Libraries:

Incorporating and understanding external libraries, such as rsa and hex, might require familiarity with their documentation and potentially adjusting code to match their conventions.
Error Handling Paradigm Shift:

Shifting from C-style error handling to Rust's Result type might be challenging initially. Ensuring comprehensive error checking is crucial for robust code.
Ensuring Memory Safety in Cryptographic Operations:

Given the sensitivity of cryptographic operations, thorough testing and validation are essential to ensure memory safety, avoiding potential vulnerabilities.


4)
