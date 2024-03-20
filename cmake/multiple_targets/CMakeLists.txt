cmake_minimum_required(VERSION 3.14)
project(multiple_targets)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(mymath)

add_executable(adder
	add_main.cc
)

target_link_libraries(adder PUBLIC mymath)

add_executable(subber
	sub_main.cc
)
target_link_libraries(subber PUBLIC mymath)
