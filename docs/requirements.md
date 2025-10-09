# 功能需求整理

## 背景

silent-grpc 需要基于 Silent 框架实现完整的 gRPC 服务端，聚焦 HTTP/2 双向流特性，并通过 Protobuf 协议实现跨语言互通。

## 核心功能

- [x] 使用 Silent 搭建 gRPC 服务器，监听默认端口 `50051`，完成服务启动与生命周期管理。
- [x] 定义 Protobuf 接口并生成对应的 Rust 代码，保证与主流 gRPC 客户端兼容。
- [x] 实现基础的问候服务（`helloworld.Greeter`），满足以下调用模式：
  - [x] Unary RPC：实现 `SayHello` 方法，接收请求返回单次响应。
  - [x] Server Streaming：实现 `LotsOfReplies` 方法，服务端返回多条消息。
  - [x] Client Streaming：实现 `LotsOfGreetings` 方法，客户端上传多条消息后返回汇总。
  - [x] Bidirectional Streaming：实现 `BidiHello` 方法，支持全双工交互。
- [x] 提供一个简单的命令行客户端示例，用于互通验证。

## 非功能需求

- 服务默认使用本地时间（`chrono::Local::now().naive_local()`）记录必要的时间字段。
- 所有 ID 使用 `scru128` 生成，保证高可用与分布式唯一性。
- 代码需通过 `cargo check`（或 `cargo clippy`）确保基本质量。
- 前端工具使用 Yarn（当前项目暂无前端模块，仅保留约束以便后续迭代）。

## 当前状态（参考 README）

| 功能                    | 状态 |
| ----------------------- | ---- |
| Unary RPC               | ✅ 已实现 |
| Protobuf 编解码         | ✅ 已实现 |
| Server Streaming        | ✅ 已实现 |
| Client Streaming        | ✅ 已实现 |
| Bidirectional Streaming | ✅ 已实现 |
| TLS 支持                | ❌ 未计划 |
| 拦截器与中间件          | ❌ 未计划 |
| 负载均衡                | ❌ 未计划 |

> 注：状态标记基于现有代码库存，需随实现进度更新。
