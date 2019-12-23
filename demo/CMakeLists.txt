if (NOT WIN32)
    set(CMAKE_C_FLAGS "--std=gnu99 -fPIC -Wno-nullability-completeness ${CMAKE_C_FLAGS}")
else ()
    set(CMAKE_C_FLAGS "--std=gnu99 ${CMAKE_C_FLAGS}")
endif ()

add_executable(caesiumd main.c)

target_link_libraries(caesiumd LINK_PUBLIC caesium)