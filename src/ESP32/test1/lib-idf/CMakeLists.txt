cmake_minimum_required(VERSION 3.5)

# Override to using the python virtual env
set(PYTHON "${CMAKE_CURRENT_BINARY_DIR}/../pyvirtenv/pyenv/Scripts/python")

# Include idf cmake file
include($ENV{IDF_PATH}/tools/cmake/project.cmake)

# Project binary
project(idf_lib)
