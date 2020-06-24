
GHC_ROOT=$(dirname $(dirname $(stack exec which ghc)))
GHC_LIB="$GHC_ROOT/lib/ghc-8.6.5/"
RTS_PATH="$GHC_LIB/rts"

LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$RTS_PATH:$(pwd)/ext_lib
maturin develop
