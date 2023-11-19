# POPL-Project-Group-11
Problem Statement (Original Statement):
Migration of C/C++ Cryptographic Algorithms to Rust, Demonstrating Memory Safety
POPL Angle:
The Principles of Programming Languages (POPL) angle in our project lies in the exploration of memory safety aspects when migrating cryptographic algorithms from C/C++ to Rust. POPL focuses on the design and analysis of programming languages, and in this context, our project aims to demonstrate how Rust, with its emphasis on memory safety, can provide a more secure alternative to traditional languages like C/C++ for implementing cryptographic algorithms.
Previous Solutions:
Cryptographic algorithms have been implemented in various programming languages, including C/C++ for their efficiency. However, these languages are notorious for memory-related vulnerabilities such as buffer overflows and data corruption.
Differences in Our Solution:
Our solution involves migrating the RC4 and RSA cryptographic algorithms from C/C++ to Rust, a language known for its focus on memory safety. By doing so, we aim to address and mitigate common vulnerabilities associated with C/C++ implementations. This migration allows for a comparative analysis of the memory safety aspects between the two languages. It's not just about implementing the algorithms; it's about leveraging Rust's features to enhance the security of cryptographic code.


Software Architecture Overview:

Components:

Cryptographic Algorithms Implementation:
Both Rust and C implementations of RC4 and RSA cryptographic algorithms.
Performance Measurement Module:
Measures and records the execution time of the algorithms in both Rust and C.
Architecture:

Dual Implementation:
Rust and C implementations coexist for each cryptographic algorithm.
Rust Implementation:
Leverages Rust's memory safety features.
C Implementation:
Represents the traditional approach with potential memory vulnerabilities.
Client-Server Architecture:

Client Side:
The client side includes the user interface or command-line interface for interacting with the cryptographic algorithms.
It initiates the execution of the algorithms and records the time taken for each implementation.
Server Side:
In this context, the "server" doesn't necessarily mean a networked server. It represents the environment where the cryptographic algorithms are executed.
Hosts the implementations in both Rust and C.
Testing Component:

Local Testing:
The testing component is primarily placed on the local machine.
Various test cases and inputs are provided to assess the correctness and performance of the cryptographic algorithms.
This local testing ensures that the implementations are functional and secure within the development environment.
Database:

No Explicit Database:
Since the focus is on cryptographic algorithms and their memory safety, there might not be a need for a database in the traditional sense.
However, you may have a data storage component for recording and analyzing performance metrics if required.
Reuse vs. Development:

Reused Components:
Cryptographic algorithms themselves may be based on well-established standards, and you might reuse parts of existing libraries or implementations (e.g., OpenSSL).
Developed Components:
The memory safety features and specific implementations in Rust are likely developed from scratch or adapted from existing Rust libraries.
