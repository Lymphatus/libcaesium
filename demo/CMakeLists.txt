add_executable(caesiumd main.c helper.c)

target_link_libraries(caesiumd LINK_PUBLIC caesium)