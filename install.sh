#!/bin/sh

#mozjpeg
git clone https://github.com/mozilla/mozjpeg
cd mozjpeg/
autoreconf -fiv
autoreconf -fiv
mkdir build && cd build
../configure
make && sudo make install

#zopflipng
git clone https://github.com/google/zopfli.git
cd zopfli
make libzopflipng
sudo cp libzopflipng.so.1.0.0 /usr/lib
sudo ln -s libzopflipng.so.1.0.0 /usr/lib/libzopflipng.so
sudo ln -s libzopflipng.so.1.0.0 /usr/lib/libzopflipng.so.1
sudo mkdir /usr/include/zopflipng
sudo cp src/zopflipng/zopflipng_lib.h /usr/include/zopflipng
