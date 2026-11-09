#!/bin/sh
set -e
cd "$(dirname "$0")"
glslc triangle.vert -o triangle.vert.spv
glslc triangle.frag -o triangle.frag.spv
