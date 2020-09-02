
export PATH="$HOME/.local/bin:$PATH"
mkdir -p $HOME/.stack/global-project
cp .github/stack/stack.yaml $HOME/.stack/global-project

pushd .
cd $HOME
stack config set resolver ghc-8.6.5
popd

maturin develop
