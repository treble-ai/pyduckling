
# Set LD_LIBRARY_PATH in order to load dynamic libraries
GHC_PATH=$(stack exec -- ghc --print-libdir)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$GHC_PATH/rts:$(pwd)/ext_lib

pytest -x -v --cov=duckling duckling/tests
