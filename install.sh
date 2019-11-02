#!/bin/bash

SOURCE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

#mozjpeg
git clone --branch 'v3.3.1' --depth 1 https://github.com/mozilla/mozjpeg.git
cd mozjpeg/ || exit
mkdir build && cd build || exit
cmake -G"Unix Makefiles" ..
make && sudo make install

cd "${SOURCE}" || exit

#zopflipng
git clone --branch 'zopfli-1.0.2' --depth 1 https://github.com/google/zopfli.git
cd zopfli || exit
make libzopflipng
if [[ "$OSTYPE" == "darwin"* ]]; then
    sudo cp libzopflipng.so.1.0.2 /usr/local/lib
    sudo ln -s libzopflipng.so.1.0.2 /usr/local/lib/libzopflipng.so
    sudo ln -s libzopflipng.so.1.0.2 /usr/local/lib/libzopflipng.so.1
    sudo mkdir /usr/local/include/zopflipng
    sudo cp src/zopflipng/zopflipng_lib.h /usr/local/include/zopflipng
else
    sudo cp libzopflipng.so.1.0.2 /usr/lib
    sudo ln -s libzopflipng.so.1.0.2 /usr/lib/libzopflipng.so
    sudo ln -s libzopflipng.so.1.0.2 /usr/lib/libzopflipng.so.1
    sudo mkdir /usr/include/zopflipng
    sudo cp src/zopflipng/zopflipng_lib.h /usr/include/zopflipng
fi

cd "${SOURCE}" || exit
