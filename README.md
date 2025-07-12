# MarketSentinel: Decentralized Real-Time Crypto Price Alerts

MarketSentinel provides a decentralized and low-latency solution for receiving real-time cryptocurrency price alerts. Leveraging on-chain oracles and serverless WebSocket technology, MarketSentinel empowers users to define dynamic price thresholds and receive immediate notifications when those thresholds are met, all without relying on centralized alert services.

This system addresses the limitations of traditional crypto price alert services, which often suffer from latency issues, single points of failure, and lack of transparency regarding price data sources. MarketSentinel ensures data integrity by utilizing verifiable on-chain data feeds, while its serverless architecture guarantees high availability and scalability. This results in a more reliable and trustworthy alert system that provides traders and investors with a significant competitive advantage in the fast-paced crypto market.

MarketSentinel is designed for developers seeking to integrate sophisticated price alert functionality into their applications or build custom trading strategies. By abstracting away the complexities of data aggregation, threshold monitoring, and notification delivery, MarketSentinel allows developers to focus on their core business logic. The modular architecture enables easy customization and extension, making it a powerful tool for a wide range of use cases, including algorithmic trading, risk management, and portfolio monitoring.

The core principle of MarketSentinel is to minimize latency and maximize decentralization. Price data is fetched directly from decentralized oracles, reducing the risk of manipulation or censorship. Notifications are delivered through serverless WebSockets, eliminating the need for dedicated servers and ensuring near-instantaneous delivery of alerts. The entire system is built with performance and security in mind, providing a robust and reliable foundation for real-time crypto price monitoring.

Key Features:

*   **Decentralized Price Data:** Utilizes on-chain oracles (e.g., Chainlink, Band Protocol) to fetch real-time price data, ensuring data integrity and eliminating reliance on centralized exchanges. The specific oracle used is configurable via the environment.

*   **Dynamic Threshold Definitions:** Allows users to define complex price thresholds based on various parameters, such as percentage changes, absolute price levels, and moving averages. Threshold definitions are expressed using a simple configuration language and evaluated on-chain.

*   **Low-Latency Notifications:** Employs serverless WebSocket technology to deliver notifications to users with minimal latency. Alerts are pushed directly to client applications as soon as a threshold is crossed.

*   **Configurable Alert Triggers:** Enables users to customize the conditions under which alerts are triggered, allowing for fine-grained control over notification frequency. This is achieved via configurable trigger logic deployed to the oracle network.

*   **Rust-Based Core Engine:** Implemented in Rust for performance, security, and reliability. Rust's memory safety features minimize the risk of vulnerabilities and ensure stable operation.

*   **Modular Architecture:** Designed with a modular architecture that allows for easy customization and extension. New oracles, notification channels, and threshold types can be added without modifying the core engine.

*   **REST API for Management:** Provides a REST API for managing subscriptions, defining thresholds, and monitoring system status. This allows for seamless integration with existing applications and platforms.

Technology Stack:

*   **Rust:** The primary programming language for the core engine, chosen for its performance, security, and concurrency capabilities.
*   **Actix-Web:** A powerful and pragmatic Rust web framework, used for building the REST API.
*   **WebSockets:** A communication protocol that provides full-duplex communication channels over a single TCP connection, used for real-time notification delivery.
*   **Serverless Functions (AWS Lambda, Google Cloud Functions, or similar):** Used to host the WebSocket server and handle notification delivery, ensuring scalability and high availability.
*   **On-Chain Oracles (Chainlink, Band Protocol, etc.):** Decentralized data feeds providing real-time price data.
*   **Solidity (if utilizing custom oracle contracts):** Used to interact with the oracle contracts on the blockchain.

Installation:

1.  Install Rust: Follow the instructions on the official Rust website ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)) to install Rust and Cargo (Rust's package manager).

2.  Clone the Repository:
    git clone [https://github.com/uhsr/MarketSentinel.git](https://github.com/uhsr/MarketSentinel.git)
    cd MarketSentinel

3.  Build the Project:
    cargo build --release

4. Install required cloud provider CLI tool (e.g. AWS CLI) and configure credentials. This step is necessary for deploying the serverless function.

5. Deploy serverless function (e.g., to AWS Lambda). The deployment process depends on the provider and you should consult their documentation. A sample deployment script might look like this (assuming you have a compiled binary `market_sentinel_websocket`):
   zip deploy.zip market_sentinel_websocket
   aws lambda update-function-code --function-name marketSentinelWebSocket --zip-file fileb://deploy.zip

Configuration:

The following environment variables are required:

*   `ORACLE_ENDPOINT`: The URL of the on-chain oracle (e.g., an Ethereum node endpoint).
*   `ORACLE_CONTRACT_ADDRESS`: The address of the oracle contract on the blockchain.
*   `ALERT_DELIVERY_ENDPOINT`: The WebSocket endpoint where alerts should be delivered. This is usually the URL of your serverless function.
*   `DATABASE_URL`: The URL of the database used to store subscriptions and threshold definitions. We currently support Postgres.

Example setting environment variables:
export ORACLE_ENDPOINT="https://example.com/eth"
export ORACLE_CONTRACT_ADDRESS="0x1234567890abcdef"

Usage:

After deploying the serverless function and configuring the environment variables, you can interact with the MarketSentinel API to manage subscriptions and define thresholds.

Example:

To subscribe to price alerts for Bitcoin with a threshold of $50,000:

curl -X POST -H "Content-Type: application/json" -d '{"asset": "BTC", "threshold": 50000}' <API_ENDPOINT>/subscriptions

Refer to the API documentation (available in the `docs/` directory) for more detailed information on available endpoints and request parameters.

Contributing:

We welcome contributions to MarketSentinel! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise code with thorough documentation.
4.  Submit a pull request with a detailed description of your changes.
5. Ensure all tests pass before submitting the pull request.

License:

This project is licensed under the MIT License. See the [LICENSE](https://github.com/uhsr/MarketSentinel/blob/main/LICENSE) file for details.

Acknowledgements:

We would like to acknowledge the contributions of the Rust community and the developers of the on-chain oracles that make this project possible.