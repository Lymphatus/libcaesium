add_executable(caesiumd main.c)

target_link_libraries(caesiumd LINK_PUBLIC caesium)

install(TARGETS caesiumd DESTINATION bin)