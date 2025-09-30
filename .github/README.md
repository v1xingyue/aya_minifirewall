# GitHub Actions 工作流说明

本项目包含以下 GitHub Actions 工作流：

## 🔄 工作流文件

### 1. CI 工作流 (`ci.yml`)
- **触发条件**: 推送到 main/develop 分支，或创建 PR
- **功能**: 代码检查、格式化检查、编译测试
- **权限**: 只读权限

### 2. Release 工作流 (`release.yml`)
- **触发条件**: 推送标签 (v*) 或手动触发
- **功能**: 构建二进制文件并创建 GitHub Release
- **权限**: 写入权限 (contents, packages, id-token)

### 3. 完整构建和发布工作流 (`build-and-release.yml`)
- **触发条件**: 推送标签 (v*) 或手动触发
- **功能**: 完整的构建、测试和发布流程
- **权限**: 写入权限

## 🚀 使用方法

### 自动发布 Release

1. **创建标签**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **手动触发发布**:
   - 进入 GitHub Actions 页面
   - 选择 "Release" 工作流
   - 点击 "Run workflow"
   - 输入版本号 (如: v1.0.0)

### 权限配置

确保在 GitHub 仓库设置中启用以下权限：

1. **Actions 权限**:
   - Settings → Actions → General
   - Workflow permissions: "Read and write permissions"
   - Allow GitHub Actions to create and approve pull requests: ✅

2. **Release 权限**:
   - 确保有创建 Release 的权限
   - 确保有上传文件的权限

## 📦 发布内容

每次发布将包含：

- `aya-minifirewall`: 主程序二进制文件
- `aya-minifirewall-ebpf`: eBPF 内核程序
- `build-info.txt`: 构建信息
- `aya-minifirewall-linux-x86_64.tar.gz`: 完整发布包

## 🔧 故障排除

### 权限错误
如果遇到权限错误，检查：
1. 仓库的 Actions 权限设置
2. 工作流文件中的 permissions 配置
3. GitHub 账户的权限设置

### 构建失败
如果构建失败，检查：
1. Rust 工具链版本
2. eBPF 依赖是否正确安装
3. 内核头文件是否可用

### Release 创建失败
如果 Release 创建失败，检查：
1. 标签格式是否正确 (v*)
2. 权限设置是否正确
3. 文件路径是否正确
