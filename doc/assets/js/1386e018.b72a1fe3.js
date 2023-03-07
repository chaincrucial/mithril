"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[335],{47385:e=>{e.exports=JSON.parse('{"blogPosts":[{"id":"/2023/03/02/era-switch-feature","metadata":{"permalink":"/doc/dev-blog/2023/03/02/era-switch-feature","source":"@site/blog/2023-03-02-era-switch-feature/index.md","title":"Mithril Era Switch","description":"An new Era Switch behavior will be introduced soon to the Mithril networks","date":"2023-03-02T00:00:00.000Z","formattedDate":"March 2, 2023","tags":[{"label":"era","permalink":"/doc/dev-blog/tags/era"},{"label":"era activation","permalink":"/doc/dev-blog/tags/era-activation"},{"label":"era markers","permalink":"/doc/dev-blog/tags/era-markers"},{"label":"era switch","permalink":"/doc/dev-blog/tags/era-switch"},{"label":"hard fork","permalink":"/doc/dev-blog/tags/hard-fork"}],"readingTime":1.47,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Mithril Era Switch","authors":[{"name":"Mithril Team"}],"tags":["era","era activation","era markers","era switch","hard fork"]},"nextItem":{"title":"Mithril Release Process","permalink":"/doc/dev-blog/2022/12/05/release-process"}},"content":"### An new Era Switch behavior will be introduced soon to the Mithril networks\\n\\n**Epic**: `Implement eras behavior switch #707](Implement eras behavior switch` [#707](https://github.com/input-output-hk/mithril/issues/707)\\n\\n:warning: The Era Switch is not deployed yet to the `pre-release-preview` and `release-preprod` network. A special announcement will be made on the **moria** Discord channel when a new release condaidate distribution is ready.\\n\\nIn order to guarantee that any breaking change of the Mithril nodes does not break the Certificate Chain and the that new snapshots are consistently produced, the Mithril team has developped an Era Switch Behavior. This mechanism enables to embed new features in the signer and aggregator nodes prior ro releasing them. Also the activation of these new features will take place in a coordinated manner: all the eligible nodes will hot switch to a new era at the same Cardano epoch transition. To do so, the nodes rely on a transaction that is stored on the Cardano chain that provides era markers with the associated activations epochs for the eras.\\n\\n:fire: Activating this feature will require an update of configuration of the signer nodes after updating their binary:\\n- The `ERA_READER_ADAPTER_TYPE` env var must be set to `cardano-chain`\\n- The `ERA_READER_ADAPTER_PARAMS` env var must be set to the result of the command `jq -nc --arg address $(wget -q -O - **YOUR_ERA_READER_ADDRESS**) --arg verification_key $(wget -q -O - **YOUR_ERA_READER_VERIFICATION_KEY**) \'{\\"address\\": $address, \\"verification_key\\": $verification_key}\'` (the ****YOUR_ERA_READER_ADDRESS**** and ****YOUR_ERA_READER_VERIFICATION_KEY**** are values provided in the networks configuration matrix)\\n\\nAll theses information will be available at the updated [`Run a Mithril Signer node (SPO)`](https://mithril.network/doc/manual/getting-started/run-signer-node) guide.\\n\\nHere is a schema that illustrates the era switch behavior:\\n[![Era Switch Schema](./img/schema.jpg)](./img/schema.jpg)\\n\\nMore information is also available at this [ADR](https://mithril.network/doc/adr/4).\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/12/05/release-process","metadata":{"permalink":"/doc/dev-blog/2022/12/05/release-process","source":"@site/blog/2022-12-05-release-process/index.md","title":"Mithril Release Process","description":"Mithril follows a defined release process","date":"2022-12-05T00:00:00.000Z","formattedDate":"December 5, 2022","tags":[{"label":"process","permalink":"/doc/dev-blog/tags/process"}],"readingTime":3.54,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Mithril Release Process","authors":[{"name":"Mithril Team"}],"tags":["process"]},"prevItem":{"title":"Mithril Era Switch","permalink":"/doc/dev-blog/2023/03/02/era-switch-feature"},"nextItem":{"title":"Mithril environments are updated","permalink":"/doc/dev-blog/2022/10/28/updated-environments"}},"content":"### Mithril follows a defined release process\\n\\nAs the Mithril project grew and more and more SPOs became involved in testing Mithril, it became obvious we need clearer identification of artifacts running on various parts of the network. Moreover, on our road towards mainnet availability we\'ll need to strengthen our testing process in order to validate Mithril network on more realistic environments.\\n\\n### Release Process\\n\\nWe want our release process to follow some basic principles:\\n  * _Continuous Integration_: New code is integrated into the `main` codeline frequently which triggers automated build and test process.\\n  * _Continuous Deployment_: New artifacts produced by the build process are continuously deployed to a suitable _environment_ where it can be used and tested by an increasing number of parties.\\n  * _Deployment Pipeline_: The deployment process is embodied in a _pipeline_ that describes and implements all the necessary steps to release a new version of Mithril.\\n  * _Artifact Promotion_: An artifact is built _once and only once_ and is _promoted_ while travelling through the build pipeline.\\n\\nHere is a high-level picture of this process:\\n\\n[![Release Process](./img/release_process.jpg)](./img/release_process.jpg)\\n\\n* We will use a custom version based on [SemVer](https://semver.org) for all the crates, binaries and containers of the repository and for the GitHub release.\\n* We release a new distribution every 2 weeks (this duration is subject to changes as the project matures)\\n  * The released version is named after the year and its week number: **YYWW.patch** (e.g. `2250.0`).\\n  * In case of critical regressions happening in production, a patch version will be released in between \\"official\\" releases as a hotfix.\\n* A new version `YYWW.0` will have the following life cycle:\\n    * A commit `abc123` merged on `main` branch is deployed on the network named `testing-preview`.\\n    * A commit `def456` tagged with `YYWW.0-prerelease` is deployed on the network named `pre-release-preview`.\\n    * A GitHub release `YYWW.0-prerelease` is created and linked with the `YYWW.0-prerelease` tag and marked as `pre-release`.\\n    * A tag `YYWW.0-prerelease` is qualified and selected for release or rejected (and replaced by a `YYWW.1-prerelease` tag if necessary on a `fed789`).\\n    * If the tag `YYWW.0-prerelease` is selected, a new tag is created and name `YYWW.0` on the same commit `def456`.\\n    * A GitHub release `YYWW.0` is created and linked to the `YYWW.0` tag and marked as `release`.\\n    * The commit `def456` with tag `YYWW.0` is deployed to the network named `release-preprod`.\\n* The `Cargo.toml` versions of the crates are updated (if required) just before creating the `YYWW.0-prerelease` tag .\\n* The documentation website is also updated at the same time where the `next` version becomes the `current` version, leaving future updates be appended to the `next` version during the upcoming developments.\\n* In order to simplify the life of Mithril users, we have introduced a version of the `Mithril API` used between client/signer and aggregators to check if the nodes are able to communicate together (following semver and considering the versions are compatible only if they share the same minor).\\n* Our main distribution artefact is currently docker (OCI) images. We also provide more packages, eg. `.deb` packages or compiled binaries (some of them available on multiple platforms, e.g. Windows or macOS) to simplify users\' life.\\n* We also publish some of our crates on the `crates.io` registry whenever a new version is created (e.g. [`mithril-stm`](https://crates.io/crates/mithril-stm)).\\n\\n### Networks\\n\\n* We maintain different Mithril networks (eg. servers, VMs, configurations...) to which artifacts are deployed at various stages of the process:\\n  * `testing-preview`: This is an internal environment based on the `preview` cardano testnet where most of the automated tests happen. It is also used to test features as soon as they are merged on the `main` branch.\\n  * `pre-release-preview`: This is a persistent environment based on the `preview` cardano testnet. SPOs which are active on preview are welcomed to take part in the Mithril signing process and to test new `pre-release` distributions deployed there.\\n  * `release-preprod`: Another persistent environment, based on the `preprod` cardano testnet, where more SPOs are expected to join and test, updated less frequently (on actual `release` distributions).\\n  * (_LATER_) `mainnet`: Production environment where new releases are deployed once qualifed on `release-preprod`.\\n\\n### Further Reading\\n\\n* The Mithril developers have redacted an ADR [Release process and versioning](https://mithril.network/doc/adr/3/) that also describes more technically this process."},{"id":"/2022/10/28/updated-environments","metadata":{"permalink":"/doc/dev-blog/2022/10/28/updated-environments","source":"@site/blog/2022-10-28-updated-environments.md","title":"Mithril environments are updated","description":"The Mithril environments are updated","date":"2022-10-28T00:00:00.000Z","formattedDate":"October 28, 2022","tags":[{"label":"release-process","permalink":"/doc/dev-blog/tags/release-process"},{"label":"re-spin","permalink":"/doc/dev-blog/tags/re-spin"},{"label":"preview","permalink":"/doc/dev-blog/tags/preview"},{"label":"preprod","permalink":"/doc/dev-blog/tags/preprod"},{"label":"environments","permalink":"/doc/dev-blog/tags/environments"}],"readingTime":1.3,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Mithril environments are updated","authors":[{"name":"Mithril Team"}],"tags":["release-process","re-spin","preview","preprod","environments"]},"prevItem":{"title":"Mithril Release Process","permalink":"/doc/dev-blog/2022/12/05/release-process"},"nextItem":{"title":"Mithril Keys Certification","permalink":"/doc/dev-blog/2022/10/11/keys-certification-badge"}},"content":"### The Mithril environments are updated\\n\\n**PR**: `New hosted environments` [#561](https://github.com/input-output-hk/mithril/pull/561)\\n\\n**Issue**: `Setup new hosted environments for testing-preview, pre-release-preview and release-preprod) with their terraform and GitHub environments` [#542](https://github.com/input-output-hk/mithril/issues/542)\\n\\nOn Tuesday, November 1st, 2022 the `preview` Cardano network will be re-spun and will be unavailable for 48h.\\n\\nIn the mean time, the Mitril team is also implementing a new Release Process that will make use of several new environments.\\n\\nThe Mithril testing environments are thus evolving in this context:\\n\\n- The current testing environment that runs on `preview` network and that most of the Pioneer SPOs are running is **deprecated** and will be decommissioned just after the `preview` network re-spin.\\n\\n- This environment will then be replaced by a new `pre-release-preview` environment open to SPOs that are eager to test pre releases of the Mithril nodes.\\n\\n- A new `release-preprod` environment has been launched on the `preprod` Cardano nework and will become the `stable` environment on which SPOs are encouraged to run their nodes.\\n\\n- :warning: The new `release-preprod` environment is in `unstable` status, therefore it is subject to re-genesis. We expect it to be in `stable` status within 1-2 weeks.\\n\\nIn the future, when Mithril reaches `mainnet`, we assume that the `release-preprod` will be replaced by a `release-mainnet` environment. This means that we will have the following environments at this time: `testing-preview`, `pre-release-preprod` and `release-mainnet`.\\n\\nMore information about:\\n\\n- The `Mithril Networks` and their availability [here](https://mithril.network/doc/manual/developer-docs/references#mithril-networks).\\n\\n- The `Release Process` is available in this [ADR](https://mithril.network/doc/adr/3).\\n\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/10/11/keys-certification-badge","metadata":{"permalink":"/doc/dev-blog/2022/10/11/keys-certification-badge","source":"@site/blog/2022-10-11-keys-certification-badge/index.md","title":"Mithril Keys Certification","description":"Update 2022/12/19: The signer registration with declarative PoolId has been decommissioned.","date":"2022-10-11T00:00:00.000Z","formattedDate":"October 11, 2022","tags":[{"label":"cardano","permalink":"/doc/dev-blog/tags/cardano"},{"label":"poolId","permalink":"/doc/dev-blog/tags/pool-id"},{"label":"operational-certificate","permalink":"/doc/dev-blog/tags/operational-certificate"},{"label":"kes-keys","permalink":"/doc/dev-blog/tags/kes-keys"},{"label":"mithril-keys","permalink":"/doc/dev-blog/tags/mithril-keys"},{"label":"hybrid-mode","permalink":"/doc/dev-blog/tags/hybrid-mode"}],"readingTime":2.39,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Mithril Keys Certification","authors":[{"name":"Mithril Team"}],"tags":["cardano","poolId","operational-certificate","kes-keys","mithril-keys","hybrid-mode"]},"prevItem":{"title":"Mithril environments are updated","permalink":"/doc/dev-blog/2022/10/28/updated-environments"},"nextItem":{"title":"Mithril internal stores switch to SQLite.","permalink":"/doc/dev-blog/2022/09/14/sqlite-stores"}},"content":"**Update 2022/12/19**: The signer registration with **declarative** PoolId has been decommissioned.\\n\\n**Update 2022/11/30**: The signer registration with **declarative** PoolId has been deprecated and the **certified** PoolId is now the stable mode.\\n\\n### The way the Mithril nodes handle the Certification of the SPOs is evolving\\n\\n**PR**: `New STM registration procedure` [#433](https://github.com/input-output-hk/mithril/pull/433)\\n\\n**Issues**: `Implement Certification of the Mithril Verification Keys in Signer/Aggregator` [#455](https://github.com/input-output-hk/mithril/issues/455)\\n\\nWe have released a new Mithril Signer Verification Keys Certification mechanism:\\n\\n- Mithril Signer nodes running the previous version are still able to interact with the network without any further intervention\\n- Mithril Signer nodes that are updated from a previous version must migrate some of their stores\\n- This mechanism is **experimental** and can be activated on demand by the SPOs\\n\\n#### Upgrade a Mithril Signer running a previous version\\n\\nThe SPOs need to recompile their Signer node (as in this [guide](https://mithril.network/doc/manual/getting-started/run-signer-node)).\\n\\nThe data stores of the node need to be updated by running the following command:\\n\\n```bash\\n# The path to your data stores directory, which defaults to:\\nDATA_STORES_DIRECTORY=/opt/mithril/mithril-signer/stores\\n\\n# Run this command to upgrade your stores:\\nsqlite3 ${DATA_STORES_DIRECTORY}/signer.sqlite3 \\"UPDATE protocol_initializer SET value = json_object(\'stm_initializer\', json(value), \'kes_signature\', null) WHERE json_extract(value, \'$.stm_initializer\') IS NULL;\\"\\n```\\n\\n:warning: If you don\'t update your data stores with this procedure, your node will not be able to register to the Aggregator temporarily. It should then take up to `3` epochs before it is able to successfully register its individual signatures with the Aggregator.\\n\\n#### Hybrid Certification mode in the Mithril network\\n\\nFrom now, SPOs can either run their node by:\\n\\n- **Declaring their Cardano `PoolId`**:\\n\\n  - This is the mode that all nodes were running prior to this release\\n  - This mode is still the **stable** mode\\n  - We intend to deprecate this mode in the near future\\n\\n- **Certifying their Cardano `PoolId`**:\\n\\n  - The certification is done by providing the Mithril Signer node with `KES Secret Key Path` and `Operational Certificate Path`\\n  - This is an **experimental** mode\\n  - We intend to make this mode the only way of providing a `PoolId` in the near future\\n  - These `PoolIds` will be marked with a `Verified Signer` green badge in the [Mithril Explorer](https://mithril.network/explorer/) (`2` epochs after activating the Certification mode)\\n\\nThe setup of a Mithril Signer node with these two modes is available in this [guide](https://mithril.network/doc/manual/getting-started/run-signer-node).\\n\\nHere is an example of the `Verified Signer` badge displayed in the Certificate details popin:\\n![Verified Signer Badge](./img/badge.png)\\n\\n#### How Keys Certification works\\n\\nWe rely on the Cardano `KES Keys` and `Operational Certificate` to be able to:\\n\\n- Compute automatically the `PoolId` from a valid `Operational Certificate`\\n- Sign the `Mithril Signer Verification Key` with the `KES Secret Key`\\n- Verify that the `Mithril Signer Verification Key` is associated to the owner of the pool\\n\\n![Keys Certification Schema](./img/schema.jpg)\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/09/14/sqlite-stores","metadata":{"permalink":"/doc/dev-blog/2022/09/14/sqlite-stores","source":"@site/blog/2022-09-14-sqlite-stores.md","title":"Mithril internal stores switch to SQLite.","description":"What is that?","date":"2022-09-14T00:00:00.000Z","formattedDate":"September 14, 2022","tags":[{"label":"store","permalink":"/doc/dev-blog/tags/store"},{"label":"sqlite","permalink":"/doc/dev-blog/tags/sqlite"},{"label":"breaking-change","permalink":"/doc/dev-blog/tags/breaking-change"}],"readingTime":3.005,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Mithril internal stores switch to SQLite.","authors":[{"name":"Mithril Team"}],"tags":["store","sqlite","breaking-change"]},"prevItem":{"title":"Mithril Keys Certification","permalink":"/doc/dev-blog/2022/10/11/keys-certification-badge"},"nextItem":{"title":"Stake Distribution retrieval fixed","permalink":"/doc/dev-blog/2022/09/13/stake-distribution-retrieval"}},"content":"## What is that?\\n\\nSince almost the beginning of the Mithril project, the software used to rely on a store mechanism to save its different states allowing Signers and Aggregators to resume on correct state when switched on and off. This internal store mechanism used to be a bunch of JSON files saved in a given directory. Even though this does the job it still presents flaws: data are hard to query when debugging especially when crossing data (which signers have participated in this multi-signature?). Also, data are stored in different places which can be a problem when moving these files from one place to another. We also had to imagine what would be a migration scenario in case of a structure change. Switching to a file based SQL database solves these issues.\\n\\nThe new release now uses SQLite stores in place of JSON file storage. This means that to continue running a Signer or an Aggregator node it is necessary to migrate from the old storage system to SQLite. This release comes with a tool to perform the migration which should be as straightforward as launching a command line (read below). The migration tool will be available only for a limited time in order to make Mithril beta testers able to migrate their existing data.\\n\\n## How to migrate data from old storage system to SQLite stores?\\n\\nThere are 2 ways of getting the new version and the associated migration tool. Either downloading binaries from GitHub or compiling them yourself.\\n\\n### Downloading\\n\\nDownload the new `mithril-signer` and `mithril-signer-migrate` files from the [nightly builds page](https://github.com/input-output-hk/mithril/releases/tag/nightly). Make them executable:\\n\\n```\\n$> chmod +x mithril-signer*\\n$> ls -1F mithril-signer*\\nmithril-signer*\\nmithril-signer-migrate*\\n```\\n\\n_note_: the suffix `*` appended to the the entries output above indicates the file is executable. If it is not present, ensure the `chmod` command does not produce any error.\\n\\n### Compiling\\n\\nIf you used to compile your node as stated in the [guide](https://mithril.network/doc/manual/getting-started/run-signer-node), you have to compile the migration tool as well:\\n\\n```\\n$> cd mithril-signer\\n$> cargo build --all-targets --release\\n  Compiling mithril-signer v0.1.0 (/home/somebody/shared/mithril/mithril-signer)\\n    Finished release [optimized] target(s) in 4.56s\\n$> ls -1F ../target/release/mithril-signer*\\n../target/release/mithril-signer*\\n../target/release/mithril-signer.d\\n../target/release/mithril-signer-migrate*\\n../target/release/mithril-signer-migrate.d\\n```\\n\\n### Running the migration\\n\\nThe first step is to stop the running Mithril node if any. The `mithril-signer-migrate` executable can perform the migration automatically once you know where your actual JSON files are located. Have a look in your configuration file (default `/opt/mithril/mithril-signer/service.env`), check the value associated with the `DATA_STORES_DIRECTORY` key (default to `/opt/mithril/mithril-signer/stores`) and copy the path indicated here. Copy this path after the `--db-dir` option on the following command line:\\n\\n```\\n$> ./mithril-signer-migrate automatic --db-dir /paste/the/data/stores/directory/here\\nMithril Aggregator JSON \u2192 SQLite migration tool.\\nMigrating protocol_initializer_store data\u2026\\nOK \u2713\\nMigrating stake_store data\u2026\\nOK \u2713\\n```\\n\\nAt the end of this command, a file `signer.sqlite3` (or `aggregator.sqlite3` if you run an Aggregator) should be present in the specified base directory. \\n\\nThat should be enough, launch your upgraded mithril node.\\n\\n**Note:** The migration executable does not remove the old JSON files from the disk. \\n\\n### Manual migration process\\n\\nThe executable also provides a `manual` switch for migrating Mithril JSON store directories placed in custom directories. This is mainly intended for developers who work on tweaked environments. Each internal store has its own data structure. In order to correctly migrate and process data, the type of the store has to be given on the command line.\\n\\n```\\n$> ./mithril-signer-migrate manual --help\\n```\\n\\nThe command above should give you all informations needed to run a custom store migration. \\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/09/13/stake-distribution-retrieval","metadata":{"permalink":"/doc/dev-blog/2022/09/13/stake-distribution-retrieval","source":"@site/blog/2022-09-13-stake-distribution-retrieval.md","title":"Stake Distribution retrieval fixed","description":"The way the Mithril nodes retrieve the Stake Distribution is changing","date":"2022-09-13T00:00:00.000Z","formattedDate":"September 13, 2022","tags":[{"label":"stake-distribution","permalink":"/doc/dev-blog/tags/stake-distribution"},{"label":"certificate","permalink":"/doc/dev-blog/tags/certificate"}],"readingTime":1.64,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Stake Distribution retrieval fixed","authors":[{"name":"Mithril Team"}],"tags":["stake-distribution","certificate"]},"prevItem":{"title":"Mithril internal stores switch to SQLite.","permalink":"/doc/dev-blog/2022/09/14/sqlite-stores"},"nextItem":{"title":"Signers list computation in Certificates","permalink":"/doc/dev-blog/2022/09/12/certificate-signers-list"}},"content":"### The way the Mithril nodes retrieve the Stake Distribution is changing\\n\\n**PR**: `Fix Stake Distribution retrieval` [#499](https://github.com/input-output-hk/mithril/pull/499)\\n\\n**Issue**: `Stake distribution discrepancy` [#497](https://github.com/input-output-hk/mithril/issues/497)\\n\\nWe have noticed that the way the Mithril nodes computed the `Stake Distribution` was erroneous: the epoch that was used to make the computation was the **current epoch** instead of the **previous epoch**. This has lead to some de-synchronization between the Signers and the hosted GCP Aggregator for a few epochs.\\n\\nIndeed, the `Stake Distribution` retrieved from the Cardano node depended on the time at which it was done: the nodes where having differents values that prevented them from being able to work together to produce valid multi-signatures. The problem is related to the epoch that is used (**current epoch**) to make the computation of the `Stake Distribution` when the `cardano-cli query stake-distribution` command is ran, whereas the Mithril protocol needs to work with the **previous epoch**.\\n\\nA workaround is being implemented in this fix that will compute differently the `Stake Distribution` to target the **previous epoch**. To do so, the Stake value that is now retrieved sequentially for each pool available in the `cardano-cli query stake-distribution` by using the command `cardano-cli query stake-snapshot --stake-pool-id **pool-id*`. This guarantees that the `Stake Distribution` is computed deterministically on all nodes of the Mithril Network.\\n\\nWe will continue our efforts to enhance the way the `Stake Distribution` is retrieved in the future, and so that it works smoothly on the `mainnet` (where the numbers of pools is bigger `~3,000` vs `~100` on the `preview` network).\\n\\nThe SPOs need to recompile their Signer node in order to compute correctly the `Stake Distributions` on their node (as in this [guide](https://mithril.network/doc/manual/getting-started/run-signer-node)).\\nIt should then take up to `2` epochs before they are able to successfully register their individual signatures with the Aggregator.\\n\\nMore information about the `Certificate Chain` and the epochs retrieval requirements is available [here](https://mithril.network/doc/mithril/mithril-protocol/certificates).\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/09/12/certificate-signers-list","metadata":{"permalink":"/doc/dev-blog/2022/09/12/certificate-signers-list","source":"@site/blog/2022-09-12-certificate-signers-list.md","title":"Signers list computation in Certificates","description":"The way the Signers list is computed inside a Certificate on the Mithril Aggregator is changing","date":"2022-09-12T00:00:00.000Z","formattedDate":"September 12, 2022","tags":[{"label":"certificate","permalink":"/doc/dev-blog/tags/certificate"}],"readingTime":0.825,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Signers list computation in Certificates","authors":[{"name":"Mithril Team"}],"tags":["certificate"]},"prevItem":{"title":"Stake Distribution retrieval fixed","permalink":"/doc/dev-blog/2022/09/13/stake-distribution-retrieval"},"nextItem":{"title":"Genesis Certificate support added","permalink":"/doc/dev-blog/2022/09/07/genesis-certificate-feature"}},"content":"### The way the Signers list is computed inside a Certificate on the Mithril Aggregator is changing\\n\\n**PR**: `Implement filtered Signers in Certificate` [#494](https://github.com/input-output-hk/mithril/pull/494)\\n\\n**Issue**: `Record \'contributing\' Signers only in Certificate` [#495](https://github.com/input-output-hk/mithril/issues/495)\\n\\nBefore this change, the list of Signers displayed in the `Certificate` detail of the [Mithril Explorer](https://mithril.network/explorer/) was the list of **all eligible** Signers of the epoch used for signing (those who have successfully registered with the Mithril Aggregator `2` epochs earlier).\\n\\nNow that this change has been merged, the list of Signers displayed will only include the **contributing** Signers, which means only those who have successfully sent individual signatures.\\n\\nNote that the already existing `Certificates` will not be updated as this would break the `Certificate Chain` and therefore would involve the bootstraping of a new `Genesis Certificate`.\\n\\nThis change is transparent to the Signer nodes runned by the SPOs and does not require any specific action from them.\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."},{"id":"/2022/09/07/genesis-certificate-feature","metadata":{"permalink":"/doc/dev-blog/2022/09/07/genesis-certificate-feature","source":"@site/blog/2022-09-07-genesis-certificate-feature.md","title":"Genesis Certificate support added","description":"Update: The PR has been merged and the feature is being deployed on the GCP Mithril Aggregator.","date":"2022-09-07T00:00:00.000Z","formattedDate":"September 7, 2022","tags":[{"label":"genesis","permalink":"/doc/dev-blog/tags/genesis"},{"label":"certificate","permalink":"/doc/dev-blog/tags/certificate"},{"label":"breaking-change","permalink":"/doc/dev-blog/tags/breaking-change"}],"readingTime":1.12,"hasTruncateMarker":false,"authors":[{"name":"Mithril Team"}],"frontMatter":{"title":"Genesis Certificate support added","authors":[{"name":"Mithril Team"}],"tags":["genesis","certificate","breaking-change"]},"prevItem":{"title":"Signers list computation in Certificates","permalink":"/doc/dev-blog/2022/09/12/certificate-signers-list"}},"content":"**Update**: The PR has been merged and the feature is being deployed on the GCP Mithril Aggregator.\\n\\n### This afternoon, we plan to merge the PR that activates the Genesis Certificate feature on the GCP Mithril Aggregator\\n\\n**PR**: `Implement Real Genesis Certificate` [#438](https://github.com/input-output-hk/mithril/pull/438)\\n\\n**Issue**: `Bootstrap Certificate Chain w/ Genesis Certificate` [#364](https://github.com/input-output-hk/mithril/issues/364)\\n\\nThis will involve some manual operations that will prevent temporarily the service to be running:\\n\\n* We will have to reset the stores of the `Snapshots` and `Certificates`. This means that the [Mithril Explorer](https://mithril.network/explorer/) will display a `No snapshot available` message.\\n\\n* The Mithril Signers will have to wait until the next epoch `#30` to be able to sign. This means that we should see the first available `Snapshot` 1 hour after the epoch transition.\\n\\nThe SPOs that are currently running a Mithril Signer will have to recompile their node in order ot take advantage of the latest improvements (such as the registration of the nodes that will take few minutes instead of few hours). However, the previously compiled node will be able to contribute to signatures.\\n\\nIn order to restore a Mithril Snapshot, a Mithril Client will now need access to the `Genesis Verification Key` by adding an environment variable when running: `GENESIS_VERIFICATION_KEY=$(wget -q -O - https://raw.githubusercontent.com/input-output-hk/mithril/main/TEST_ONLY_genesis.vkey)`.\\n\\nFeel free to reach out to us on the [Discord channel](https://discord.gg/5kaErDKDRq) for questions and/or help."}]}')}}]);