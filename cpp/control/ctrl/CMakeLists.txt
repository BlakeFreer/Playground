add_library(ctrl STATIC)

add_subdirectory(src)
target_include_directories(ctrl PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_subdirectory(third-party/eigen)
target_link_libraries(ctrl PUBLIC Eigen3::Eigen)