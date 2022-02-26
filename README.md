# Smart Lottery on chain

This is a mono-repo for a smart contract on multiple blockchains. It aims to compare a (kind of) minumum viable code, for an actually useful smart-contract, on different blockchains.

The contracts implement a lottery logic with these features:
- A user can deposit tokens into the contract (`play_lottery` function)
- A random number can be generated on chain (`get_random_seed` function)
- The `do_lottery` function is triggered by external events
    - A real-world-time interval
    - A cross-chain event - when a certain sum of number of players reached _across all chains_
- The `share_prize` function transfers token to winners
    - For a single-chain lottery winners chosen from one chain
    - An ambitious multi-chain lottery - tokens across multiple chains are shared between winners on those chains, with price conversions and cross-chain value transfer 😎

| Chain     | `play_lottery` | `get_random_seed` | `do_lottery` | `share_prize` |
|-----------|----------------|-------------------|--------------|---------------|
| Ethereum  |                |                   |              |               |
| Solana    | wip            |                   |              |               |
| Cardano   |                |                   |              |               |
| Polkadot  | ✓              |                   |              |               |
| Near      | ✓              |                   |              |               |
| Avalanche |                |                   |              |               |
| BSC       |                |                   |              |               |
| Harmony   |                |                   |              |               |
|           |                |                   |              |               |

