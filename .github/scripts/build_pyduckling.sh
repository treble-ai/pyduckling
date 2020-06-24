
# GHC_ROOT=$(dirname $(dirname $(stack exec which ghc)))
# GHC_LIB="$GHC_ROOT/lib/ghc-8.6.5/"
# RTS_PATH="$GHC_LIB/rts"

# export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$RTS_PATH:$(pwd)/ext_lib
ls $HOME/.stack/programs/x86_64-linux/
maturin develop
