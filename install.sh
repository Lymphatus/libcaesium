#!/bin/sh

SOURCE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

#mozjpeg
git clone https://github.com/mozilla/mozjpeg
cd mozjpeg/
autoreconf -fiv
autoreconf -fiv #It's not a typo, trust me
mkdir build && cd build
../configure
make && sudo make install

cd $SOURCE

#zopflipng
git clone https://github.com/google/zopfli.git
cd zopfli
make libzopflipng
sudo cp libzopflipng.so.1.0.0 /usr/lib
sudo ln -s libzopflipng.so.1.0.0 /usr/lib/libzopflipng.so
sudo ln -s libzopflipng.so.1.0.0 /usr/lib/libzopflipng.so.1
sudo mkdir /usr/include/zopflipng
sudo cp src/zopflipng/zopflipng_lib.h /usr/include/zopflipng

cd $SOURCE
