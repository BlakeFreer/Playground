file(GLOB EXECUTABLES RELATIVE ${CMAKE_CURRENT_SOURCE_DIR} *.cpp)

foreach(sourcefile ${EXECUTABLES})
    get_filename_component(name ${sourcefile} NAME_WE)
    add_executable(${name} ${sourcefile})
    target_link_libraries(${name} PRIVATE ctrl)
endforeach(sourcefile ${EXECUTABLES})
