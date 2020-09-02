set -ex

mkdir -p $HOME/.stack/global-project
cp .github/stack/stack.yaml $HOME/.stack/global-project

pushd $HOME
stack config set resolver ghc-8.6.5
popd

which python
conda activate test
python setup.py bdist_wheel

if [[ $PYTHON_VERSION == "3.9" ]]; then
    python setup.py sdist
fi
