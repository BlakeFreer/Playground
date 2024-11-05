cmake_minimum_required(VERSION 3.28)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_USE_RESPONSE_FILE_FOR_INCLUDES OFF)

SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic -Wunused")

project(main)

add_executable(main main.cpp)

add_subdirectory(third-party/eigen)
target_link_libraries(main Eigen3::Eigen)