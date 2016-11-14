set(CMAKE_C_FLAGS "--std=gnu99 -fPIC ${CMAKE_C_FLAGS}")

find_library(zoflipng zopflipng /usr/local/lib)
find_library(jpeg jpeg /opt/mozjpeg/lib)
find_library(turbojpeg turbojpeg /opt/mozjpeg/lib)

add_library(caesium SHARED caesium.c error.c utils.c png.c lodepng.c jpeg.c)
add_library(caesium_static STATIC caesium.c error.c utils.c png.c lodepng.c jpeg.c)

set_target_properties(caesium_static PROPERTIES OUTPUT_NAME caesium)

target_link_libraries(caesium zopflipng jpeg turbojpeg)

# Make sure the compiler can find include files for our Caesium library
# when other libraries or executables link to Caesium
target_include_directories(caesium PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
