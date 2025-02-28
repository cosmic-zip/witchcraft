#!/bin/bash
set -e

# Install deps
echo && echo "Install dependencies"
sudo apt update > /dev/null
sudo apt install -y nmap whois dirb dnsenum libc-bin iproute2 xxd iptables coreutils wget curl \
dnsutils traceroute openssl openssh-server xattr libimage-exiftool-perl tor foremost pkg-config \
libssl-dev steghide libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev 7zip \
libayatana-appindicator3-dev librsvg2-dev chromium-browser git > /dev/null

# Install rust if not exists
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

# Install data
echo && echo "Install spellbook data"
sudo cp -r --update=none spellbook/ /var/
sudo chown -R $(whoami):$(whoami) /var/spellbook
echo 'export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/

# Wordlist
echo && echo "Wordlists and Malware sigs are too big 700Mb so, if you want or need then, just run:"
echo "git clone https://github.com/cosmic-zip/witchcraft-wordlists /var/spellbook/"

# Build binary
echo && echo "Cargo build"
cargo build --release --manifest-path witchcraft/Cargo.toml
chmod +x ./witchcraft/target/release/witchcraft
sudo cp -r ./witchcraft/target/release/witchcraft /bin/witchcraft

echo && echo "Build dist packages"
rm -rf ./dist
mkdir ./dist
mkdir ./dist/lite
mkdir ./dist/full

echo && echo "Creating the installer"
cp spellbook/archive/scripts/installer.sh dist/lite
cp spellbook/archive/scripts/uninstall.sh dist/lite

cp spellbook/archive/scripts/installer.sh dist/full
cp spellbook/archive/scripts/uninstall.sh dist/full

cp -r ./witchcraft/target/release/witchcraft ./dist/lite
cp -r ./witchcraft/target/release/witchcraft ./dist/full

cp -r spellbook ./dist/lite
cp hidden/spellbook.tar.xz ./dist/full

zip -r witchcraft_lite.zip dist/lite/ > /dev/null
zip -r witchcraft_full.zip dist/full/ > /dev/null

rm -r dist/lite
rm -r dist/full

mv witchcraft_lite.zip dist/
mv witchcraft_full.zip dist/


echo && echo "Done!"
