# QuantaSignal: High-Performance Signal Processing in Rust

A robust and efficient library for real-time signal processing and analysis.

QuantaSignal is a meticulously crafted Rust library designed to provide developers with a powerful and versatile toolkit for manipulating, analyzing, and generating signals. Built with performance and precision in mind, QuantaSignal leverages Rust's memory safety and concurrency features to deliver high-speed signal processing capabilities without sacrificing code reliability. This library is suitable for a wide range of applications, including audio processing, communications systems, scientific computing, and financial modeling. It aims to be a comprehensive solution for both novice and experienced signal processing engineers seeking a performant and reliable platform.

This library prioritizes a modular design, allowing users to select and integrate only the specific modules required for their application. This minimizes overhead and improves overall efficiency. The core components are optimized for single instruction, multiple data (SIMD) processing where applicable, utilizing Rust's ability to integrate with platform-specific intrinsics for maximum performance. Furthermore, the library offers both low-level functions for fine-grained control and high-level abstractions for ease of use, catering to different levels of user expertise and project requirements. QuantaSignal is continuously evolving, with planned additions including advanced filtering techniques, adaptive algorithms, and support for hardware acceleration.

One of the primary benefits of QuantaSignal is its strong focus on safety and reliability, inherent characteristics of the Rust language. The library incorporates rigorous unit tests and integration tests to ensure the accuracy and stability of all components. Memory management is handled automatically by Rust's ownership and borrowing system, eliminating the risk of common programming errors such as memory leaks and segmentation faults. This makes QuantaSignal a trustworthy foundation for building mission-critical signal processing applications. This approach sets it apart from other signal processing libraries, many of which are written in languages susceptible to memory-related vulnerabilities.

## Key Features

*   **Fast Fourier Transform (FFT):** Implementation of the Cooley-Tukey FFT algorithm optimized for real-valued signals. Utilizes a radix-2 decimation-in-time approach with precomputed twiddle factors for improved speed. Supports both forward and inverse transforms with configurable input and output scaling. Includes complex number support via the `num-complex` crate.
*   **Window Functions:** A collection of window functions including Hamming, Hanning, Blackman, and Kaiser windows. Provides implementations that generate window coefficients using vectorized operations. Offers customizable window lengths and parameter adjustments, such as the beta parameter for the Kaiser window.
*   **Filtering:** Includes implementations of Finite Impulse Response (FIR) and Infinite Impulse Response (IIR) filters. Supports direct form I and II structures. The FIR filter implementation utilizes the overlap-add method for efficient processing of long signals. The IIR filter design is currently limited to Butterworth filters but will be expanded in future versions.
*   **Signal Generation:** Functions for generating common signal types such as sine waves, square waves, and white noise. The sine wave generator supports frequency modulation and phase adjustment. The white noise generator utilizes a cryptographically secure random number generator for high-quality noise.
*   **Signal Statistics:** Computes statistical measures such as mean, variance, standard deviation, and root mean square (RMS) value. Implementations are optimized for both single-threaded and multi-threaded execution using Rust's `rayon` crate for parallelization.
*   **Resampling:** Implements bandlimited interpolation for signal resampling. Supports both upsampling and downsampling with configurable interpolation filters. Uses a polyphase filter structure for efficient implementation.
*   **Audio I/O:** Integration with the `cpal` crate for cross-platform audio input and output. Provides functions for capturing audio from a microphone or playing audio to a speaker. Supports multiple audio formats and sample rates.

## Technology Stack

*   **Rust:** The primary programming language, known for its performance, memory safety, and concurrency features.
*   **`num-complex`:** A Rust crate for complex number arithmetic, used extensively in FFT and other signal processing algorithms.
*   **`rayon`:** A data parallelism library for Rust, enabling efficient multi-threaded execution of computationally intensive tasks.
*   **`rand`:** A Rust crate for random number generation, used for generating noise signals and other applications.
*   **`cpal`:** A cross-platform audio I/O library for Rust, enabling audio capture and playback.

## Installation

1.  Ensure you have Rust installed and configured correctly. You can download and install Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  Clone the QuantaSignal repository from GitHub:
    git clone https://github.com/jjfhwang/QuantaSignal.git

3.  Navigate to the project directory:
    cd QuantaSignal

4.  Build the project using Cargo:
    cargo build --release

## Configuration

The QuantaSignal library utilizes environment variables for certain configurable parameters.

*   `QUANTASIGNAL_THREAD_COUNT`: Specifies the number of threads to use for parallel processing. If not set, the library will automatically determine the optimal number of threads based on the system's CPU cores. Setting this variable can be useful for fine-tuning performance on specific hardware. Example: `export QUANTASIGNAL_THREAD_COUNT=8`

*   `QUANTASIGNAL_DEFAULT_SAMPLE_RATE`: Sets the default sample rate for audio processing. This value is used as a fallback if the sample rate is not explicitly specified in the audio I/O functions. Example: `export QUANTASIGNAL_DEFAULT_SAMPLE_RATE=44100`

## Usage

Example: Performing an FFT on a generated sine wave.

First add the following to your Cargo.toml

Then in your rust code:



## Contributing

We welcome contributions to the QuantaSignal project! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise code with appropriate comments.
4.  Include unit tests and integration tests for your changes.
5.  Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/QuantaSignal/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the Rust community and the developers of the `num-complex`, `rayon`, `rand`, and `cpal` crates for their contributions to the Rust ecosystem.