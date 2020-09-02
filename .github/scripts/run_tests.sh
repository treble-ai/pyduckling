
# Set LD_LIBRARY_PATH in order to load dynamic libraries
GHC_PATH=$(stack exec -- ghc --print-libdir)


if [[ "$(uname)" != Darwin ]]; then
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$GHC_PATH/rts:$(pwd)/ext_lib
else
    export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$GHC_PATH/rts:$(pwd)/ext_lib
fi
pytest -x -v --cov=duckling duckling/tests
