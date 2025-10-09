# silent-grpc

## 项目概述

silent-grpc 是 Silent Odyssey 第三阶段的核心项目，旨在基于 Silent 框架实现一个完整的 gRPC 服务端。该项目聚焦于 HTTP/2 双向流特性，采用 Protobuf 作为接口定义语言，实现跨语言互通能力。通过高并发场景下的流式通信与协议契约验证，检验 Silent 框架在复杂网络协议和多样化应用场景下的适应性与性能表现。

## 协议范围

本项目当前支持以下 gRPC 核心特性：

- Unary RPC（单次请求响应）
- Server Streaming（服务端流式响应）
- Client Streaming（客户端流式请求）
- Bidirectional Streaming（双向流式通信）

接口定义采用 Protobuf（.proto 文件），确保与主流 gRPC 生态系统兼容，实现跨语言调用。

## 架构设计

推荐目录结构示例：

```
src/
├── main.rs           # 入口，gRPC 服务器启动
├── service.rs        # gRPC 服务实现
├── proto/            # 存放 .proto 文件与生成代码
├── handler.rs        # 请求处理逻辑
└── client.rs         # 客户端测试与互通验证
```

- `main.rs` 负责启动 HTTP/2 服务并加载 gRPC 服务。
- `service.rs` 实现具体的 gRPC 服务接口。
- `proto/` 目录存放 Protobuf 定义文件及自动生成的 Rust 代码。
- `handler.rs` 包含业务请求处理逻辑。
- `client.rs` 用于测试客户端调用，验证互通性。

## 使用说明

### 启动服务端

```bash
cargo run
```

服务默认监听 `0.0.0.0:50051`，启动日志会输出当前监听地址。根路由会返回健康检查信息，其余 gRPC 方法由 `Greeter` 服务提供。

### 内置客户端验证

项目附带一个简单的命令行客户端，涵盖四种 RPC 调用方式：

```bash
cargo run -- --client
```

执行后可在终端观察 Unary、服务端流、客户端流以及双向流的示例输出。

### 使用 grpcurl

服务端已开启 gRPC Reflection，可直接使用 `grpcurl` 探测并调用接口。例如：

```bash
grpcurl -plaintext -d '{"name":"Silent"}' localhost:50051 helloworld.Greeter/SayHello
```

其他方法同理，例如：

- 服务端流式响应：

  ```bash
  grpcurl -plaintext -d '{"name":"Streaming"}' localhost:50051 helloworld.Greeter/LotsOfReplies
  ```

- 客户端流与双向流可通过 `-d @` 进入交互模式：

  ```bash
  grpcurl -plaintext -d @ localhost:50051 helloworld.Greeter/LotsOfGreetings <<'EOF'
  {"name":"Alice"}
  {"name":"Bob"}
  {"name":"Charlie"}

  EOF
  ```

## 测试与性能评测

推荐使用 `ghz` 或自定义基准测试工具对服务进行性能验证，重点关注：

- 请求每秒数（RPS）
- 延迟（Latency）
- 带宽利用率
- 流式吞吐能力

通过多维度指标评估 Silent 框架在高并发流式 RPC 场景下的表现。

## 当前功能与路线图

| 功能                    | 状态 |
| ----------------------- | ---- |
| Unary RPC               | ✅ 已完成 |
| Protobuf 编解码         | ✅ 已完成 |
| Server Streaming        | ✅ 已完成 |
| Client Streaming        | ✅ 已完成 |
| Bidirectional Streaming | ✅ 已完成 |
| gRPC Reflection         | ✅ 已完成 |
| TLS 支持                | ✅ 已完成 |
| 拦截器与中间件          | ✅ 已完成 |
| 负载均衡                | ❌ 未计划 |

我们将持续迭代完善，逐步支持更多功能。
