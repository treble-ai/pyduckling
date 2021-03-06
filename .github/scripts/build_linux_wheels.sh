#!/bin/bash
set -ex
shopt -s nullglob

PYBIN=/opt/python/cp$(echo $PYTHON_VERSION | sed -e 's/\.//g')*/bin
echo $PYBIN
# PYBIN=$(echo $PYBIN)

# Install ZLib and sudo
yum install -y zlib-devel sudo
export HOME="/root"

# Install Rustup
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
export PATH="$HOME/.cargo/bin:$PATH"

# Install Stack
curl -sSL https://get.haskellstack.org/ | sh
export PATH="$HOME/.local/bin:$PATH"

# Set stack resolver to 8.6.5
mkdir -p $HOME/.stack/global-project
cp .github/stack/stack.yaml $HOME/.stack/global-project
cp packaging/0001-Allow-binaries-larger-than-32MB.patch $HOME

pushd $HOME
stack config set resolver ghc-8.6.5
popd

# Compile patchelf and apply 64MB patch
pushd /root
git clone https://github.com/NixOS/patchelf
cd patchelf
git apply $HOME/0001-Allow-binaries-larger-than-32MB.patch

bash bootstrap.sh
./configure
make
make install
popd

# Compile libducklingffi
pushd duckling-ffi

stack build
cp libducklingffi.so ../ext_lib
popd

# Produce wheels and patch binaries for redistribution
PYBIN=$(echo $PYBIN)
GHC_LIB=$(stack exec -- ghc --print-libdir)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$GHC_LIB/rts:$(pwd)/ext_lib
# for PYBIN in /opt/python/cp{35,36,37,38,39}*/bin; do
"${PYBIN}/pip" install -U setuptools wheel setuptools-rust auditwheel
"${PYBIN}/python" packaging/build_wheels.py
# done

if [[ $PYTHON_VERSION == "3.9" ]]; then
    "${PYBIN}/python" setup.py sdist
fi
