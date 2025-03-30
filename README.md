Rust Blockchain

This project is a simple blockchain implementation in Rust. It demonstrates fundamental blockchain principles, such as creating a chain of blocks, validating hashes, and ensuring data integrity.

Features
	•	Creates a blockchain with an initial block.
	•	Adds new blocks with proof-of-work validation.
	•	Validates the integrity of the blockchain.

Project Overview

The blockchain consists of multiple blocks linked together using cryptographic hashes. Each block contains:
	•	An ID
	•	A timestamp
	•	Data
	•	A hash derived from the previous block
	•	A nonce for mining

Blocks are added only if they meet validity criteria, ensuring a tamper-proof chain.

Getting Started

Prerequisites
	•	Rust (install from https://www.rust-lang.org/tools/install)

Installation

Clone the repository and navigate to the project directory:

git clone https://github.com/your-repo/rust_block_chain.git
cd rust_block_chain

Running the Project

cargo run

Structure
	•	src/models/block.rs - Defines the block structure.
	•	src/models/blockchain.rs - Implements blockchain functionalities.

License

MIT License
