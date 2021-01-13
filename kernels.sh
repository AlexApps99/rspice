#!/bin/sh

cd "$(dirname $0)"
xargs -n 1 -a kernels.txt curl -s --output-dir kernels/ -O
