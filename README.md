# NFT Minter & Swapper

This project contains two Anchor programs for minting NFTs, locking them in a vault, and swapping them for $SOL. The project is built using the Anchor framework on the Solana blockchain.

## Table of Contents

1. [Project Structure](#project-structure)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Building the Project](#building-the-project)
5. [Deploying the Programs](#deploying-the-programs)
6. [Running the Tests](#running-the-tests)
7. [Usage](#usage)
8. [License](#license)


## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://project-serum.github.io/anchor/getting-started/installation.html)
- [Node.js](https://nodejs.org/en/download/) and [npm](https://www.npmjs.com/get-npm)

## Installation

1. Clone the repository:

```sh
git clone https://github.com/your-username/nft_project.git
cd nft_project
```
2. Install dependencies:
```sh
anchor build
```

## Building the Project
To build the project, run:
```sh
anchor build
```

## Deploying the Programs
1. Start a local Solana test validator:
```sh
solana-test-validator
```
2. In a new terminal window, deploy the programs:
```sh
anchor deploy
```
Take note of the program IDs displayed after deployment. You will need these to interact with the programs.

## Running the Tests
To run the tests, execute:
```sh
anchor test
```
This command runs all the tests in the tests/ directory to ensure the programs are working correctly.

## Minting an NFT
To mint an NFT, call the mint_nft function from the nft_minter program with the appropriate parameters. This can be done using the Anchor CLI or by writing a client program in Rust or JavaScript.
```sh
anchor exec --program-id YourProgramIDHere nft_minter mint_nft --uri "https://example.com/nft_metadata"
```
## Creating a Vault
To create a vault, call the create_vault function from the nft_minter program.
```sh
anchor exec --program-id YourProgramIDHere nft_minter mint_nft --uri "https://example.com/nft_metadata"
```

## Creating a Vault
To create a vault, call the create_vault function from the nft_minter program.
```sh
anchor exec --program-id YourProgramIDHere nft_minter create_vault
```

## Locking an NFT
To lock an NFT in the vault, call the lock_nft function from the nft_minter program.
```sh
anchor exec --program-id YourProgramIDHere nft_minter lock_nft --rent 1000
```

## Swapping SOL for an NFT
To swap SOL for an NFT, call the swap_sol_for_nft function from the nft_swapper program.
```sh
anchor exec --program-id YourProgramIDHere nft_swapper swap_sol_for_nft --amount 1000
```
