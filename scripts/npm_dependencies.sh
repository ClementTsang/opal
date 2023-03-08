#!/bin/sh

set -eux

echo "Installing tailwindcss..."
npm install -g tailwindcss@3.2.4

echo "Installing binaryen..."
npm install -g binaryen@111.0.0;

echo "Installing minify..."
npm install -g minify@9.2.0;
