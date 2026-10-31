# CraftCraft

[English](README.md) · [Français](README.fr.md) · [Español](README.es.md) · **中文** · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · [বাংলা](README.bn.md) · [Português](README.pt.md)

*本文为译文，如有出入，以[英文 README](README.md) 为准。*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**用 Rust 重写的知名软件：一个启动器即可安装、更新和打开它们。**

用 PhotoCraft 代替 Photoshop，用 GridCraft 代替 Excel，用 BIMCraft 代替 Revit，用 RustDesk
代替 TeamViewer。CraftCraft 是一个小巧的原生应用（Rust 和 egui，不用 Electron），它列出可替代你正在使用的付费软件的
开源 Rust 应用，为你的电脑安装合适的版本，根据发布的 SHA-256 校验每一次下载，
并在有更新或目录中有新应用时通知你。

## 安装

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

该脚本会从 [Releases](https://github.com/haruny/craftcraft/releases) 下载适用于你系统的最新版本，
校验其 SHA-256，仅为当前用户安装（无需管理员权限），然后打开它。再次运行即可更新。

| | 安装位置 | 另外 |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | “开始”菜单快捷方式；列在“设置 → 应用”中，可在那里卸载；运行时显示托盘图标 |
| macOS | `~/Applications/CraftCraft.app` | 一个版本同时支持 Apple 芯片和 Intel |
| Linux | `~/.local/bin/craftcraft` | 应用程序菜单项和图标 |

也可以从 [Releases](https://github.com/haruny/craftcraft/releases/latest) 下载安装程序：

- **Windows**：`craftcraft-windows-x86_64.msi`（Arm 版为 `-aarch64.msi`）。它会询问为谁安装
  CraftCraft：**Only for me**（仅为我）与脚本的安装方式相同（无需管理员权限），**Everyone**（所有人）
  则在 Windows 请求管理员批准后，安装到 `%ProgramFiles%\CraftCraft` 并添加到每个用户的“开始”菜单。
  之后它会提议打开 CraftCraft。运行更新的安装程序即可更新；它必须与要替换的副本做出相同的选择
  （要切换，请先卸载该副本）。不显示窗口时（`msiexec /i … /qn`），它为当前用户安装，
  加上 `ALLUSERS=1` 则为所有人安装。对于仅为你安装的副本，脚本和安装程序可以相互替换：
  最后运行的那个拥有该安装，“设置 → 应用”中只列出一次。
- **macOS**：`craftcraft-macos-universal.dmg`。打开后将 CraftCraft 拖到“应用程序”文件夹。

在 Windows 和 macOS 上，**设置**中还有：

- **登录时启动**：登录时，CraftCraft 以 `craftcraft --background` 方式在托盘（macOS 上为菜单栏）中启动。
  Windows 通过 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` 下的 `CraftCraft` 值运行它；
  macOS 通过 `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist` 运行它。
- **关闭时最小化到托盘**（默认开启）：关闭窗口后 CraftCraft 仍留在托盘中，继续检查更新；
  图标的提示文字会显示更新数量。在 Windows 上，单击图标打开窗口，右键单击可立即检查或退出。
  在 macOS 上，单击图标会显示同样的菜单；窗口关闭时程序坞中没有图标，从访达打开 CraftCraft
  会重新显示窗口。⌘Q 仍会退出。

只会运行一个副本：再次打开 CraftCraft 会调出托盘中那个副本的窗口。在 Windows 上，
不是通过脚本或安装程序安装的副本（例如从 zip 解压的）会把自己添加到“开始”菜单。

**设置 → 调试模式**（所有平台）会将 CraftCraft 的操作记录到数据文件夹中的 `logs/craftcraft.log`，
并保留下载的安装程序及其日志，而不是删除它们。

## 语言

除非你在 **设置 → 语言** 中选择其他语言，否则 CraftCraft 显示英文。可选语言有：Français、
Español、中文（简体）、हिन्दी、العربية、বাংলা 和 Português。更改立即生效，托盘菜单也会随之更改。
来自应用目录的文字（应用标语和说明、分类、通知）仍为英文，来自系统的错误详情也是如此。

译文位于 [`locales/`](locales)，每种语言一个 JSON 文件，内置于启动器中；译文缺少的内容会回退为英文。
这些译文是新加的，欢迎母语人士指正：编辑文件后运行 `cargo test`，它会检查每个文件是否具有与
`en.json` 相同的键和占位符。

中文、印地语、阿拉伯语和孟加拉语需要内置 Geist 字体所没有的字符，因此启动器会使用电脑上已有的字体：
Windows 上为 Microsoft YaHei、Nirmala UI 和 Segoe UI；macOS 上为 PingFang 或 Hiragino、Kohinoor
以及 SF Arabic 或 Geeza Pro；Linux 上为 Noto（Debian/Ubuntu：`fonts-noto-cjk` 和 `fonts-noto-core`）。
如果都没有，设置中会提示。阿拉伯语文字从右向左显示，但窗口布局仍保持从左到右。

其他语言的 README 均译自英文 README，以英文版为准。

## 一切由目录驱动

应用列表并未内置于启动器中。它来自本仓库中的 [`catalog/catalog.json`](catalog/catalog.json)，
每当它在 `main` 上发生变化，就会发布到 `https://haruny.github.io/craftcraft/catalog.json`。
启动器和网站都读取它。启动器会在以下时候获取它：

- 启动时，并发送上次的 ETag，因此目录未变化时只需一个很小的请求；
- 运行期间每隔几小时（默认 8 小时；可在 **设置** 中设为 1–168），失败后 15 分钟重试；
- 当你点击 **检查更新** 时。

它会保留上一份有效副本，因此能立即打开，也能离线使用。当目录发生变化时：

| `catalog.json` 中的变化 | 启动器显示的内容 |
|---|---|
| `apps` 中新增一个应用 | 一条“目录新增”横幅和“新”徽标，直到用户点击 **知道了** |
| `launcher.version` 比当前版本新 | 一条带下载链接的更新横幅 |
| `notices` 中新增一个条目 | 一条横幅，直到被忽略 |
| 某个项目发布了新版本 | 该应用卡片上出现 **更新** |
| 某个应用列在 `featured.json` 中 | 它排在默认“精选”排序的前面，并带有“精选”徽标 |

**发布信息来自一个文件，而不是 GitHub 的 API。** 目录工作流每小时将每个应用的最新版本
（文件和 SHA-256）、星数和许可证汇总到 `https://haruny.github.io/craftcraft/releases.json`。
启动器在获取目录时一并通过 ETag 获取它，因此无论目录有多长，检查所有应用的更新只需一个请求，
也没有人会碰到 GitHub 每小时 60 次 API 请求的上限。只有索引尚未涵盖的应用（例如正在测试的本地目录）
才会直接在 GitHub 上查询。星数和发布日期也用于 **排序** 菜单：精选、最受欢迎、名称和最近更新。

**要添加应用，请提交一个将其添加到 `catalog/catalog.json` 的拉取请求**，或者把
[ADD_APP.md](ADD_APP.md) 和该应用的 GitHub 仓库交给 AI 智能体处理。
[CONTRIBUTING.md](CONTRIBUTING.md) 中列出了规则：应用必须能替代付费软件，并且在 github.com 上开源。

目录地址是固定的：正式版本始终读取已发布的目录，没有可以更改它的设置。

## 你已有的应用

启动器会在启动时、每次检查目录后以及每次安装或卸载后，查找电脑上已有的每个目录应用。
以这种方式找到的应用会显示为已安装并附带版本号，出现 **打开** 按钮，并在其项目发布新版本时出现 **更新**。

| 位置 | 查找方式 | 版本来源 |
|---|---|---|
| Windows 桌面应用 | 已安装程序列表（注册表的 Uninstall 条目），严格按名称匹配：`PhotoCraft`、`PhotoCraft 0.16.0` 和 `DesignCraft (Craft Apps)` 匹配，`ArtCraft Launcher` 不匹配 ArtCraft | `DisplayVersion` |
| macOS 桌面应用 | `/Applications/<Name>.app`、`~/Applications/<Name>.app` | 应用包的 `Info.plist` |
| Linux 桌面应用 | 目录中的 `launch` 路径、PATH 中的命令，或 `~/Applications` 中的 AppImage | dpkg 或 rpm，或 AppImage 的文件名 |
| 命令行和终端应用（所有平台） | PATH 中的命令 | 运行 `<command> --version`（隐藏运行，最多 3 秒） |

更新由其自带安装程序安装的应用时，会运行新版本的安装程序，就地升级。MSI 包以 `/passive` 运行，
当已安装的副本是为所有人安装的时还会加上 `ALLUSERS=1`，因此 Windows 会请求管理员批准；
哪些副本属于这种情况以 Windows Installer 自身的记录为准，因为它也会把仅为单个用户安装的副本列在 HKLM 下。
以其他方式（winget、cargo、脚本）安装的命令行应用无法被替换，因此 **安装最新版** 会改为在启动器的文件夹中
放置一个最新副本。**卸载…** 会运行该应用自带的卸载程序。

当安装、更新、卸载或 **打开** 失败时，卡片会说明哪一步失败以及原因，例如
*安装程序失败：This app is already installed for everyone…*，悬停可查看完整内容。
对于 MSI 包，原因来自 Windows Installer 的日志，可通过 **⋯ → 安装程序日志** 打开；
**⋯ → 复制错误信息** 会复制全部内容。

`craftcraft --detect <catalog.json>` 会打印启动器找到的内容。

## 应用的安装方式

文件只来自每个应用自己的 GitHub 发布（`https://github.com/<repo>/releases/download/…`），
并根据 GitHub 为每个发布文件公布的 SHA-256 进行验证。没有 SHA-256 的文件仍会安装，但会被标记为 **未经验证**。

| 发布文件 | 处理方式 |
|---|---|
| `.zip`、`.tar.gz`、`.tar.xz` | 解压到启动器的 `apps/<id>` 文件夹。更新会解压到旁边，完成后才替换，因此更新失败时旧版本仍可使用 |
| `.AppImage`、单个 `.exe` | 作为单个可执行文件保存在 `apps/<id>` 中 |
| `.msi`、`.exe` 安装程序、`.dmg`、`.pkg`、`.deb`、`.rpm` | 交给系统安装程序处理。使用目录中的 `launch` 路径来查找并打开它 |

终端和命令行应用还会在 `bin` 文件夹中获得一个启动器（Windows 上为 `.cmd` 文件，其他平台上为符号链接）。
在 Windows 上，**设置 → 添加到我的 PATH** 会将该文件夹加入 PATH；在 macOS 和 Linux 上，
设置中会显示需要添加到 shell 配置文件中的那一行。**在终端中打开** 会打开一个 PATH 中已包含该文件夹的终端。

所有内容都位于一个数据文件夹中：Windows 上为 `%LOCALAPPDATA%\CraftCraft\CraftCraft\data`，
macOS 上为 `~/Library/Application Support/io.CraftCraft.CraftCraft`，Linux 上为 `~/.local/share/craftcraft`，
设置了 `$CRAFTCRAFT_HOME` 时则为该路径。在你点击 **安装** 或 **更新** 之前不会下载任何内容。
在 Windows 上，安装程序运行后即被删除，MSI 的日志在安装成功后删除；在 macOS 和 Linux 上，
启动器交接时系统安装程序仍处于打开状态，因此这些文件会在一周后删除。调试模式会保留所有这些文件。

## 构建和运行

你需要 Rust 1.90 或更高版本。在 Linux 上，请先安装窗口系统头文件（Debian/Ubuntu：
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`）。

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| 环境变量 | 用途 |
|---|---|
| `CRAFTCRAFT_FEED` | **仅限调试版本：** 从其他地址或本地文件读取目录，例如编辑时使用 `catalog/catalog.json`。正式版本会忽略它 |
| `CRAFTCRAFT_HOME` | 使用另一个数据文件夹（便于在干净的环境中测试） |
| `GITHUB_TOKEN` | 将 GitHub 的上限从每小时 60 次请求提高到 5,000 次 |

`craftcraft --check-catalog <catalog.json>` 会完全按启动器的方式解析目录，并列出会被跳过的内容。

## 发布

推送 `v0.2.0` 这样的标签（与 `Cargo.toml` 中的 `version` 一致）会运行
[release.yml](.github/workflows/release.yml)。它会在各自的运行器上构建 Windows（x86_64、Arm）、
macOS（一个通用 `.app`，同时提供 `.dmg`）和 Linux（x86_64、Arm），用安装脚本安装每个构建以进行测试，
将签名后的 Windows 构建打包为 `.msi` 安装程序并同样测试，然后发布一个包含这些包和 `SHA256SUMS` 文件的
GitHub 发布。手动运行该工作流会完成除发布以外的所有步骤：构建产物附加到该次运行中，供测试使用。
发布后，请提升目录中的 `launcher.version`，让旧版启动器提示更新。

要在本地构建并试用一个包（与用户获得的方式相同）：

```powershell
./scripts/dist.ps1                                   # Windows: dist/craftcraft-windows-<arch>.zip
$env:CRAFTCRAFT_DIST = "$PWD\dist"; ./install/install.ps1
# The .msi, with WiX 3 (https://github.com/wixtoolset/wix3) installed:
./scripts/msi.ps1 -Exe target/x86_64-pc-windows-msvc/release/craftcraft.exe -Arch x86_64
```
```sh
sh scripts/dist.sh                                   # macOS or Linux
CRAFTCRAFT_DIST="$PWD/dist" sh install/install.sh
```

### 签名

在仓库的 **Settings → Secrets and variables → Actions** 中设置以下内容后，发布工作流就会对构建进行签名。
在此之前，它发布的是未签名的构建。

**macOS（Developer ID，然后公证）。** 以下五项均为机密：

| 机密 | 说明 |
|---|---|
| `APPLE_CERTIFICATE` | 一个 **Developer ID Application** 证书及其私钥，从“钥匙串访问”导出为 `.p12`，再进行 base64 编码（`base64 -i cert.p12 \| pbcopy`） |
| `APPLE_CERTIFICATE_PASSWORD` | 你为 `.p12` 设置的密码 |
| `APPLE_API_KEY` | 一个 App Store Connect API 密钥（Users and Access → Integrations → Team Keys，角色为 Developer）：`.p8` 文件，经 base64 编码 |
| `APPLE_API_KEY_ID` | 该密钥的 ID |
| `APPLE_API_ISSUER` | 密钥上方显示的 Issuer ID |

如果只设置前两项，应用会被签名但不会被公证，从浏览器下载时 Gatekeeper 仍会发出警告。要在本地签名，
请在运行 `scripts/dist.sh` 之前设置 `APPLE_SIGNING_IDENTITY`（以及 `APPLE_API_KEY_*` 变量，
其中 `APPLE_API_KEY_PATH` 指向 `.p8` 文件）。

**Windows（Microsoft Artifact Signing，前身为 Trusted Signing）。** 无需机密：工作流通过 GitHub 的
OpenID Connect 登录 Azure，而 Azure 只信任来自本仓库 `release` 环境的登录。

1. 在 Azure portal 中创建一个 **Artifact Signing account**（Basic 层级即可），并记下其终结点，
   例如 `https://eus.codesigning.azure.net/`。
2. 在该账户中完成 **identity validation**（Public），然后基于它创建一个类型为 Public Trust 的
   **certificate profile**。
3. 在 Microsoft Entra ID 中注册一个应用。在 Certificates & secrets → Federated credentials 下，
   为 GitHub Actions 添加一个凭据：organization 为 `haruny`，repository 为 `craftcraft`，entity type 为
   **Environment**，name 为 `release`。
4. 在签名账户上，为该应用授予 **Artifact Signing Certificate Profile Signer** 角色
   （Access control → Add role assignment）。
5. 在 GitHub 中创建环境 `release`（Settings → Environments）。将其限制为 `main` 分支和 `v*` 标签，
   这样只有经过审查的代码才会被签名。
6. 添加以下仓库变量：

| 变量 | 值 |
|---|---|
| `AZURE_CLIENT_ID` | 应用注册的 Application (client) ID |
| `AZURE_TENANT_ID` | 它的 Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | 签名账户所在的订阅 |
| `ARTIFACT_SIGNING_ENDPOINT` | 第 1 步中的终结点 |
| `ARTIFACT_SIGNING_ACCOUNT` | 签名账户的名称 |
| `ARTIFACT_SIGNING_PROFILE` | 证书配置文件（certificate profile）的名称 |

此后，发布工作流的每次运行都会对两个 Windows zip（x86_64 和 Arm）中的 `craftcraft.exe` 进行签名，
并在发布 zip 之前检查每个签名和时间戳。手动启动的运行也会签名，因此你可以在打标签之前先试一试。

## 隐私

完整信息见 [Privacy_Statement.md](Privacy_Statement.md)。简而言之：

- **计数。** 正式版本通过 CraftCraft 自己的端点获取目录，该端点按启动器版本、操作系统及其版本、
  处理器和国家/地区统计每天的目录检查次数。它不保留 IP 地址或 ID。我们由此了解 CraftCraft 运行在哪些电脑上。
- **使用统计**，可以关闭。包括启动次数，以及你安装、更新、打开和移除了哪些应用，使用一个与你或你的电脑
  无关的随机 ID 发送。在你看到相关通知之前不会发送任何内容。可以在通知中或在“设置 > 隐私”中关闭。
- `DO_NOT_TRACK=1` 或 `CRAFTCRAFT_NO_TELEMETRY=1` 会同时关闭两者：此时目录直接来自 GitHub Pages。

从源代码构建的版本两者都没有（见 [telemetry/README.md](telemetry/README.md)）。除此之外，
CraftCraft 只会连接 `haruny.github.io` 和 `api.github.com` 以获取发布信息、获取目录中列出的应用图标，
以及从 `github.com` 获取你选择安装的发布文件。与任何网络请求一样，这些服务能看到你的 IP 地址。

## 目录结构

```
catalog/         catalog.json and featured.json; check.mjs (CI runs it on every change); publish.mjs
                 and releases.mjs, which build what's published (catalog.json, releases.json)
locales/         the launcher's text, one JSON file per language
install/         install.ps1 and install.sh, the one-line installers
telemetry/       the usage statistics endpoint (a Cloudflare Worker)
installer/       craftcraft.wxs, the Windows .msi (WiX 3)
scripts/         dist.ps1 and dist.sh, which build and package a release (dist.sh: the .dmg too);
                 msi.ps1, which packages the .msi
src/index.rs     the release index: every app's latest release and stars, in one file
src/catalog.rs   the catalog format: parsing, validation, platforms, picking a release file
src/feed.rs      fetching and caching the catalog
src/github.rs    an app's latest release from GitHub's API, for apps the index doesn't cover
src/detect.rs    finding apps already installed (registry, app bundles, packages, PATH)
src/install.rs   download, verify, unpack, swap in, shims, uninstall
src/system.rs    opening apps, terminals, folders and links; PATH
src/icons.rs     app icons, cached and decoded off the UI thread
src/worker.rs    background jobs and the events they send the UI
src/i18n.rs      the launcher's text in each language (from locales/), and right-to-left lines
src/fonts.rs     finding a system font for Chinese, Hindi, Arabic and Bengali
src/tray.rs      the tray (menu bar) icon on Windows and macOS; on Windows, keeping to one running copy
src/ui.rs        the window
src/theme.rs     colours, fonts and widget styling
```

## 赞助

CraftCraft 是免费的，由我利用业余时间开发。如果它为你省下了一两份许可证费用，
欢迎通过 [GitHub Sponsors](https://github.com/sponsors/haruny) 支持它。

## 许可证

MIT OR Apache-2.0。内置的 Geist 字体（`assets/fonts`）采用 SIL Open Font License
（`assets/fonts/OFL.txt`）。启动器安装的每个应用都采用其各自的许可证。
