
# Set LD_LIBRARY_PATH in order to load dynamic libraries
GHC_PATH=$(stack exec -- ghc --print-libdir)
GHC_VERSION=$(stack exec -- ghc --numeric-version)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$GHC_PATH/lib/ghc-${GHC_VERSION}/rts:$(pwd)/ext_lib

pytest -x -v --cov=duckling duckling/tests
