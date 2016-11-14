cmake_minimum_required(VERSION 2.8.12)

project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)
set(MOZJPEG_INCLUDE /opt/mozjpeg/include)
set(MOZJPEG_LIB32 /opt/mozjpeg/lib)
set(MOZJPEG_LIB64 /opt/mozjpeg/lib64)

include_directories(${MOZJPEG_INCLUDE})
link_directories(${MOZJPEG_LIB32} ${MOZJPEG_LIB64})

add_subdirectory(caesium)
add_subdirectory(demo)
