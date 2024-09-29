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

