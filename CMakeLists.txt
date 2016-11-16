cmake_minimum_required(VERSION 2.8.12)

project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)

set(MOZJPEG_INCLUDE /opt/mozjpeg/include)
set(MOZJPEG_LIB32 /opt/mozjpeg/lib)
set(MOZJPEG_LIB64 /opt/mozjpeg/lib64)

include_directories(${MOZJPEG_INCLUDE})

if(EXISTS ${MOZJPEG_LIB64})
    link_directories(${MOZJPEG_LIB64})
else()
    link_directories(${MOZJPEG_LIB32})
endif()

add_subdirectory(caesium)
add_subdirectory(demo)


