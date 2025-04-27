FROM rust:1.85.0-bookworm AS rust

ENV RUSTUP_DIST_SERVER="https://rsproxy.cn" \
  RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN sed -i 's/deb.debian.org/mirrors.tencent.com/g' /etc/apt/sources.list.d/debian.sources  &&\
  sed -i 's|security.debian.org/debian-security|mirrors.tencent.com/debian-security|g'      \
    /etc/apt/sources.list.d/debian.sources

RUN apt update && apt install -y wget

# 下载并添加 GPG 密钥
# 添加 LLVM 18 的 APT 源（Debian 12 代号为 bookworm）
RUN wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | tee /etc/apt/trusted.gpg.d/llvm.asc &&\
  echo "deb http://apt.llvm.org/bookworm/ llvm-toolchain-bookworm-18 main" | tee /etc/apt/sources.list.d/llvm-18.list

# libssl-dev 和 pkg-config 是 cargo-component 的依赖
# clang 是 mold 链接器的依赖
RUN apt update                                          &&\
  apt install -y clang-18 lldb-18 libssl-dev pkg-config &&\
  apt install -y git vim make                           &&\
  apt clean                                             &&\
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN ln -sf `which clang-18` /usr/bin/clang && ln -sf `which lldb-18` /usr/bin/lldb 

RUN rustup component add rustfmt && rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2


FROM busybox:1.36.1 AS wasi-sdk

ARG WASI_SDK_MAJOR_VERSION

ARG WASI_SDK_MINOR_VERSION

WORKDIR /workspace

RUN wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-$WASI_SDK_MAJOR_VERSION/wasi-sdk-$WASI_SDK_MAJOR_VERSION.$WASI_SDK_MINOR_VERSION-linux.tar.gz

RUN tar xvf wasi-sdk-$WASI_SDK_MAJOR_VERSION.$WASI_SDK_MINOR_VERSION-linux.tar.gz

RUN mv wasi-sdk-$WASI_SDK_MAJOR_VERSION.$WASI_SDK_MINOR_VERSION /output


FROM busybox:1.36.1 AS wasm-tools

ARG WASM_TOOLS_VERSION

ENV REPO_URL=https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}

RUN wget $REPO_URL/wasm-tools-$WASM_TOOLS_VERSION-x86_64-linux.tar.gz

RUN mkdir -p /output && tar -C /output --strip-components 1 -zxvf wasm-tools-$WASM_TOOLS_VERSION-x86_64-linux.tar.gz


FROM busybox:1.36.1 AS wasmtime

ARG WASMTIME_VERSION

ENV REPO_URL=https://github.com/bytecodealliance/wasmtime/releases/download/v$WASMTIME_VERSION

RUN wget $REPO_URL/wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz

RUN mkdir -p /output && tar -C /output --strip-components 1 -Jxvf wasmtime-v$WASMTIME_VERSION-x86_64-linux.tar.xz

WORKDIR /output/adapter-modules

RUN wget $REPO_URL/wasi_snapshot_preview1.command.wasm  &&\
  wget $REPO_URL/wasi_snapshot_preview1.proxy.wasm      &&\
  wget $REPO_URL/wasi_snapshot_preview1.reactor.wasm


FROM busybox:1.36.1 AS wit-bindgen

ARG WIT_BINDGEN_VERSION

ENV REPO_URL=https://github.com/bytecodealliance/wit-bindgen/releases/download/v$WIT_BINDGEN_VERSION

RUN wget $REPO_URL/wit-bindgen-$WIT_BINDGEN_VERSION-x86_64-linux.tar.gz

RUN mkdir -p /output && tar -C /output --strip-components 1 -zxvf wit-bindgen-$WIT_BINDGEN_VERSION-x86_64-linux.tar.gz


FROM busybox:1.36.1 AS rc

WORKDIR /output

COPY --from=wasi-sdk /output wasi-sdk

COPY --from=wasm-tools /output wasm-tools

COPY --from=wasmtime /output wasmtime

COPY --from=wit-bindgen /output wit-bindgen


FROM rust

ENV PATH=/opt/wasmtime:/opt/wasm-tools:/opt/wit-bindgen:$PATH

COPY --from=rc /output /opt
