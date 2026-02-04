# Contributing to Pinocchio Escrow Workspace

感谢你关注本项目！我们欢迎各种形式的贡献，无论是修复 Bug、改进文档还是添加新的托管模式实现。

为了保持代码库的高质量和一致性，请在提交贡献前阅读以下指南。

## 🛠 开发环境准备

在开始编写代码之前，请确保你的本地环境满足以下要求：

* **Rust**: `1.88.0` 或更高版本（推荐使用 `1.92.0`）。
* **Solana CLI**: `2.1.0` 或更高版本（Agave 工具链）。
* **Just**: 本项目使用 `just` 作为命令运行器。
* **Edition**: 所有程序必须使用 **Rust 2024 Edition**。

## 📂 分支策略

* `master`: 稳定分支。所有代码合并都必须通过 Pull Request。
* 特性开发：建议使用 `feat/your-feature-name` 或 `fix/your-bug-name` 格式的分支。

## 📜 代码规范

由于本项目使用 **Pinocchio (Zero-std)** 框架，请遵循以下技术约束：

1. **禁止使用 `std**`: 程序必须保持 `no_std` 以确保极致的计算单元（CU）效率。
2. **Lint 检查**: 提交前请运行 `just fmt`。我们对 `unexpected_cfgs` 进行了专门配置，请勿随意更改 `Cargo.toml` 中的 `[lints.rust]` 部分。
3. **注释规范**:

* 关键的参数顺序（如 `Payer`, `Vault`, `Owner`）必须在代码中清晰标注。
* 尽量避免使用 LaTeX 渲染简单的单位（如使用 `10%` 而非 `$10\%$`）。

## 🚀 提交步骤

### 1. 编译验证

在根目录下运行以下命令，确保所有子项目都能通过编译：

```bash
just build-all

```

### 2. 静态检查

运行 `cargo-deny`（如果已安装）来检查依赖安全性：

```bash
cargo deny check

```

### 3. 提交信息 (Commit Messages)

我们遵循简化的约定式提交规范：

* `feat`: 新功能或新版本程序（如 `feat: add pinocchio_escrow v0.10.1`）
* `fix`: 修复 Bug（如 `fix: corrected account info order in make.rs`）
* `docs`: 文档更新
* `chore`: 更改构建任务、依赖库更新等

## 🧪 测试

虽然 Pinocchio 是底层框架，但我们鼓励为每个 `instruction` 编写集成测试。新增加的功能应尽可能包含对应的测试用例。

## ⚖️ 许可证

当你向本项目提交贡献时，即表示你同意你的贡献将遵循仓库中的 [MIT License](https://www.google.com/search?q=./LICENSE)。

---
