# rust-zk
# SVM Framework on Solana

This project implements a simple **Support Vector Machine (SVM)** framework on **Solana** using **Rust**. The SVM model can make predictions based on input data points, with all operations performed on-chain using Solana smart contracts. The client interacts with the Solana program to make predictions using pre-trained SVM models.

## Features
- A simple linear **SVM model** that performs predictions.
- **Solana smart contract** (program) that executes SVM predictions on-chain.
- A **client** to send data to the Solana program and retrieve predictions.

---

## Prerequisites

Before getting started, make sure you have the following tools installed:

### 1. Rust
Install Rust using [rustup](https://rustup.rs/).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


