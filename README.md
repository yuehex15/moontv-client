# MoonTV 轻量 Windows 客户端

基于 **Tauri v2** 的极轻量 MoonTV 影视客户端，专为低内存设备设计（如小米平板2，2GB RAM）。

## ✨ 特性

- 🪶 **极低内存占用**：仅 ~50-100MB RAM
- 📦 **极小安装包**：压缩包仅 ~8 MB
- 🚀 **快速启动**：用系统 WebView2 渲染
- 🎮 **平板优化**：默认 1280x720 窗口，支持全屏

## 📥 下载

从 [Releases](https://github.com/yuehex15/moontv-client/releases) 页面下载最新版的 `MoonTV-Windows-x64.zip`。

## 📝 使用

1. 解压 zip 到任意目录（如 `D:\Apps\MoonTV\`）
2. 运行 `MoonTV.exe`
3. 输入账号密码登录观看

## 🔧 修改域名（重要）

MoonTV 是自建服务，**域名可能会变**。域名默认值在仓库源码中：

```
src-tauri/src/lib.rs → get_default_url()
```

修改返回的 URL 后推送，GitHub Actions 会自动重新构建，新版本即使用新域名。

## ⚙️ 运行时配置

同目录下 `settings.ini` 可自定义**代理和 WebView2 优化**（不涉及域名）：

```ini
# 代理模式：system / direct / custom
proxy_mode = system

# 自定义代理（仅 custom 模式）
# custom_proxy = 127.0.0.1:10809

# 禁用 GPU 加速（省内存，默认开启）
disable_gpu = true

# JS 堆内存上限（MB，默认 128）
max_js_heap = 128
```

## ⌨️ 快捷键

- `F11` - 切换全屏
- `Ctrl+R` - 刷新

## 🖥 最低配置

| 项目 | 要求 |
|------|------|
| 系统 | Windows 10/11 x64 |
| 内存 | 最小 1GB，推荐 2GB |
| WebView2 | 系统自带（Win10/11 已内置） |