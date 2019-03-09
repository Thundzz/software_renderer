#!/bin/bash


mkdir -p target/package/

rm -rf target/package/*
rm -rf target/starfield.tar.gz

cp target/debug/software_rendering.exe target/package/
cp msvc/dll/64/SDL2.dll target/package/

cd target/package/
tar cvzf ../starfield.tar.gz *