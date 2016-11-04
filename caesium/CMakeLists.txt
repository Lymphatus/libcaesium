add_library(caesium caesium.c error.c utils.c)

# Make sure the compiler can find include files for our Caesium library
# when other libraries or executables link to Caesium
target_include_directories(caesium PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})