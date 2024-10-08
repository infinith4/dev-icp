
### install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash

#### install node
. $HOME/.nvm/nvm.sh && \
    nvm install --lts && \
    nvm use --lts && \
    node -v && npm -v && \
    npm i -g yarn
npm install -g npm@10.9.0
nvm use --lts

sudo apt-get update
sudo apt-get install expect -y


bash ./postCommandDfx.sh


dfx --version


bash ./postCommandRust.sh


. "$HOME/.cargo/env"

cargo --version




exec bash