https://internetcomputer.org/docs/current/home

https://internetcomputer.org/docs/current/developer-docs/getting-started/overview-of-icp

https://internetcomputer.org/docs/current/developer-docs/getting-started/install/


```
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

source "$HOME/.local/share/dfx/env"

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

. "$HOME/.cargo/env"

cargo --version

cargo 1.81.0 (2dbb1af80 2024-08-20)


dfx --version


cd src/
npm i
dfx deploy

dfx canister call hello_backend greet world


https://internetcomputer.org/blog/features/dfx-deploy-playground



cd src/
dfx new hello_world


cd src/hello_world/

npm i

dfx deploy --playground

dfx canister --network playground call hello_world_backend greet '("everyone")'

dfx start --clean --background

dfx deploy hello_world_backend --playground

npm --version
node --version



=====


cd src/hello_world/


dfx start hello_world_backend --clean --background


rustup target add thumbv7em-none-eabihf

dfx deploy hello_world_backend --playground


dfx canister call hello_backend greet world --playground


dfx canister create hello_backend --network playground


dfx start  --clean --background


dfx stop


cat /home/vscode/.local/share/dfx/network/local


---

https://zenn.dev/plum_tt/articles/b14250bdc1c2e3


dfx new hellorust

vscode ➜ /src $ dfx new hellorust
✔ Select a backend language: · Rust
✔ Select a frontend framework: · React
✔ Add extra features (space to select, enter to confirm) · 
Fetching manifest https://sdk.dfinity.org/manifest.json
WARN: You seem to be running an outdated version of dfx.
WARN: 
You are strongly encouraged to upgrade by running 'dfx upgrade'!



cd hellorust


To learn more before you start coding, see the documentation available online:

- Quick Start: https://internetcomputer.org/docs/current/tutorials/deploy_sample_app
- SDK Developer Tools: https://internetcomputer.org/docs/current/developer-docs/setup/install/
- Motoko Language Guide: https://internetcomputer.org/docs/current/motoko/main/about-this-guide
- Motoko Quick Reference: https://internetcomputer.org/docs/current/motoko/main/language-manual
- Rust CDK Guide: https://internetcomputer.org/docs/current/developer-docs/backend/rust/


rustup target add wasm32-unknown-unknown

dfx start  --clean --background

dfx deploy

dfx killall


```
vscode ➜ /src/hellorust (master) $ dfx deploy
Creating the "default" identity.
WARNING: The "default" identity is not stored securely. Do not use it to control a lot of cycles/ICP.
To create a more secure identity, create and use an identity that is protected by a password using the following commands:
    dfx identity new <my-secure-identity-name> # creates a password protected identity
    dfx identity use <my-secure-identity-name> # uses this identity by default

  - generating new key at /home/vscode/.config/dfx/identity/default/identity.pem
Your seed phrase: false decide merit access business tired license flag theory street loyal private company rotate bitter code message provide tennis scrub roof author upgrade multiply
This can be used to reconstruct your key in case of emergency, so write it down in a safe place.
Created the "default" identity.
Deploying all canisters.
Creating canisters...
Creating canister hellorust_backend...
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "default" is "bnz7o-iuaaa-aaaaa-qaaaa-cai"
hellorust_backend canister created with canister id: bkyz2-fmaaa-aaaaa-qaaaq-cai
Creating canister hellorust_frontend...
hellorust_frontend canister created with canister id: bd3sg-teaaa-aaaaa-qaaba-cai
Building canisters...
WARN: Cannot check for vulnerabilities in rust canisters because cargo-audit is not installed. Please run 'cargo install cargo-audit' so that vulnerabilities can be detected.
Executing: cargo build --target wasm32-unknown-unknown --release -p hellorust_backend --locked
```