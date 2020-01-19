if (NOT WIN32)
    set(CMAKE_C_FLAGS "--std=gnu99 -fPIC -Wno-nullability-completeness ${CMAKE_C_FLAGS}")
else ()
    set(CMAKE_C_FLAGS "--std=gnu99 ${CMAKE_C_FLAGS}")
endif ()

if (APPLE)
    find_library(zopflipng zopflipng /usr/local/lib)
    find_library(jpeg jpeg /usr/local/opt/mozjpeg/lib)
    find_library(turbojpeg turbojpeg /usr/local/opt/mozjpeg/lib)
ELSE ()
    find_library(zopflipng zopflipng /usr/lib)
    find_library(jpeg jpeg /opt/mozjpeg/lib)
    find_library(turbojpeg turbojpeg /opt/mozjpeg/lib)
ENDIF ()

add_library(caesium SHARED caesium.c error.c utils.c png.c vendor/lodepng.c jpeg.c)
add_library(caesium_static STATIC caesium.c error.c utils.c png.c vendor/lodepng.c jpeg.c)
set_target_properties(caesium_static PROPERTIES OUTPUT_NAME "caesium")

target_link_libraries(caesium jpeg turbojpeg zopflipng)
target_link_libraries(caesium_static jpeg turbojpeg zopflipng)

# Make sure the compiler can find include files for our Caesium library
# when other libraries or executables link to Caesium
target_include_directories(caesium PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_include_directories(caesium_static PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(FILES caesium.h DESTINATION ${CMAKE_INSTALL_PREFIX}/include)
install(TARGETS caesium
        LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/lib)
install(TARGETS caesium_static
        ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/lib)