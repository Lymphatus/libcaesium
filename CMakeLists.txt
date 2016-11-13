cmake_minimum_required(VERSION 2.8.12)

macro(use_c99)
    if (CMAKE_VERSION VERSION_LESS "3.1")
        if (CMAKE_C_COMPILER_ID STREQUAL "GNU")
            set(CMAKE_C_FLAGS "--std=gnu99 ${CMAKE_C_FLAGS}")
        endif ()
    else ()
        set(CMAKE_C_STANDARD 99)
    endif ()
endmacro(use_c99)

project(libcaesium)

set(CMAKE_MACOSX_RPATH 1)

#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

#set(SOURCE_FILES demo/main.cpp)
#add_executable(libcaesium ${SOURCE_FILES})

add_subdirectory(caesium)
add_subdirectory(demo)