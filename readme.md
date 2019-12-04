Example command line utility for querying latest state from a substrate based chain.

This example supports querying some, but not all public storage items used in the
substrate-node-template.

Example usage:

```
# Get the balance of the Alice account
substrate-client-cli ws://127.0.0.1:9944 read balances-free-balance '"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"'
```

Input and output are encoded as json.

# How it works

Substrate state queries consist of the following steps:

- take some key K
- convert K to a raw trie index (usually by SCALE encoding then hashing)
- lookup the index in chainstate
- decode index as some value V

In short: `K -> SCALE encode -> Hash -> lookup -> SCALE decode -> V`

This example utility composes the process to allow queries from the command line.

`json decode -> K -> SCALE encode -> Hash -> lookup -> SCALE decode -> V -> json encode`

`K` must implement `serde::Deserialze` and `V` must implement `serde::Deserialze`.

# Typechecking

Traits from `srml_support::storage::hashed::generator` like `StorageMap` provide type safety.
For example try changing the return type of `SystemAccountNonce` from `u32` to something else.
("./src/query.rs"). Rustc will shout at you because node-template-runtime uses `u32` for
account nonces.

# Limits

Functionality is limited. Here are some things this tool could do, but does not. (PRs welcome)

- Extrinsic submission
- Querying state for blocks other the latest
- StorageDoubleMap queries (would be straightforward to implement, needs a test case)
- StorageLinkedMap queries (would be straightforward to implement, needs a test case)
- `#[derive(StorageQuery)]`
