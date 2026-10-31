# CraftCraft

[English](README.md) · [Français](README.fr.md) · [Español](README.es.md) · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · **বাংলা** · [Português](README.pt.md)

*এটি একটি অনুবাদ; মূল রেফারেন্স হলো [ইংরেজি README](README.md)।*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**সুপরিচিত সফটওয়্যার, Rust-এ নতুন করে লেখা: সেগুলো ইনস্টল, আপডেট ও খোলার জন্য একটিই লঞ্চার।**

Photoshop-এর বদলে PhotoCraft, Excel-এর বদলে GridCraft, Revit-এর বদলে BIMCraft, TeamViewer-এর
বদলে RustDesk। CraftCraft একটি ছোট নেটিভ অ্যাপ (Rust ও egui, কোনো Electron নয়), যা আপনি ইতিমধ্যে
যে পেইড সফটওয়্যার ব্যবহার করেন তার ওপেন-সোর্স Rust বিকল্পগুলোর তালিকা দেখায়, আপনার কম্পিউটারের জন্য
সঠিক বিল্ডটি ইনস্টল করে, প্রতিটি ডাউনলোড তার প্রকাশিত SHA-256-এর সঙ্গে মিলিয়ে দেখে, এবং কোনো আপডেট বা
ক্যাটালগে নতুন অ্যাপ এলে আপনাকে জানায়।

## ইনস্টল

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

