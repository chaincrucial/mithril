---
sidebar_position: 2
---

import NetworksMatrix from '../../../networks-matrix.md';
import CompiledBinaries from '../../../compiled-binaries.md'

# Mithril Signer Node

:::info

This is the node of the **Mithril Network** responsible for producing individual signatures that are collected and aggregated by the **Mithril Aggregator**.

:::

:::tip

* For more information about the **Mithril Network**, please refer to the [Architecture](../../../mithril/mithril-network/architecture.md) page.

* For more information about the **Mithril Signer**, please refer to the [Signer Node](../../../mithril/mithril-network/signer.md) page.

* Checkout the [`Run a Mithril Signer node (SPO)`](../../getting-started/run-mithril-devnet.md) guide.

:::

:::note Mithril Networks

<NetworksMatrix />

:::

## Resources

| Node | Source Repository | Rust Documentation | Docker Packages |
|:-:|:-----------------:|:------------------:|:---------------:|
**Mithril Signer** | [:arrow_upper_right:](https://github.com/input-output-hk/mithril/tree/main/mithril-signer) | [:arrow_upper_right:](https://mithril.network/mithril-signer/doc/mithril_signer/index.html) | [:arrow_upper_right:](https://github.com/input-output-hk/mithril/pkgs/container/mithril-signer)

## Pre-requisites

* Install a [correctly configured](https://www.rust-lang.org/learn/get-started) Rust toolchain (latest stable version)

* Install OpenSSL development libraries, for example on Ubuntu/Debian/Mint run `apt install libssl-dev`

## Download source

Download from GitHub (HTTPS)

```bash
git clone https://github.com/input-output-hk/mithril.git
```

Or (SSH)

```bash
git clone git@github.com:input-output-hk/mithril.git
```

Switch to build branch / tag

```bash
# **YOUR_BUILD_BRANCH_OR_TAG** depends on the Mithril network you target, 
# please refer to the **Build From** column of the above **Mithril Networks** table
git switch **YOUR_BUILD_BRANCH_OR_TAG**
```

Change directory

```bash
cd mithril/mithril-signer
```

## Development test and build

Run tests

```bash
make test
```

Create the help menu

```bash
make help
```

Generate the Rust documentation

```bash
make doc
```

Run in debug mode with default configuration

```bash
make debug
```

## Release build and run binary

Build and run in release with default configuration

```bash
make run
```

Or, build only in release

```bash
make build
```

Display the help menu

```bash
./mithril-signer --help
```

You should see

```bash
An implementation of a Mithril Signer

Usage: mithril-signer [OPTIONS]

Options:
  -r, --run-mode <RUN_MODE>
          Run Mode [env: RUN_MODE=] [default: dev]
  -v, --verbose...
          Verbosity level, add more v to increase
  -c, --configuration-dir <CONFIGURATION_DIR>
          Directory where the configuration file is located [default: ./config]
      --disable-digests-cache
          Disable immutables digests cache
      --reset-digests-cache
          If set the existing immutables digests cache will be reset
  -h, --help
          Print help information (use `--help` for more detail)
  -V, --version
          Print version information
```

Run in release with default configuration

```bash
./mithril-signer
```

Run in release with a specific mode

```bash
./mithril-signer -r preview
```

Run in release with a custom configuration via env vars

```bash
NETWORK=**YOUR_CARDANO_NETWORK** AGGREGATOR_ENDPOINT=**YOUR_AGGREGATOR_ENDPOINT** ./mithril-signer
```

:::tip

If you want to dig deeper, you can get access to several level of logs from the Mithril Signer:

* Add `-v` for some logs (WARN)
* Add `-vv` for more logs (INFO)
* Add `-vvv` for even more logs (DEBUG)
* Add `-vvvv` for all logs (TRACE)

:::

## Download pre-built binary

<CompiledBinaries />

## Build and run Docker container

Build a local Docker image

```bash
make docker-build
```

Run a local Docker container

```bash
make docker-run
```

## Configuration parameters

The configuration parameters are set either:

* In a configuration file (depending on the `--run-mode` parameter). If runtime mode is `testnet` the file is located in `./conf/testnet.json`.
* The value can be overridden by an environment variable whose name is the parameter name uppercased.

Here is a list of the available parameters:

| Parameter | Command Line (long) |  Command Line (short) | Environment Variable | Description | Default Value | Example | Mandatory |
|-----------|---------------------|:---------------------:|----------------------|-------------|---------------|---------|:---------:|
| `verbose` | `--verbose` | `-v` | `VERBOSE` | Verbosity level | - | Parsed from number of occurrences: `-v` for `Warning`, `-vv` for `Info`, `-vvv` for `Debug` and `-vvvv` for `Trace` | :heavy_check_mark: |
| `run_mode` | `--run-mode` | `-r` | `RUN_MODE` | Runtime mode | `dev` | - | :heavy_check_mark: |
| `db_directory` | `--db-directory` | - | `DB_DIRECTORY` | Directory to snapshot from the **Cardano Node** | `/db` | - | :heavy_check_mark: |
| `network` | - | - | `NETWORK` | Cardano network | - | `testnet` or `mainnet` or `devnet` | :heavy_check_mark: |
`network_magic` | - | - | `NETWORK_MAGIC` | Cardano Network Magic number (for `testnet` and `devnet`) | - | `1097911063` or `42` | - |
| `party_id` | - | - | `PARTY_ID` | Party Id of the signer, usually the `Pool Id` of the SPO | - | `pool1pxaqe80sqpde7902er5kf6v0c7y0sv6d5g676766v2h829fvs3x` | - | Mandatory in `Pool Id Declaration Mode`  where the owner is not verified (decommissioned, only available when built with `allow_skip_signer_certification` feature, for test only)
| `run_interval` | - | - | `RUN_INTERVAL` | Interval between two runtime cycles in ms | - | `60000` | :heavy_check_mark: |
| `aggregator_endpoint` | - | - | `AGGREGATOR_ENDPOINT` | Aggregator node endpoint | - | `https://aggregator.pre-release-preview.api.mithril.network/aggregator` | :heavy_check_mark: |
| `data_stores_directory` | - | - | `DATA_STORES_DIRECTORY` | Directory to store signer data (Stakes, Protocol initializers, ...) | - | `./mithril-signer/stores` | :heavy_check_mark: |
| `store_retention_limit` | - | - | `STORE_RETENTION_LIMIT` | Maximum number of records in stores. If not set, no limit is set. | - | - | - |
| `kes_secret_key_path` | - | - | `KES_SECRET_KEY_PATH` | Path to the `Cardano KES Secret Key` file. Mandatory in `Pool Id Certification Mode` where the owner is verified (experimental, soon to be stable & preferred mode) | - | - | - |
| `operational_certificate_path` | - | - | `OPERATIONAL_CERTIFICATE_PATH` | Path to the `Cardano Operational Certificate` file. Mandatory in `Pool Id Certification Mode` where the owner is verified (experimental, soon to be stable & preferred mode) | - | - | - |
| `era_reader_adapter_type` | `--era-reader-adapter-type` | - | `ERA_READER_ADAPTER_TYPE` | Era reader adapter type that can be `cardano-chain`, `file` or `bootstrap`. | `bootstrap` | - | - |
| `era_reader_adapter_params` | `--era-reader-adapter-params` | - | `ERA_READER_ADAPTER_PARAMS` | Era reader adapter params that is an optional JSON encoded parameters structure that is expected depending on the `era_reader_adapter_type` parameter | - | - | - |
