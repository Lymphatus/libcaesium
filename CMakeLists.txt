cmake_minimum_required(VERSION 2.8.12)
project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)

#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

#set(SOURCE_FILES demo/main.cpp)
#add_executable(libcaesium ${SOURCE_FILES})

add_subdirectory(caesium)
add_subdirectory(demo)