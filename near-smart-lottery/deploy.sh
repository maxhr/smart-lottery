#!/bin/bash
set -e

./test.sh

./build.sh

near deploy --wasmFile target/wasm32-unknown-unknown/release/smart_lottery.wasm --accountId smartlottery.testnet