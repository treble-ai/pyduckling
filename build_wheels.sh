#!/bin/bash
set -ex

# Install ZLib and sudo
yum install -y zlib-devel sudo

# Install Rustup
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
export PATH="$HOME/.cargo/bin:$PATH"

# Install Stack
curl -sSL https://get.haskellstack.org/ | sh
export PATH="$HOME/.local/bin:$PATH"

# Set stack resolver to 8.6.5
sudo chmod 777 $HOME
stack config set resolver ghc-8.6.5

# Compile patchelf and apply 64MB patch
pushd /root
git clone https://github.com/NixOS/patchelf
cd patchelf
git apply /io/packaging/0001-Allow-binaries-larger-than-32MB.patch

bash bootstrap.sh
./configure
make
make install
popd

# Compile libducklingffi
cd /io
pushd duckling-ffi

stack build
cp libducklingffi.so ../ext_lib
popd

# Produce wheels and patch binaries for redistribution
PYBIN=/opt/python/cp$(echo $PYTHON_VERSION | sed -e 's/\.//g')*/bin
# for PYBIN in /opt/python/cp{35,36,37,38,39}*/bin; do
"${PYBIN}/pip" install -U setuptools wheel setuptools-rust
"${PYBIN}/python" packaging/build_wheels.py
# done
