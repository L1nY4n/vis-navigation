# AI Navigation

# 相关技术

## 后端

- `Rust` 熟悉基本的语法 cargo相关的操作等
- `Tauri` 使用v2 版本 了解框架的基本结构、相关的配置

## 前端相关的技术栈
- [vite](vite.dev) 前端构建的工具 ,类似 webpack
- [bun](bun.sh) 前端打包工具, 类似 npm、pnpm、yarn , 速度快
- [react] 前端都懂,还是最主流的前端框架
- Typescript 前端都懂

## 其他

- 需要一点命令行的操作 基本的shell 理解 export 暴露环境变量等基础指令


# 环境准备
## 安装环境
- 安装 Rust ,参考官网操作, 最新的 stable 版本即可
- 安装 Node js, 参考官网操作,搞前端必备
- 安装 bun 参考官网操作

# 开发和编译

## 开发模式

- 进入工程的目录

```shell
# 安装前端的依赖
bun i
# 开发模式
bun tauri dev
```

## 打包 📦

- 生成本平台相关的app执行包  
- release 模式使用了优化编译 时间会久一点
- 输出远程升级包需要配置密钥 详细过程查看 [关于升级](/doc/updater.md)

```shell
# 芜湖 🚀
bun tauri build
```

# 项目结构

```

├── build.sh                             ------快速打包脚本
├── bun.lockb                          ------bun相关依赖生成的lock文件
├── dist                                    ------前端输出的文件夹
│   ├── assets
│   └── index.html
├── doc                                   ------ 相关说明文档
│   └── updater.md                  -------升级说明
├── index.html                        -------前端入口文件
├── node_modules                 -------前端都懂的
├── package.json                   -------前端都懂的
├── public                               ------- web 公共文件,最终打包器会复制到 dist 下面
├── src                                    ------- 前端工程源码路径
│   ├── App.css                        -------样式文件
│   ├── App.tsx                         ------- 前端主组组件
│   ├── assets                           ------- 图片等资源
│   ├── component                   ------- 公共组件
│   ├── main.tsx                        ------- ts 入口文件
│   ├── pages                            ------- 相关的页面路径
│   └── vite-env.d.ts                 ------- vite 环境相关的类型配置
├── tsconfig.json                    ------- ts 配置文件
├── tsconfig.node.json          --------ts 配置文件
└── vite.config.ts                    ------- vite 相关的工程配置
├── src-tauri                           ------- Tauri 源码路径
│   ├── Cargo.lock                   ------- rust 包管理器生成的 lock 文件 类似 bun.lockb  都是用于管理依赖
│   ├── Cargo.toml                   ------- rust 工程/包管理文件 类似 node工程的package.json、PHP的composer.json
│   ├── build.rs                         ------- 编译入口文件,详细查看 cargo的文档
│   ├── capabilities                  ------- Tauri 的权限配置文件
│   ├── gen                               ------- 编译生成相关库,配置平台相关的资源啥的
│   ├── icons                            ------- 放 APP 的 图标
│   ├── src                                ------- tauri 的源码
│    │   ├── cmds.rs                  -------  指令相关文件
│    │   ├── handle.rs                -------  hander 处理 逻辑封装等
│    │   ├── lib.rs                       -------  工程库的入口
│    │   ├── main.rs                   -------  rust 工程入口
│    │   ├── shortcuts.rs           -------  快捷键注册
│    │   ├── tray.rs                     -------  系统托盘处理
│    │   ├── const_data.rs        -------  常量定义,URL定义等
│    │   └── util.rs                     -------  工具函数
│    ├── target                         ------- 打包输出文件夹
│    └── tauri.conf.json           ------- Tauri 的配置文件夹
```