if (MSVC)
    add_compile_options( /constexpr:steps5000000 /analyze:stacksize370000 )
endif()

add_executable(day22_2018 main.cpp)
set_property(TARGET day22_2018 PROPERTY VS_DEBUGGER_WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")
