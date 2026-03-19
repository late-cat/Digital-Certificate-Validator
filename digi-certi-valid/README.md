# Digital Certificate Validator (Soroban)

## Project Description

Digital Certificate Validator is a Soroban smart contract for issuing, storing, revoking, and verifying digital certificates on Stellar.
The contract keeps certificate records on-chain and supports a simple admin-controlled workflow.

## What it does

- Initializes the contract with one admin account.
- Issues certificate records with certificate ID, holder name, course name, issuer, and issue timestamp.
- Stores certificates in persistent Soroban storage.
- Revokes certificates when required.
- Returns certificate details for verification.
- Exposes a validity check endpoint for quick verification.

## Features

- On-chain certificate registry
- Admin-only issue and revoke operations
- Public certificate verification flow
- Duplicate certificate ID protection
- Basic unit tests for issue, verify, and revoke lifecycle

## Deployed Smart Contract Link

https://lab.stellar.org/r/testnet/contract/CBC2GSZYKIK2M43YPUJELGUA4NEUHVYB7LQLNQISFNCXINCVCJ2JBRPK

## Project Structure

```text
digi-certi-valid/
	Cargo.toml
	README.md
	contracts/
		hello-world/
			Cargo.toml
			Makefile
			src/
				lib.rs
				test.rs
```

## Requirements

- Rust toolchain
- Stellar CLI
- Soroban target support (installed automatically when needed by CLI in most setups)

## Important Working Directory

Run all build, test, and deploy commands from:

```bash
cd /Users/bapimondal/stellarx/digi-certi-valid
```

If you run commands from the parent folder, build fails because there is no Cargo.toml there.

## Build and Test

```bash
cd /Users/bapimondal/stellarx/digi-certi-valid
stellar contract build
cargo test -p hello-world
```

You can also use the contract Makefile:

```bash
cd /Users/bapimondal/stellarx/digi-certi-valid/contracts/hello-world
make build
make test
```

## Deploy (Testnet)

Generate and fund an identity (once):

```bash
cd /Users/bapimondal/stellarx/digi-certi-valid
stellar keys generate alice --network testnet --fund
```

Deploy:

```bash
cd /Users/bapimondal/stellarx/digi-certi-valid
stellar contract deploy \
	--wasm target/wasm32v1-none/release/hello_world.wasm \
	--source-account alice \
	--network testnet \
	--alias hello_world
```

## Contract Methods

- init(admin: Address)
- issue_certificate(cert_id, holder_name, course_name, issuer, issued_at)
- revoke_certificate(cert_id)
- get_certificate(cert_id)
- is_certificate_valid(cert_id)
- get_admin()

## Example Invocations (Testnet)

Initialize contract admin:

```bash
stellar contract invoke \
	--id hello_world \
	--source alice \
	--network testnet \
	-- \
	init \
	--admin GBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

Issue a certificate:

```bash
stellar contract invoke \
	--id hello_world \
	--source alice \
	--network testnet \
	-- \
	issue_certificate \
	--cert_id CERT-2026-001 \
	--holder_name "Alice Doe" \
	--course_name "Blockchain Fundamentals" \
	--issuer "Stellar Academy" \
	--issued_at 1742348800
```

Verify validity:

```bash
stellar contract invoke \
	--id hello_world \
	--network testnet \
	-- \
	is_certificate_valid \
	--cert_id CERT-2026-001
```

Fetch certificate details:

```bash
stellar contract invoke \
	--id hello_world \
	--network testnet \
	-- \
	get_certificate \
	--cert_id CERT-2026-001
```

## Deployment Records

Latest testnet deployment:

- Network: testnet
- Contract Alias: hello_world
- Contract ID: CBC2GSZYKIK2M43YPUJELGUA4NEUHVYB7LQLNQISFNCXINCVCJ2JBRPK
- Contract (Stellar Expert): https://stellar.expert/explorer/testnet/contract/CBC2GSZYKIK2M43YPUJELGUA4NEUHVYB7LQLNQISFNCXINCVCJ2JBRPK
- Contract (Stellar Lab): https://lab.stellar.org/r/testnet/contract/CBC2GSZYKIK2M43YPUJELGUA4NEUHVYB7LQLNQISFNCXINCVCJ2JBRPK
- Deploy TX 1: https://stellar.expert/explorer/testnet/tx/169f15cdc7a808a0321e806fc5bcdc5ba0cf4ee92f223afc71a4f36a4a3de84e
- Deploy TX 2: https://stellar.expert/explorer/testnet/tx/300c9123583da89392ce574fdedaf82b108ccad8bf73e73e0f008141bce83b5c

Previous futurenet deployment:

- Contract ID: CAMX7VDY6PEA6S4ZSOMURNMNRHP2G5CE2U5M2IZQ4Q3N3YN5IROY2RKC
- Contract link: https://stellar.expert/explorer/futurenet/contract/CAMX7VDY6PEA6S4ZSOMURNMNRHP2G5CE2U5M2IZQ4Q3N3YN5IROY2RKC
