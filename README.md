# QuantaSignal: Quantifying Forex Signal Efficacy with RNN-Driven Feature Selection

QuantaSignal is a Rust-based backtesting framework designed to rigorously evaluate the performance of Forex trading signals. It provides a robust and efficient environment for analyzing historical market data and simulating trading strategies, enabling traders and developers to objectively assess the viability of various signal generation methodologies. Beyond simple backtesting, QuantaSignal integrates a recurrent neural network (RNN) module for advanced feature selection, allowing users to identify the most impactful indicators and market parameters for optimizing signal generation and improving trading outcomes. This combined approach provides a powerful toolkit for evidence-based Forex trading strategy development.

The core purpose of QuantaSignal is to bridge the gap between theoretical signal generation and practical trading profitability. Traditional backtesting methods often lack the sophistication to handle complex market dynamics and interdependencies between indicators. QuantaSignal addresses this limitation by incorporating RNN analysis to dynamically assess feature importance, adapting to evolving market conditions and improving predictive accuracy. The framework supports various data sources, signal formats, and trading strategies, offering a flexible and customizable platform for a wide range of research and development applications. The ability to quantify signal efficacy objectively is paramount to successful trading, and QuantaSignal empowers users with the tools to achieve this level of analytical rigor.

By leveraging the performance and safety of Rust, QuantaSignal ensures both speed and reliability in backtesting simulations. The RNN module is built upon established deep learning libraries and optimized for efficient training and inference on time-series Forex data. The framework's modular design allows for easy extension and integration with other trading platforms and data providers. The goal is to provide a comprehensive and user-friendly environment that empowers users to iteratively refine their trading strategies based on quantifiable evidence, ultimately leading to more informed and profitable trading decisions.

## Key Features

*   **Comprehensive Backtesting Engine:** Supports various order types (market, limit, stop), slippage modeling, commission structures, and risk management parameters. It allows for realistic simulation of trading execution under different market conditions. The backtesting engine utilizes vectorized operations for optimal performance, enabling rapid simulation of large historical datasets.

*   **Recurrent Neural Network (RNN) Feature Selection:** Implements an RNN-based module to analyze the impact of different technical indicators and market variables on signal performance. Specifically, it uses Long Short-Term Memory (LSTM) networks to capture temporal dependencies and identify the most predictive features for optimizing signal generation.

*   **Customizable Signal Format Support:** Allows users to define their own signal formats and data sources, providing flexibility for integrating with existing trading systems and data feeds. The framework includes parsers for common data formats like CSV and JSON.

*   **Detailed Performance Reporting:** Generates comprehensive reports on backtesting results, including metrics like profit factor, Sharpe ratio, maximum drawdown, and win rate. Visualizations are provided to illustrate performance trends and identify areas for improvement.

*   **Modular Architecture:** Designed with a modular architecture that allows for easy extension and customization. New indicators, trading strategies, and data sources can be added without modifying the core framework.

*   **Rust's Safety and Performance:** Built using Rust, leveraging its memory safety features and high performance for reliable and efficient backtesting simulations. Rust's ownership system prevents common errors such as data races and null pointer dereferences.

*   **Parallel Processing:** Utilizes Rust's concurrency features to parallelize backtesting simulations, allowing for faster analysis of large datasets and complex trading strategies. This significantly reduces the time required to evaluate different signal parameters and market conditions.

## Technology Stack

*   **Rust:** The primary programming language, chosen for its performance, safety, and concurrency features.
*   **ndarray:** Rust library for numerical computation and array manipulation, used for handling market data and indicator calculations.
*   **serde:** Rust library for serialization and deserialization, used for handling data input and output in various formats.
*   **rand:** Rust library for random number generation, used for simulating market volatility and slippage.
*   **tch-rs:** Rust bindings for PyTorch, used for implementing the RNN feature selection module. This allows utilizing the powerful deep learning capabilities of PyTorch within the Rust framework.
*   **csv:** Rust library for reading and writing CSV files, a common format for Forex market data.

## Installation

1.  Ensure you have Rust and Cargo installed. You can download them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  Clone the QuantaSignal repository:
    git clone [https://github.com/jjfhwang/QuantaSignal](https://github.com/jjfhwang/QuantaSignal)
    cd QuantaSignal

3.  Build the project:
    cargo build --release

4. Install the necessary system dependencies for `tch-rs` (PyTorch bindings). Follow the instructions at [https://github.com/LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs). This generally involves installing `libtorch`.

5. Verify the installation by running the example:
    cargo run --release --example basic_backtest

## Configuration

The application is configured through environment variables and configuration files.

*   **Environment Variables:**
    *   `DATA_DIR`: Specifies the directory where historical market data is stored. Defaults to `./data`.
    *   `MODEL_PATH`: Specifies the path to the pre-trained RNN model. Defaults to `./model/rnn_model.pt`.
    *   `COMMISSION`: The commission per trade, expressed as a decimal (e.g., 0.0001 for 0.01%). Defaults to 0.0001.

*   **Configuration Files:**
    Trading strategies and backtesting parameters are defined in YAML configuration files. The structure of these files is documented in the `examples` directory. A sample configuration file might look like this:

    

## Usage

1.  **Data Preparation:** Prepare historical Forex data in a supported format (e.g., CSV). Ensure the data includes open, high, low, close, and volume (OHLCV) information for each time period. Store the data in the directory specified by the `DATA_DIR` environment variable.

2.  **Configuration:** Create a YAML configuration file specifying the trading strategy, backtesting parameters, and data source.

3.  **Running Backtests:** Use the `quanta_signal` executable to run backtests. For example:
    ./target/release/quanta_signal --config config.yaml

4.  **RNN Feature Selection:** To train or use the pre-trained RNN model for feature selection, refer to the documentation in the `rnn` directory. This requires additional setup and data preprocessing steps. An example of how to use the RNN is provided.

## Contributing

We welcome contributions to QuantaSignal! Please follow these guidelines:

*   Fork the repository and create a new branch for your feature or bug fix.
*   Write clear and concise commit messages.
*   Include unit tests for your code.
*   Submit a pull request with a detailed description of your changes.
*   Adhere to the Rust code style guidelines.
*   Ensure that your code builds and passes all tests before submitting a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/QuantaSignal/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the Rust community for providing excellent tools and libraries, and the PyTorch community for their contributions to deep learning.