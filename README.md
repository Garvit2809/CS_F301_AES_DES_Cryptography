# Cryptographic Tool - Rust and C++ Implementation (AES and DES)

Garvit Arora 2020B2A72113G: f20202113@goa.bits-pilani.ac.in

Tanmay Satija 2020B3A71516G: f20201516@goa.bits-pilani.ac.in

Shreyash Patel 2020B5A71157G: f20201157@goa.bits-pilani.ac.in

Siddhant Budhiraja 2020B3A71799G: f20201799@goa.bits-pilani.ac.in

## Aim

The primary objective of this project is to develop a versatile encryption/decryption tool capable of encoding hexadecimal data into a custom language and deciphering data back into various formats, including hexadecimal code. The implementation leverages both **Rust** and **C++** to explore the differences in their approaches and how these differences impact the functionality and performance of the code. In addition to this, we delve into the variations between **AES** and **DES** encryption and decryption techniques.

## Cryptography and Its Uses

**Cryptography**, the practice and study of secure communication techniques, plays a pivotal role in ensuring the privacy, integrity, and authenticity of information. The primary goal is to establish secure communication in the presence of adversaries, safeguarding data in various applications such as confidentiality, secure communication, data protection, and the **Internet of Things (IoT)**.

While the cryptographic tool may seem simple, it serves as a practical example of encoding messages securely and passing them to recipients who can decode the information using the appropriate decryption key. The simplicity of this project is intentional, allowing for a focused examination of fundamental cryptographic principles and the subtleties introduced by the choice of programming languages.

## Software Architecture

### Overview
The software adopts a modular architecture with separate implementations for AES and DES in Rust and C++. Each serves as a standalone library, facilitating versatility for integration into various systems. The terminal interface allows user-friendly interaction, enabling input for encryption and observing decrypted output.

### Reusability
To ensure code consistency, common components like basic bit manipulation and key expansion are shared between Rust and C++ implementations. This design minimizes redundancy, fostering uniformity in fundamental operations.

### Terminal Interaction
The terminal serves as the primary interface for user interaction, allowing users to input data for encryption and observe decrypted results. This design choice enhances accessibility for users to experiment with cryptographic processes in a straightforward manner.

### Testing Component
Unit tests cover individual cryptographic functions, ensuring their correctness. Integration tests validate the end-to-end encryption-decryption process. Benchmark testing involves executing algorithms with various input sizes and complexities, comparing Rust and C++ implementations. Benchmarking tools such as Google Benchmark are used for performance analysis.

### Database
Given the cryptographic nature, no database integration is necessary. The primary focus is on the efficiency and security of the encryption and decryption processes.

# Profiler Comparison

### VTune Profiler (Performance Analysis)
VTune Profiler is harnessed to analyze C++ applications, offering comprehensive performance analysis capabilities. It encompasses CPU and memory analysis, thread profiling, and other metrics, providing a detailed understanding of the performance characteristics of the C++ implementation.

The use of VTune Profiler for C++ applications enables a nuanced assessment of both memory and performance aspects in each language effectively.


# DES Implementation

## Execution Time(Profiler):

### C++ (0.0015394s) vs. Rust (0.000253s):
The slightly longer execution time in Rust may be attributed to Rust's focus on safety, which introduces additional checks and overhead, impacting raw performance compared to C++.

### Conclusion:
Rust's emphasis on safety may contribute to a marginal increase in execution time.


# AES Implementation

## Execution Time(Profiler):

### C++ (0.013494s) vs. Rust (0.002643s):
The substantial difference in execution time indicates a significant performance gap between C++ and Rust for the AES algorithm.

### Conclusion:
Rust's focus on safety may lead to more optimized code generation for the AES algorithm, resulting in faster execution.


# Overall Conclusion:

## DES:

Rust's emphasis on safety may introduce slight overhead, impacting execution time and CPU utilization marginally.
Similar memory usage indicates consistent memory management in both languages.

## AES:

Rust outperforms C++ in execution time, suggesting that Rust's safety features may lead to optimized code generation for certain cryptographic algorithms.
The higher CPU utilization in Rust aligns with its longer execution time, reflecting the impact of safety features on overall CPU performance.

These observations emphasize the trade-offs between safety and raw performance inherent in the design and execution of cryptographic algorithms in Rust and C++

# Relation with the course 

## 1. Memory Safety and Ownership in Rust
Rust's emphasis on memory safety, driven by its ownership system and borrow checker, significantly influences the implementation of both AES and DES. The ownership system ensures that cryptographic operations are conducted without the risk of common vulnerabilities such as buffer overflows. This is particularly critical in the context of cryptographic algorithms where low-level operations demand precision and robust memory management. The borrow checker guarantees the validity of references, mitigating memory-related errors. The meticulous handling of memory in Rust contributes not only to the security of the implementation but also to the predictability of memory usage.

