cmake_minimum_required(VERSION 2.8.12)

project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)
set(CMAKE_INSTALL_RPATH_USE_LINK_PATH TRUE)

# The version number.
set(VERSION_MAJOR 0)
set(VERSION_MINOR 5)
set(VERSION_PATCH 2)

if (DEFINED VERBOSE)
    set(VERBOSE ${VERBOSE})
else ()
    set(VERBOSE 0)
endif ()

configure_file(
        "caesium/config.h.in"
        "${PROJECT_BINARY_DIR}/config.h"
)

include_directories("${PROJECT_BINARY_DIR}")

if (APPLE)
    set(MOZJPEG_DEFAULT_PATH /opt/mozjpeg)
    set(ZOPFLI_DEFAULT_PATH /usr/local)
elseif (WIN32)
    set(MOZJPEG_DEFAULT_PATH C:\\mozjpeg)
    set(ZOPFLI_DEFAULT_PATH C:\\zopfli)
else ()
    set(MOZJPEG_DEFAULT_PATH /opt/mozjpeg)
    set(ZOPFLI_DEFAULT_PATH /usr)
endif ()


if (NOT DEFINED MOZJPEG_PATH)
    set(MOZJPEG_PATH ${MOZJPEG_DEFAULT_PATH})
endif()

if (NOT DEFINED ZOPFLI_PATH)
    set(ZOPFLI_PATH ${ZOPFLI_DEFAULT_PATH})
endif()

include_directories(${MOZJPEG_PATH}/include)
include_directories(${ZOPFLI_PATH}/include)

if (EXISTS ${MOZJPEG_PATH}/lib64)
    link_directories(${MOZJPEG_PATH}/lib64)
else ()
    link_directories(${MOZJPEG_PATH}/lib)
endif ()
link_directories(${ZOPFLI_PATH}/lib)

add_subdirectory(caesium)
add_subdirectory(demo)


