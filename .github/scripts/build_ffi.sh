
# Install Stack
# curl -sSL https://get.haskellstack.org/ | sh
# export PATH="$HOME/.local/bin:$PATH"

cd duckling-ffi
stack build
cp libducklingffi.so ../ext_lib
