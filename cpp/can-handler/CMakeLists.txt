cmake_minimum_required(VERSION 3.28)

enable_language(C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS ON)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES ${CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES})
set(CMAKE_CXX_EXTENSIONS ON)

# clangd export settings
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_USE_RESPONSE_FILE_FOR_INCLUDES OFF)

project(CanHandler)

add_executable(main)
add_subdirectory(src)
add_subdirectory(etl)
add_subdirectory(canlib)

target_link_libraries(main PRIVATE canlib)