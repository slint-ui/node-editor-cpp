# Copyright © SixtyFPS GmbH <info@slint.dev>
# SPDX-License-Identifier: MIT

cmake_minimum_required(VERSION 3.21)
project(node-editor LANGUAGES CXX)

# Use an installed Slint if present, otherwise fetch it (needs a Rust toolchain).
if (NOT TARGET Slint::Slint)
    find_package(Slint QUIET)
endif()
if (NOT TARGET Slint::Slint)
    include(FetchContent)
    FetchContent_Declare(
        Slint
        GIT_REPOSITORY https://github.com/slint-ui/slint.git
        GIT_TAG release/1
        SOURCE_SUBDIR api/cpp
    )
    FetchContent_MakeAvailable(Slint)
endif()

add_executable(node-editor main.cpp)
target_compile_features(node-editor PRIVATE cxx_std_20)
target_link_libraries(node-editor PRIVATE Slint::Slint)
slint_target_sources(node-editor node-editor.slint)

# On Windows, copy the Slint runtime DLL next to the binary.
if (WIN32)
    add_custom_command(
        TARGET node-editor POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
            $<TARGET_FILE:Slint::Slint> $<TARGET_FILE_DIR:node-editor>
    )
endif()