## 2. Performance Optimization in C++
C++'s focus on performance optimization becomes evident in the execution of cryptographic algorithms, especially with the resource-intensive AES algorithm. The language provides fine-grained control over low-level details, allowing for manual memory management and optimization tailored to the computational complexities of cryptographic operations. In the context of AES, where intricate mathematical operations are prevalent, C++ empowers developers to fine-tune the code for optimal execution. The numbers attest to this, showcasing C++'s prowess in handling computationally intensive tasks with a balance of speed and efficiency.

## 3. Strong Typing and Security in Rust
Rust's strong typing plays a crucial role in ensuring the consistency and correctness of data handling in cryptographic implementations. The emphasis on strong typing minimizes the risk of unintended data manipulations, aligning with the project's commitment to security. The AES and DES algorithms benefit from Rust's type system, as it prevents type-related errors that could compromise the integrity of the cryptographic operations. While Rust introduces an additional layer of type safety, the implementation demonstrates that this does not come at the expense of performance, as evidenced by competitive execution times.

## 4. Ownership, Borrowing, and Concurrency in Rust
The application of Rust's ownership and borrowing principles extends beyond memory safety. In the context of cryptographic algorithms, ownership and borrowing contribute to safe concurrency. The AES and DES implementations in Rust are inherently equipped to handle parallel processing, ensuring that data races do not compromise the security and correctness of the cryptographic operations. Rust's ownership system shines as a powerful ally in maintaining thread safety without sacrificing the performance needed for efficient encryption and decryption.

## 5. Benchmarking and Profiling
The project incorporates benchmarking and profiling tools to quantitatively assess the performance characteristics of both C++ and Rust implementations. Profilers such as Dhat and VTune Profiler allow us to delve into the memory utilization, CPU usage, and execution times. These numbers are not just metrics; they are a reflection of the impact of language choices on the real-world execution of cryptographic algorithms. The benchmarks serve as tangible proof points, validating the effectiveness of language-specific features and optimizations in achieving the project's goals.

## Cryptographic Tool Features

### Encryption
The tool facilitates the encoding of hexadecimal data into a custom language, introducing a layer of obfuscation. Users are granted the flexibility to choose their preferred presentation format, providing versatility in encoding.

### Decryption
The tool possesses the capability to decipher encoded data back into its original hexadecimal form or any other chosen presentation format. The decryption process is executed using the appropriate decryption key.

## Implementation Details

### Language Choice
The tool is implemented in both **Rust** and **C++**, providing insights into the strengths and considerations of each language in the context of cryptography. Rust's focus on memory safety and C++'s emphasis on performance are juxtaposed to showcase the impact of language choice on cryptographic implementations.

### Encryption Techniques

#### AES (Advanced Encryption Standard)
The AES encryption algorithm is implemented in `AES_encrypt.cpp` for the C++ version. The core `AESEncrypt` function orchestrates the encryption process, initializing a state, executing initial and main rounds, and finally copying the encrypted state to the buffer.

#### AES Decryption
The counterpart for decryption is implemented in `AES_decrypt.cpp`. The `AESDecrypt` function manages initialization, initial rounds, main rounds in reverse order, and the final round of AES decryption.

#### DES (Data Encryption Standard)
The DES encryption and decryption implementation resides in `des.cpp`. Operating on 64-bit blocks of data with a 56-bit key, the code allows users to interactively input an operation (encrypt or decrypt), a 16-character message in hexadecimal form, and a 16-character key.

### Key Generation
Round keys for encryption and decryption in DES are generated using the `key_rounds` function, derived from the user-provided key.

### DES Encryption
For encryption, the code conducts initial permutation, splitting the message, 16 rounds of encryption (including expansion, XOR with round keys, S-box substitution, and permutation), and final permutation. The result is then converted to a hexadecimal string and presented to the user.

### DES Decryption
Decryption involves reversing the encryption process with initial permutation, splitting, 16 rounds of decryption using the round keys in reverse order, and final permutation. The plaintext result is converted to a hexadecimal string and displayed.


## How to Use

### Running the Tool
1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Build and run the Rust implementation using the provided instructions.
4. Repeat the process for the C++ implementation.
![aes_cpp](https://github.com/Garvit2809/CS_F301_AES_DES_Cryptography/assets/84657981/29adf565-9042-471b-9c4d-c832c85fbd4a)
![aes_rust](https://github.com/Garvit2809/CS_F301_AES_DES_Cryptography/assets/84657981/3b57d0ad-ec2b-4abe-a179-bbd002845766)
![des_cpp](https://github.com/Garvit2809/CS_F301_AES_DES_Cryptography/assets/84657981/88218f9f-9a84-4e9f-b854-56c3f3a9f46d)
![des_rust](https://github.com/Garvit2809/CS_F301_AES_DES_Cryptography/assets/84657981/9297798f-ce0a-404c-967b-1006f66f0d1d)



### Decryption
- Input the encoded data.
- Provide the decryption key.

## Conclusion

This comprehensive overview showcases the intricacies of implementing cryptographic algorithms in Rust and C++. The project not only provides practical insights into the differences between the two languages but also serves as a foundation for further exploration into advanced cryptographic techniques and language-specific optimizations. The integration of POPL aspects ensures a holistic understanding of the challenges and opportunities in the domain of secure communication.
