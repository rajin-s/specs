@echo off
cargo run 1> logs\out.log.sp 2> logs\err.log -- in\%1.sp out\%1.c