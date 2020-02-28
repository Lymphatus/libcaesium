if (NOT WIN32)
    set(CMAKE_C_FLAGS "--std=gnu99 -fPIC -Wno-nullability-completeness ${CMAKE_C_FLAGS}")
else ()
    set(CMAKE_C_FLAGS "--std=gnu99 ${CMAKE_C_FLAGS}")
endif ()

find_library(jpeg jpeg ${MOZJPEG_PATH}/lib)
find_library(turbojpeg turbojpeg ${MOZJPEG_PATH}/lib)
find_library(zopflipng zopflipng ${ZOPFLI_PATH}/lib)

add_library(caesium SHARED caesium.c error.c utils.c png.c vendor/lodepng.c jpeg.c)

target_link_libraries(caesium jpeg turbojpeg zopfli zopflipng)

# Make sure the compiler can find include files for our Caesium library
# when other libraries or executables link to Caesium
target_include_directories(caesium PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(FILES caesium.h DESTINATION ${CMAKE_INSTALL_PREFIX}/include)
install(TARGETS caesium
        LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/lib)
