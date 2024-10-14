


```
dfx new basic-rust-react
```

rust, reactを選択する

cd samples/basic-rust-react
rustup target add wasm32-unknown-unknown


dfx start  --clean --background

dfx deploy

dfx stop
dfx killall