স্ক্রিপ্টটি [Releases](https://github.com/haruny/craftcraft/releases) থেকে আপনার সিস্টেমের জন্য
সর্বশেষ রিলিজ ডাউনলোড করে, তার SHA-256 চেক করে, শুধু আপনার ইউজারের জন্য ইনস্টল করে (কোনো অ্যাডমিন
অধিকার লাগে না), এবং সেটি খোলে। আপডেট করতে এটি আবার চালান।

| | যেখানে ইনস্টল হয় | এছাড়াও |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Start মেনুতে শর্টকাট; সেটিংস → অ্যাপস-এ তালিকাভুক্ত, যেখান থেকে এটি আনইনস্টল করা যায়; চলার সময় একটি ট্রে আইকন |
| macOS | `~/Applications/CraftCraft.app` | Apple silicon ও Intel-এর জন্য একটিই বিল্ড |
| Linux | `~/.local/bin/craftcraft` | অ্যাপ্লিকেশন মেনুতে এন্ট্রি ও আইকন |

অথবা [Releases](https://github.com/haruny/craftcraft/releases/latest) থেকে একটি ইনস্টলার ডাউনলোড করুন:

- **Windows**: `craftcraft-windows-x86_64.msi` (বা Arm-এর জন্য `-aarch64.msi`)। এটি জিজ্ঞেস করে CraftCraft
  কার জন্য: **Only for me** স্ক্রিপ্টের মতো একইভাবে ইনস্টল করে (কোনো অ্যাডমিন অধিকার লাগে না), **Everyone**
  Windows একজন অ্যাডমিনিস্ট্রেটরের অনুমোদন চাওয়ার পর এটি `%ProgramFiles%\CraftCraft`-এ এবং প্রত্যেক
  ইউজারের Start মেনুতে ইনস্টল করে। তারপর এটি CraftCraft খোলার প্রস্তাব দেয়। আপডেট করতে একটি নতুনটি চালান;
  এটিকে যে কপিটি প্রতিস্থাপন করছে তার মতো একই পছন্দ করতে হয় (বদলাতে চাইলে সেই কপিটি আনইনস্টল করুন)। উইন্ডো
  ছাড়া (`msiexec /i … /qn`) এটি বর্তমান ইউজারের জন্য ইনস্টল করে, বা `ALLUSERS=1` দিলে সবার জন্য।
  শুধু আপনার জন্য থাকা কপির ক্ষেত্রে স্ক্রিপ্ট ও ইনস্টলার একে অপরের জায়গা নিতে পারে: যেটি শেষে চলেছে
  ইনস্টলটি তার, এবং সেটিংস → অ্যাপস এটিকে একবারই দেখায়।
- **macOS**: `craftcraft-macos-universal.dmg`। এটি খুলুন এবং CraftCraft-কে Applications-এ টেনে আনুন।

Windows ও macOS-এ **সেটিংস**-এ আরও আছে:

- **লগঅনে চালু করুন**: আপনি সাইন ইন করলে CraftCraft ট্রেতে (macOS-এ মেনু বারে) চালু হয়,
  `craftcraft --background` হিসেবে। Windows এটি
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`-এর অধীনে `CraftCraft` মান থেকে চালায়; macOS চালায়
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist` থেকে।
- **বন্ধ করলে ট্রেতে মিনিমাইজ করুন** (ডিফল্টভাবে চালু): উইন্ডো বন্ধ করলে CraftCraft ট্রেতে থেকে যায়,
  আপডেট চেক করতে থাকে; আইকনের টুলটিপ সেগুলোর সংখ্যা দেখায়। Windows-এ উইন্ডো খুলতে আইকনে ক্লিক করুন,
  বা এখনই চেক করতে বা বন্ধ করতে রাইট-ক্লিক করুন। macOS-এ একই মেনুর জন্য এতে ক্লিক করুন; উইন্ডো বন্ধ
  থাকলে Dock-এ কোনো আইকন থাকে না, এবং Finder থেকে CraftCraft খুললে উইন্ডোটি ফিরে আসে। ⌘Q দিয়ে এখনও
  বন্ধ করা যায়।

শুধু একটি কপিই চলে: CraftCraft আবার খুললে ট্রেতে থাকা কপির উইন্ডোটি ফিরে আসে। Windows-এ, যে কপি
স্ক্রিপ্ট বা ইনস্টলার দিয়ে ইনস্টল করা হয়নি (যেমন zip থেকে আনপ্যাক করা), সেটি নিজেকে Start মেনুতে যোগ করে।

**সেটিংস → ডিবাগ মোড** (সব জায়গায়) CraftCraft যা করে তা ডেটা ফোল্ডারের `logs/craftcraft.log`-এ লগ করে,
এবং ডাউনলোড করা ইনস্টলার ও সেগুলোর লগ মুছে না ফেলে রেখে দেয়।

## ভাষা

আপনি **সেটিংস → ভাষা**-তে অন্য ভাষা না বেছে নিলে CraftCraft ইংরেজিতে থাকে: Français, Español,
中文（简体）, हिन्दी, العربية, বাংলা বা Português। পরিবর্তনটি সঙ্গে সঙ্গে কার্যকর হয়, ট্রে মেনুসহ।
ক্যাটালগ থেকে আসা লেখা (অ্যাপের ট্যাগলাইন ও বিবরণ, বিভাগ, নোটিশ) ইংরেজিতেই থাকে, সিস্টেম থেকে আসা
ত্রুটির বিবরণও তাই।

অনুবাদগুলো [`locales/`](locales)-এ আছে, প্রতিটি ভাষার জন্য একটি JSON ফাইল, লঞ্চারের ভেতরে বিল্ট-ইন;
কোনো অনুবাদে যা নেই তা ইংরেজিতে দেখায়। এগুলো নতুন, তাই মাতৃভাষীদের সংশোধন স্বাগত: ফাইলটি সম্পাদনা করুন
এবং `cargo test` চালান, যা চেক করে যে প্রতিটি ফাইলে `en.json`-এর মতো একই কী ও প্লেসহোল্ডার আছে।

চীনা, হিন্দি, আরবি ও বাংলার জন্য এমন অক্ষর লাগে যা বান্ডেল করা Geist ফন্টে নেই, তাই লঞ্চার কম্পিউটারে
আগে থেকে থাকা একটি ফন্ট ব্যবহার করে: Windows-এ Microsoft YaHei, Nirmala UI ও Segoe UI; macOS-এ PingFang বা
Hiragino, Kohinoor এবং SF Arabic বা Geeza Pro; Linux-এ Noto (Debian/Ubuntu: `fonts-noto-cjk` ও
`fonts-noto-core`)। কোনোটি না থাকলে সেটিংস তা জানিয়ে দেয়। আরবি লেখা ডান থেকে বামে পড়া হয়, কিন্তু
উইন্ডোর লেআউট বাম থেকে ডানেই থাকে।

অন্যান্য ভাষার README-গুলো এটির অনুবাদ, এবং এটিই রেফারেন্স।

## সবকিছু চালায় ক্যাটালগ

অ্যাপের তালিকা লঞ্চারের ভেতরে বিল্ট-ইন নয়। এটি আসে এই রিপোজিটরির
[`catalog/catalog.json`](catalog/catalog.json) থেকে, যা `main`-এ পরিবর্তন হলেই
`https://haruny.github.io/craftcraft/catalog.json`-এ প্রকাশিত হয়। লঞ্চার ও ওয়েবসাইট দুটোই এটি পড়ে।
লঞ্চার এটি আনে:

- চালু হওয়ার সময়, শেষ ETag পাঠিয়ে, যাতে অপরিবর্তিত ক্যাটালগে মাত্র একটি ছোট অনুরোধ খরচ হয়;
- চলার সময় প্রতি কয়েক ঘণ্টা পরপর (ডিফল্টভাবে 8; **সেটিংস**-এ 1–168 সেট করা যায়), ব্যর্থ হলে 15 মিনিট পরে আবার চেষ্টা করে;
- আপনি **আপডেট চেক করুন** চাপলে।

এটি শেষ ভালো কপিটি রেখে দেয়, তাই এটি সঙ্গে সঙ্গে খোলে এবং অফলাইনেও কাজ করে। ক্যাটালগ পরিবর্তন হলে:

| `catalog.json`-এ পরিবর্তন | লঞ্চার যা দেখায় |
|---|---|
| `apps`-এ একটি নতুন অ্যাপ | "ক্যাটালগে নতুন" ব্যানার ও একটি নতুন ব্যাজ, যতক্ষণ না ইউজার **বুঝেছি** ক্লিক করেন |
| `launcher.version` এই বিল্ডের চেয়ে নতুন | ডাউনলোড লিংকসহ একটি আপডেট ব্যানার |
| `notices`-এ একটি নতুন এন্ট্রি | বাতিল না করা পর্যন্ত একটি ব্যানার |
| কোনো প্রজেক্ট একটি রিলিজ প্রকাশ করে | সেই অ্যাপের কার্ডে **আপডেট করুন** |
| কোনো অ্যাপ `featured.json`-এ তালিকাভুক্ত | ডিফল্ট বিশেষ ক্রমে এটি প্রথমে থাকে, একটি বিশেষ ব্যাজসহ |

**রিলিজের তথ্য আসে একটি ফাইল থেকে, GitHub-এর API থেকে নয়।** প্রতি ঘণ্টায় ক্যাটালগ ওয়ার্কফ্লো প্রতিটি
অ্যাপের সর্বশেষ রিলিজ (ফাইল ও SHA-256), স্টার ও লাইসেন্স সংগ্রহ করে
`https://haruny.github.io/craftcraft/releases.json`-এ রাখে। লঞ্চার এটি ক্যাটালগের পাশাপাশি একটি ETag দিয়ে
আনে, তাই ক্যাটালগ যত বড়ই হোক, প্রতিটি অ্যাপের আপডেট চেক করতে একটিই অনুরোধ লাগে, এবং কেউ ঘণ্টায় 60টি API
অনুরোধের GitHub-এর সীমায় পৌঁছায় না। শুধু যে অ্যাপ ইনডেক্সে এখনও নেই (যেমন পরীক্ষাধীন কোনো লোকাল ক্যাটালগ),
সেটিই সরাসরি GitHub-এ খোঁজা হয়। স্টার ও রিলিজের তারিখ **সাজান** মেনুও চালায়: বিশেষ, সবচেয়ে জনপ্রিয়, নাম
ও সম্প্রতি আপডেট হয়েছে।

**কোনো অ্যাপ যোগ করতে, `catalog/catalog.json`-এ সেটি যোগ করে একটি pull request পাঠান**, অথবা অ্যাপের GitHub
রিপোজিটরিসহ কোনো AI এজেন্টকে [ADD_APP.md](ADD_APP.md) দেখিয়ে দিন। নিয়মগুলো
[CONTRIBUTING.md](CONTRIBUTING.md)-এ আছে: অ্যাপটিকে অবশ্যই কোনো পেইড সফটওয়্যারের বিকল্প হতে হবে, এবং
github.com-এ ওপেন সোর্স হতে হবে।

ক্যাটালগের ঠিকানা নির্দিষ্ট: রিলিজ বিল্ড সবসময় প্রকাশিতটিই পড়ে, এবং এটি পরিবর্তনের কোনো সেটিং নেই।

## আপনার কাছে আগে থেকে থাকা অ্যাপ

লঞ্চার কম্পিউটারে আগে থেকে থাকা প্রতিটি ক্যাটালগ অ্যাপ খোঁজে, চালু হওয়ার সময়, প্রতিটি ক্যাটালগ চেকের পরে এবং
প্রতিটি ইনস্টল বা আনইনস্টলের পরে। এভাবে পাওয়া অ্যাপ তার সংস্করণসহ ইনস্টল করা হিসেবে দেখায়, **খুলুন** পায়,
এবং তার প্রজেক্ট নতুন রিলিজ প্রকাশ করলে **আপডেট করুন** পায়।

| কোথায় | কীভাবে পাওয়া যায় | সংস্করণ কোথা থেকে |
|---|---|---|
| Windows ডেস্কটপ অ্যাপ | ইনস্টল করা প্রোগ্রামের তালিকা (রেজিস্ট্রির Uninstall এন্ট্রি), নাম দিয়ে কঠোরভাবে মেলানো: `PhotoCraft`, `PhotoCraft 0.16.0` ও `DesignCraft (Craft Apps)` মেলে, `ArtCraft Launcher` ArtCraft-এর সঙ্গে মেলে না | `DisplayVersion` |
| macOS ডেস্কটপ অ্যাপ | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | বান্ডেলের `Info.plist` |
| Linux ডেস্কটপ অ্যাপ | ক্যাটালগের `launch` পাথ, PATH-এ থাকা কমান্ড, বা `~/Applications`-এ একটি AppImage | dpkg বা rpm, বা AppImage-এর ফাইলের নাম |
| কমান্ড-লাইন ও টার্মিনাল অ্যাপ (সব জায়গায়) | PATH-এ থাকা কমান্ড | `<command> --version` চালিয়ে (লুকানো, সর্বোচ্চ 3 সেকেন্ড) |

নিজস্ব ইনস্টলার দিয়ে ইনস্টল করা অ্যাপ আপডেট করলে নতুন রিলিজের ইনস্টলার চলে, যা সেটিকে জায়গাতেই আপগ্রেড করে।
MSI প্যাকেজ `/passive` দিয়ে চলে, এবং ইনস্টল করা কপি সবার জন্য হলে `ALLUSERS=1` দিয়ে, তাই Windows একজন
অ্যাডমিনিস্ট্রেটরের অনুমোদন চায়; কোন কপিগুলো এমন তা Windows Installer-এর নিজস্ব রেকর্ড বলে দেয়, কারণ এটি শুধু
একজন ইউজারের জন্য ইনস্টল করা কপিগুলোও HKLM-এর অধীনে তালিকাভুক্ত করে। অন্য কোনোভাবে (winget, cargo, কোনো
স্ক্রিপ্ট) ইনস্টল করা কমান্ড-লাইন অ্যাপ প্রতিস্থাপন করা যায় না, তাই **সর্বশেষটি ইনস্টল করুন** তার বদলে লঞ্চারের
ফোল্ডারে একটি হালনাগাদ কপি রাখে। **আনইনস্টল করুন…** অ্যাপের নিজস্ব আনইনস্টলার চালায়।

কোনো ইনস্টল, আপডেট, আনইনস্টল বা **খুলুন** ব্যর্থ হলে, কার্ড জানায় কোন ধাপ ব্যর্থ হয়েছে ও কেন, যেমন
*ইনস্টলার ব্যর্থ হয়েছে: This app is already installed for everyone…*, পুরোটা হোভার করলে দেখা যায়।
MSI প্যাকেজের ক্ষেত্রে কারণটি আসে Windows Installer-এর লগ থেকে, যা **⋯ → ইনস্টলার লগ** খোলে;
**⋯ → ত্রুটি কপি করুন** পুরোটা কপি করে।

`craftcraft --detect <catalog.json>` লঞ্চার যা খুঁজে পায় তা প্রিন্ট করে।

## অ্যাপ কীভাবে ইনস্টল হয়

ফাইল আসে শুধু প্রতিটি অ্যাপের নিজস্ব GitHub রিলিজ থেকে (`https://github.com/<repo>/releases/download/…`)
এবং GitHub প্রতিটি রিলিজ ফাইলের জন্য যে SHA-256 প্রকাশ করে তার সঙ্গে যাচাই করা হয়। যে ফাইলের তা নেই সেটি
ইনস্টল হয়, কিন্তু **যাচাই করা হয়নি** হিসেবে চিহ্নিত হয়।

| রিলিজ ফাইল | যা ঘটে |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | লঞ্চারের `apps/<id>` ফোল্ডারে আনপ্যাক হয়। আপডেট পাশেই আনপ্যাক হয় এবং সম্পূর্ণ হলে তবেই বদলানো হয়, তাই ব্যর্থ আপডেটে পুরোনো সংস্করণ কাজ করতে থাকে |
| `.AppImage`, একক `.exe` | `apps/<id>`-এ একটি এক্সিকিউটেবল হিসেবে রাখা হয় |
| `.msi`, `.exe` ইনস্টলার, `.dmg`, `.pkg`, `.deb`, `.rpm` | সিস্টেম ইনস্টলারের হাতে দেওয়া হয়। এটি খুঁজে খুলতে ক্যাটালগের `launch` পাথ ব্যবহার হয় |

টার্মিনাল ও কমান্ড-লাইন অ্যাপ `bin` ফোল্ডারে একটি লঞ্চারও পায় (Windows-এ একটি `.cmd` ফাইল, অন্যত্র একটি
symlink)। **সেটিংস → আমার PATH-এ যোগ করুন** Windows-এ সেই ফোল্ডারটি আপনার PATH-এ যোগ করে; macOS ও Linux-এ
সেটিংস আপনার শেল প্রোফাইলে যোগ করার লাইনটি দেখায়। **টার্মিনালে খুলুন** এমন একটি টার্মিনাল খোলে যার PATH-এ
ফোল্ডারটি আগে থেকেই আছে।

সবকিছু একটি ডেটা ফোল্ডারে থাকে: Windows-এ `%LOCALAPPDATA%\CraftCraft\CraftCraft\data`, macOS-এ
`~/Library/Application Support/io.CraftCraft.CraftCraft`, Linux-এ `~/.local/share/craftcraft`, অথবা সেট করা থাকলে
`$CRAFTCRAFT_HOME`। আপনি **ইনস্টল করুন** বা **আপডেট করুন** ক্লিক না করা পর্যন্ত কিছুই ডাউনলোড হয় না।
Windows-এ ইনস্টলার চলে যাওয়ার পরে মুছে ফেলা হয়, এবং MSI-এর লগ সফল হওয়ার পরে; macOS ও Linux-এ, যেখানে লঞ্চার
দায়িত্ব হস্তান্তরের সময় সিস্টেম ইনস্টলার তখনও খোলা থাকে, সেগুলো এক সপ্তাহ পরে মুছে যায়। ডিবাগ মোড সবগুলো রেখে দেয়।

## বিল্ড ও চালানো

আপনার Rust 1.90 বা তার নতুন সংস্করণ লাগবে। Linux-এ আগে উইন্ডোইং হেডারগুলো ইনস্টল করুন (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`)।

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| এনভায়রনমেন্ট ভেরিয়েবল | ব্যবহার |
|---|---|
| `CRAFTCRAFT_FEED` | **শুধু ডিবাগ বিল্ডে:** অন্য কোনো ঠিকানা বা লোকাল ফাইল থেকে ক্যাটালগ পড়ে, যেমন সম্পাদনার সময় `catalog/catalog.json`। রিলিজ বিল্ড এটি উপেক্ষা করে |
| `CRAFTCRAFT_HOME` | অন্য একটি ডেটা ফোল্ডার ব্যবহার করে (একেবারে নতুন অবস্থা থেকে পরীক্ষার জন্য সুবিধাজনক) |
| `GITHUB_TOKEN` | GitHub-এর সীমা ঘণ্টায় 60 থেকে 5,000 অনুরোধে বাড়ায় |

`craftcraft --check-catalog <catalog.json>` একটি ক্যাটালগ ঠিক লঞ্চারের মতো করেই পার্স করে এবং যা কিছু বাদ পড়বে
তার তালিকা দেখায়।

## রিলিজ

`v0.2.0`-এর মতো একটি ট্যাগ পুশ করলে (`Cargo.toml`-এর `version`-এর সঙ্গে মিলিয়ে)
[release.yml](.github/workflows/release.yml) চলে। এটি নিজ নিজ রানারে Windows (x86_64, Arm), macOS (একটি ইউনিভার্সাল
`.app`, `.dmg` হিসেবেও) ও Linux (x86_64, Arm) বিল্ড করে, পরীক্ষার জন্য প্রতিটি বিল্ড ইনস্টল স্ক্রিপ্ট দিয়ে ইনস্টল
করে, সাইন করা Windows বিল্ডগুলোকে `.msi` ইনস্টলার হিসেবে প্যাকেজ করে সেগুলোও পরীক্ষা করে, এবং প্যাকেজগুলো ও একটি
`SHA256SUMS` ফাইলসহ একটি GitHub রিলিজ প্রকাশ করে। ওয়ার্কফ্লো হাতে চালালে প্রকাশ ছাড়া সবকিছুই হয়: পরীক্ষার জন্য
বিল্ডগুলো রানের সঙ্গে সংযুক্ত থাকে। রিলিজের পরে ক্যাটালগে `launcher.version` বাড়ান, যাতে পুরোনো লঞ্চারগুলো আপডেটের
প্রস্তাব দেয়।

ইউজাররা যেভাবে পান সেভাবে লোকালি একটি প্যাকেজ বিল্ড করে চেষ্টা করতে:

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

### সাইনিং

রিপোজিটরির **Settings → Secrets and variables → Actions**-এ এগুলো সেট করা হলে রিলিজ ওয়ার্কফ্লো বিল্ডগুলো সাইন
করে। ততদিন পর্যন্ত এটি বিল্ডগুলো সাইন ছাড়াই প্রকাশ করে।

**macOS (Developer ID, তারপর notarization)।** পাঁচটিই সিক্রেট:

| সিক্রেট | এটি কী |
|---|---|
| `APPLE_CERTIFICATE` | প্রাইভেট কী-সহ একটি **Developer ID Application** সার্টিফিকেট, Keychain Access থেকে `.p12` হিসেবে এক্সপোর্ট করা, তারপর base64-এ এনকোড করা (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | `.p12`-কে আপনি যে পাসওয়ার্ড দিয়েছেন |
| `APPLE_API_KEY` | একটি App Store Connect API কী (Users and Access → Integrations → Team Keys, role Developer): `.p8` ফাইলটি, base64-এ এনকোড করা |
| `APPLE_API_KEY_ID` | সেই কী-এর ID |
| `APPLE_API_ISSUER` | কীগুলোর ওপরে দেখানো Issuer ID |

শুধু প্রথম দুটি থাকলে অ্যাপটি সাইন হয় কিন্তু notarize হয় না, এবং ব্রাউজারে ডাউনলোড করলে Gatekeeper তখনও সতর্ক
করে। লোকালি সাইন করতে, `scripts/dist.sh` চালানোর আগে `APPLE_SIGNING_IDENTITY` সেট করুন (এবং `APPLE_API_KEY_*`
ভেরিয়েবলগুলো, `.p8`-কে নির্দেশ করে `APPLE_API_KEY_PATH`-সহ)।

**Windows (Microsoft Artifact Signing, আগে Trusted Signing নামে পরিচিত)।** কোনো সিক্রেট লাগে না: ওয়ার্কফ্লো
GitHub-এর OpenID Connect দিয়ে Azure-এ সাইন ইন করে, যাকে Azure শুধু এই রিপোজিটরির `release` এনভায়রনমেন্ট
থেকেই বিশ্বাস করে।

1. Azure portal-এ একটি **Artifact Signing account** তৈরি করুন (Basic tier যথেষ্ট) এবং এর এন্ডপয়েন্টটি টুকে
   রাখুন, যেমন `https://eus.codesigning.azure.net/`।
2. অ্যাকাউন্টে **identity validation** (Public) সম্পূর্ণ করুন, তারপর তা থেকে Public Trust ধরনের একটি
   **certificate profile** তৈরি করুন।
3. Microsoft Entra ID-তে একটি অ্যাপ রেজিস্টার করুন। Certificates & secrets → Federated credentials-এর অধীনে
   GitHub Actions-এর জন্য একটি যোগ করুন: organization `haruny`, repository `craftcraft`, entity type
   **Environment**, name `release`।
4. সাইনিং অ্যাকাউন্টে সেই অ্যাপটিকে **Artifact Signing Certificate Profile Signer** role দিন
   (Access control → Add role assignment)।
5. GitHub-এ `release` এনভায়রনমেন্টটি তৈরি করুন (Settings → Environments)। এটিকে `main` ব্রাঞ্চ ও `v*` ট্যাগে
   সীমিত করুন, যাতে শুধু রিভিউ করা কোডই সাইন হয়।
6. এই রিপোজিটরি ভেরিয়েবলগুলো যোগ করুন:

| ভেরিয়েবল | মান |
|---|---|
| `AZURE_CLIENT_ID` | অ্যাপ রেজিস্ট্রেশনের Application (client) ID |
| `AZURE_TENANT_ID` | এর Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | যে সাবস্ক্রিপশনে সাইনিং অ্যাকাউন্টটি আছে |
| `ARTIFACT_SIGNING_ENDPOINT` | ধাপ 1-এর এন্ডপয়েন্ট |
| `ARTIFACT_SIGNING_ACCOUNT` | সাইনিং অ্যাকাউন্টের নাম |
| `ARTIFACT_SIGNING_PROFILE` | certificate profile-এর নাম |

তারপর থেকে রিলিজ ওয়ার্কফ্লোর প্রতিটি রান দুটি Windows zip-এই, x86_64 ও Arm, `craftcraft.exe` সাইন করে, এবং zip
প্রকাশের আগে প্রতিটি সিগনেচার ও টাইমস্ট্যাম্প চেক করে। হাতে চালানো রানও সাইন করে, তাই ট্যাগ করার আগেই আপনি এটি
চেষ্টা করে দেখতে পারেন।

## গোপনীয়তা

পূর্ণ বিবরণ [Privacy_Statement.md](Privacy_Statement.md)-এ আছে। সংক্ষেপে:

- **গণনা।** রিলিজ বিল্ড CraftCraft-এর নিজস্ব এন্ডপয়েন্টের মাধ্যমে ক্যাটালগ আনে, যা লঞ্চার সংস্করণ, অপারেটিং সিস্টেম
  ও তার সংস্করণ, প্রসেসর ও দেশ অনুযায়ী প্রতিদিনের ক্যাটালগ চেক গণনা করে। এটি কোনো IP ঠিকানা বা আইডি রাখে না।
  এভাবেই আমরা জানি CraftCraft কোন কম্পিউটারগুলোতে চলে।
- **ব্যবহারের পরিসংখ্যান**, যা আপনি বন্ধ করতে পারেন। চালু হওয়া, এবং আপনি কোন অ্যাপ ইনস্টল, আপডেট, খোলেন ও সরান,
  একটি র‍্যান্ডম আইডির অধীনে পাঠানো হয় যা আপনার বা আপনার কম্পিউটারের সঙ্গে যুক্ত নয়। এ সম্পর্কে নোটিশ আপনি না দেখা
  পর্যন্ত কিছুই পাঠানো হয় না। সেখানে বা সেটিংস > গোপনীয়তা-তে এটি বন্ধ করুন।
- `DO_NOT_TRACK=1` বা `CRAFTCRAFT_NO_TELEMETRY=1` দুটোই বন্ধ করে: তখন ক্যাটালগ সরাসরি GitHub Pages থেকে আসে।

সোর্স থেকে করা বিল্ডে এর কোনোটিই নেই ([telemetry/README.md](telemetry/README.md) দেখুন)। এছাড়া CraftCraft শুধু
`haruny.github.io` ও `api.github.com` থেকে রিলিজের তথ্য, ক্যাটালগে তালিকাভুক্ত অ্যাপ আইকন, এবং `github.com` থেকে আপনি
যে রিলিজ ফাইলগুলো ইনস্টল করতে বেছে নেন সেগুলো আনতে সংযোগ করে। যেকোনো ওয়েব অনুরোধের মতোই ওই সার্ভিসগুলো আপনার IP
ঠিকানা দেখতে পায়।

## কাঠামো

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

## স্পনসর

CraftCraft বিনামূল্যে, এবং আমার অবসর সময়ে তৈরি। এটি যদি আপনার এক-দুটি লাইসেন্স ফি বাঁচায়, তাহলে আপনি
[GitHub Sponsors](https://github.com/sponsors/haruny)-এর মাধ্যমে এটিকে সমর্থন করতে পারেন।

## লাইসেন্স

MIT OR Apache-2.0। বান্ডেল করা Geist ফন্টগুলো (`assets/fonts`) SIL Open Font License-এর অধীনে
(`assets/fonts/OFL.txt`)। লঞ্চার যে প্রতিটি অ্যাপ ইনস্টল করে সেটি তার নিজস্ব লাইসেন্সের অধীনে।
