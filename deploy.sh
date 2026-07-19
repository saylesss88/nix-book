#!/usr/bin/env bash
set -e

mdbook build
cp -r book/* ~/saylesss88.github.io/
cd ~/saylesss88.github.io
jj desc -m "deploy"
jj mb
jj git push
