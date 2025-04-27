#!/bin/bash

set -e

cd `dirname ${BASH_SOURCE[0]}`

repo_tag=sammyne/wasm-lldb:alpha

build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MAJOR_VERSION=22"
build_arg_opts="$build_arg_opts --build-arg WASI_SDK_MINOR_VERSION=0"
build_arg_opts="$build_arg_opts --build-arg WASMTIME_VERSION=32.0.0"
build_arg_opts="$build_arg_opts --build-arg WASM_TOOLS_VERSION=1.229.0"
build_arg_opts="$build_arg_opts --build-arg WIT_BINDGEN_VERSION=0.27.0"

docker build $build_arg_opts -t $repo_tag .
