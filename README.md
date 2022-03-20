# rdr-rs

## 关于本仓库

本仓库为 Radar Router *a.k.a.* rdr 的 Rust 实现，包含了以下内容：
 - `rdr-core`：数据结构库，包含了消息的数据结构，+使用 Protocol Buffers 实现的消息解析器。
 - `rdr-zeromq`：核心库，包装了使用 ZeroMQ 实现的消息传输。
 - `rdr-pyo3`：核心库的 Python 绑定，目标是得到比纯 Python 库更高的性能，并提供异步接口。
 - `rdr-web`：Web 端的后端，通过 WebSocket 与浏览器进行通信，本身是 rdr 的接收端。

另外，还计划用 Rust 实现以下工具：
 - `rdr-compose`：管理 rdr 集群的启动，跨语言。
 - `rdr-arpeggiator`：通过外置进程池，让一些程序能更好的利用多核。

## 交流规范

为了看得方便，commit message 和 issue 请全部使用中文，并确保遵循 [中文文案排版指北](https://github.com/sparanoid/chinese-copywriting-guidelines) 中的排版规范。想要偷懒的话可以使用 [AutoCorrect](https://github.com/huacnlee/autocorrect) 进行自动修正。
