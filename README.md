# Aya Mini Firewall

一个使用 Aya eBPF 框架开发的简单防火墙演示程序，可以在 Linux 系统上运行。

## 功能特性

- 基于 eBPF/XDP 的高性能包过滤
- 支持按 IP 地址和端口号进行流量过滤
- 实时规则管理
- 低延迟网络包处理

## 系统要求

- Linux 内核 4.15+ (支持 eBPF/XDP)
- Rust 1.70+
- 管理员权限 (需要加载 eBPF 程序)

## 构建和安装

### 1. 安装依赖

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 eBPF 工具链
cargo install bpf-linker
```

### 2. 构建项目

```bash
# 使用构建脚本
./build.sh

# 或者手动构建
cd aya-minifirewall-ebpf
cargo build --release --target bpfel-unknown-none
cd ..
cargo build --release
```

## 使用方法

### 加载防火墙程序

```bash
# 将防火墙程序加载到指定网络接口
sudo ./target/release/aya-minifirewall load --interface eth0
```

### 管理防火墙规则

```bash
# 阻止特定 IP 地址
sudo ./target/release/aya-minifirewall block-ip 192.168.1.100

# 取消阻止 IP 地址
sudo ./target/release/aya-minifirewall unblock-ip 192.168.1.100

# 阻止特定端口
sudo ./target/release/aya-minifirewall block-port 80

# 取消阻止端口
sudo ./target/release/aya-minifirewall unblock-port 80

# 查看当前规则
sudo ./target/release/aya-minifirewall list
```

## 项目结构

```text
aya-minifirewall/
├── Cargo.toml                 # 主项目配置
├── src/
│   └── main.rs               # 用户空间程序
├── aya-minifirewall-ebpf/    # eBPF 程序目录
│   ├── Cargo.toml
│   ├── build.rs
│   └── src/
│       └── main.rs           # eBPF 内核程序
├── build.sh                  # 构建脚本
└── README.md
```

## 技术实现

### eBPF 程序 (内核空间)

- 使用 XDP (eXpress Data Path) 进行高性能包处理
- 实现 IP 地址和端口过滤逻辑
- 使用 HashMap 存储阻止规则
- 支持 TCP 和 UDP 协议

### 用户空间程序

- 使用 Aya 框架加载和管理 eBPF 程序
- 提供命令行接口进行规则管理
- 支持实时规则更新

## 注意事项

1. **权限要求**: 需要 root 权限来加载 eBPF 程序
2. **网络接口**: 确保指定的网络接口存在且支持 XDP
3. **内核版本**: 需要支持 eBPF/XDP 的现代 Linux 内核
4. **性能影响**: eBPF 程序在内核空间运行，对网络性能影响很小

## 故障排除

### 常见问题

1. **加载失败**: 检查内核是否支持 eBPF/XDP

   ```bash
   # 检查内核配置
   zcat /proc/config.gz | grep -E "(BPF|XDP)"
   ```

2. **权限错误**: 确保以 root 权限运行

   ```bash
   sudo ./target/release/aya-minifirewall load --interface eth0
   ```

3. **接口不存在**: 检查网络接口名称

   ```bash
   ip link show
   ```

## 开发说明

这是一个演示项目，展示了如何使用 Aya 框架开发 eBPF 应用程序。在生产环境中使用前，请考虑以下改进：

- 添加更完善的错误处理
- 实现持久化规则存储
- 添加日志记录和监控
- 实现更复杂的过滤规则
- 添加配置文件支持

## 许可证

MIT License
