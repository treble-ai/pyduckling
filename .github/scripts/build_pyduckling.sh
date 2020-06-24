
pushd .
cd $HOME
stack config set resolver ghc-8.6.5
popd

maturin develop
