cmake_minimum_required(VERSION 2.8.12)

project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)
set(MOZJPEG_INCLUDE /opt/mozjpeg/include)
set(MOZJPEG_LIB /opt/mozjpeg/lib64)

include_directories(${MOZJPEG_INCLUDE})
link_directories(${MOZJPEG_LIB})

set(CMAKE_C_FLAGS "--std=gnu99 -fPIC ${CMAKE_C_FLAGS}")

add_subdirectory(caesium)
add_subdirectory(demo)
