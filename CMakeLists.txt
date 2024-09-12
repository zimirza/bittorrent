cmake_minimum_required(VERSION 3.10)

project(bt)

find_package(CURL REQUIRED)

add_executable(bt src/main.c)

target_link_libraries(bt CURL::libcurl)