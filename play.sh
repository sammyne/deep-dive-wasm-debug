#!/bin/bash

repo_tag=sammyne/wasm-lldb:alpha

workdir=/workspace

cargo_home=/root/xml/workspace/rust/cargo

docker run -it --rm                     \
        -e CARGO_HOME=/root/.cargo      \
        -v $PWD:$workdir                \
        -v $cargo_home/registry:/root/.cargo/registry   \
        -v $cargo_home/git:/root/.cargo/git             \
        -v /root/xml/.ssh:/root/.ssh                    \
        -w $workdir                     \
        $repo_tag bash