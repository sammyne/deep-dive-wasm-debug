# hello-world

演示一个简单的 C++-WASM 组件的构建。

## 环境
- 参见项目根目录的 [Dockerfile](../../docker/Dockerfile)。

## 简介

文件 | 说明
-----|-------
guest | C++ 编写的 WASM 组件源码
host-rs | Rust 编写用于调用 guest 组件的逻辑

## 快速开始
```bash
make run
```

## 参考文献
- https://component-model.bytecodealliance.org/language-support/c.html

