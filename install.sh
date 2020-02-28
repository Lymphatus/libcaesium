#!/bin/bash

SOURCE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

rm -rf ./mozjpeg
rm -rf ./zopfli

#mozjpeg
git clone https://github.com/mozilla/mozjpeg.git
cd mozjpeg/ || exit
git checkout 426de82d0c081c996c23b75fed05833b6627b590
mkdir build && cd build || exit
cmake -G"Unix Makefiles" ..
make && sudo make install

cd "${SOURCE}" || exit

#zopflipng
git clone --branch 'zopfli-1.0.3' --depth 1 https://github.com/google/zopfli.git
cd zopfli || exit
mkdir build && cd build || exit
cmake -D"ZOPFLI_BUILD_SHARED=ON" -D"CMAKE_INSTALL_LIBDIR=/usr/local/lib" ..
make libzopflipng && sudo make install

cd "${SOURCE}" || exit
