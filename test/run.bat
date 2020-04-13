@echo off
clang %1.c -o exes\%1.exe 2> logs\clang.log && exes\%1.exe