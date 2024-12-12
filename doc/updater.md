# 关于更新

## 签名

通过 tauri工具生成签名

```shell
# 可以选择性输入密码 默认密码空
cargo tauri signer generate -w ~/.tauri/myapp.key
```

得到一个 `.key` 和 `.key.pub` 两个文件分别是私钥和公钥
公钥配置到 `tauri.conf.json` 中
私钥编译的时候用到

## 编译

### 设置环境变量

#### Mac/Linux

```bash
export TAURI_SIGNING_PRIVATE_KEY="路径或者明文"
# 生成密钥时如果没填密码,下面这行可以省略
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""
```

#### Windows

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY="Path or content of your private key"
<# optionally also add a password #>
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""
```

### 配置

修改 `tauri.conf.json` 输出 update 包

```json
{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  "plugins": {
    "updater": {
      //  公钥明文
      "pubkey": "CONTENT FROM PUBLICKEY.PEM",
      "endpoints": [
        //  指向某个静态的文件
        "https://host/path/latest.json"
      ]
    }
  }
}
```

### 输出内容

Mac下会额外生成 `.app.tar.gz` 文件用于更新
LInux生成安装包 同时这个安装包也是更新包, 也就是覆盖安装了
`.sig` 文件是安装包的签名, 这个签名最终发布的时候用到

####  Target文件夹结构

- Mac: target/release/bundle/macos/
  - myapp.app --安装包
  - myapp.app.tar.gz。--升级的压缩包
  - myapp.app.tar.gz.sig
- Linux: target/release/bundle/appimage/
  - myapp.AppImage --安装包,同时也是升级包
  - myapp.AppImage.sig
- Windows
  - target/release/bundle/msi/
    - myapp.msi --安装包,同时也是升级包
    - myapp.msi.sig
  - target/release/bundle/nsis
    - myapp-setup.exe --安装包,同时也是升级包
    - myapp-setup.exe.sig


### JSON 文件说明

- version SemVer 格式版本号
- notes 更新说明
- pub_date 发布时间 RFC 3339 格式
- platforms
  - 平台键值格式 OS-ARCH
    - OS linux, darwin windows,
    - ARCH x86_64, aarch64, i686 or armv7.
  - url 发布的安装包的url地址
  - signature 打包生成的sig文件的明文

```json
{
  "version": "1.0.0",
  "notes": "新版本 v1.0.0 修复了 **** 问题 ,添加新功能 ****",
  "pub_date": "2024-12-10T00:00:00.000Z",
  "platforms": {
    "linux-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSWSsxL3JDS0VvMnlmNExYdDVsbHdIRmhRdldENGovZWh0N2dqR1pBb3Y5cFpZTzdpaDZBdEFCR0ordFR3dGRZTnR3NUZZZmZXaFhzQ3k5Qnd5T0pqTDd4VXFWQW1rMXdZPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzMzOTA2OTEwCWZpbGU6YWktbmF2LmFwcC50YXIuZ3oKdW1ZMmNGeWpjUXN1ZzJrUm45d3QyMEtnSVN3MitGY1VOczdDNDhsUjBoMDExWFZJRHZqZGNNTjdtcDYzNnNWeTVobVhRNmpsS3pUb1pMa2x0Q0U1QWc9PQo=",
      "url": "http://localhost:8888/ai-nav.AppImage"
    },
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSWSsxL3JDS0VvMnlmNExYdDVsbHdIRmhRdldENGovZWh0N2dqR1pBb3Y5cFpZTzdpaDZBdEFCR0ordFR3dGRZTnR3NUZZZmZXaFhzQ3k5Qnd5T0pqTDd4VXFWQW1rMXdZPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzMzOTA2OTEwCWZpbGU6YWktbmF2LmFwcC50YXIuZ3oKdW1ZMmNGeWpjUXN1ZzJrUm45d3QyMEtnSVN3MitGY1VOczdDNDhsUjBoMDExWFZJRHZqZGNNTjdtcDYzNnNWeTVobVhRNmpsS3pUb1pMa2x0Q0U1QWc9PQo=",
      "url": "http://localhost:8888/ai-nav.msi"
    },
    "darwin-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSWSsxL3JDS0VvMnlmNExYdDVsbHdIRmhRdldENGovZWh0N2dqR1pBb3Y5cFpZTzdpaDZBdEFCR0ordFR3dGRZTnR3NUZZZmZXaFhzQ3k5Qnd5T0pqTDd4VXFWQW1rMXdZPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzMzOTA2OTEwCWZpbGU6YWktbmF2LmFwcC50YXIuZ3oKdW1ZMmNGeWpjUXN1ZzJrUm45d3QyMEtnSVN3MitGY1VOczdDNDhsUjBoMDExWFZJRHZqZGNNTjdtcDYzNnNWeTVobVhRNmpsS3pUb1pMa2x0Q0U1QWc9PQo=",
      "url": "http://localhost:8888/ai-nav.app.tar.gz"
    }
  }
}
```
