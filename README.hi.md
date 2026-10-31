# CraftCraft

[English](README.md) · [Français](README.fr.md) · [Español](README.es.md) · [中文](README.zh-CN.md) · **हिन्दी** · [العربية](README.ar.md) · [বাংলা](README.bn.md) · [Português](README.pt.md)

*यह एक अनुवाद है; संदर्भ के लिए [अंग्रेज़ी README](README.md) ही मान्य है।*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**जाने-माने सॉफ़्टवेयर, Rust में दोबारा लिखे गए: उन्हें इंस्टॉल, अपडेट करने और खोलने के लिए एक लॉन्चर।**

Photoshop की जगह PhotoCraft, Excel की जगह GridCraft, Revit की जगह BIMCraft, TeamViewer की जगह
RustDesk। CraftCraft एक छोटा नेटिव ऐप है (Rust और egui, Electron नहीं) जो उन पेड सॉफ़्टवेयर के
ओपन-सोर्स Rust विकल्पों की सूची दिखाता है जिन्हें आप पहले से इस्तेमाल करते हैं, आपके कंप्यूटर के लिए सही
बिल्ड इंस्टॉल करता है, हर डाउनलोड को उसके प्रकाशित SHA-256 से जाँचता है, और कैटलॉग में कोई अपडेट या नया ऐप
आने पर आपको बताता है।

## इंस्टॉल करें

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

स्क्रिप्ट आपके सिस्टम के लिए नवीनतम रिलीज़ [Releases](https://github.com/haruny/craftcraft/releases)
से डाउनलोड करती है, उसका SHA-256 जाँचती है, उसे सिर्फ़ आपके यूज़र के लिए इंस्टॉल करती है (व्यवस्थापक अधिकारों
के बिना), और उसे खोलती है। अपडेट करने के लिए इसे फिर से चलाएँ।

| | कहाँ इंस्टॉल होता है | और भी |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Start मेन्यू शॉर्टकट; Settings → Apps में सूचीबद्ध, जहाँ से इसे अनइंस्टॉल किया जा सकता है; चलते समय ट्रे आइकन |
| macOS | `~/Applications/CraftCraft.app` | Apple silicon और Intel के लिए एक ही बिल्ड |
| Linux | `~/.local/bin/craftcraft` | Applications मेन्यू एंट्री और आइकन |

या [Releases](https://github.com/haruny/craftcraft/releases/latest) से इंस्टॉलर डाउनलोड करें:

- **Windows**: `craftcraft-windows-x86_64.msi` (या Arm के लिए `-aarch64.msi`)। यह पूछता है कि CraftCraft
  किसके लिए है: **Only for me** इसे स्क्रिप्ट की तरह ही इंस्टॉल करता है (व्यवस्थापक अधिकारों के बिना),
  **Everyone** इसे `%ProgramFiles%\CraftCraft` में और हर यूज़र के Start मेन्यू में इंस्टॉल करता है, जब
  Windows किसी व्यवस्थापक की अनुमति माँग लेता है। फिर यह CraftCraft खोलने का विकल्प देता है। अपडेट करने
  के लिए नया इंस्टॉलर चलाएँ; उसे वही विकल्प चुनना होगा जो उस कॉपी का है जिसकी जगह वह ले रहा है (बदलने के
  लिए उस कॉपी को अनइंस्टॉल करें)। विंडो के बिना (`msiexec /i … /qn`) यह मौजूदा यूज़र के लिए इंस्टॉल होता
  है, या `ALLUSERS=1` के साथ सभी के लिए। सिर्फ़ आपके लिए इंस्टॉल की गई कॉपी में स्क्रिप्ट और इंस्टॉलर एक-दूसरे
  की जगह ले सकते हैं: जो आख़िर में चला, इंस्टॉल उसी का होता है, और Settings → Apps उसे एक ही बार दिखाता है।
- **macOS**: `craftcraft-macos-universal.dmg`। इसे खोलें और CraftCraft को Applications में ड्रैग करें।

Windows और macOS पर, **सेटिंग्स** में ये भी हैं:

- **लॉग ऑन पर शुरू करें**: साइन इन करने पर CraftCraft ट्रे में (macOS पर मेन्यू बार में)
  `craftcraft --background` के रूप में शुरू होता है। Windows इसे
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` के अंतर्गत `CraftCraft` वैल्यू से चलाता है; macOS
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist` से।
- **बंद करने पर ट्रे में छोटा करें** (डिफ़ॉल्ट रूप से चालू): विंडो बंद करने पर CraftCraft ट्रे में रहता है
  और अपडेट जाँचता रहता है; आइकन का टूलटिप उनकी गिनती दिखाता है। Windows पर, विंडो खोलने के लिए आइकन पर
  क्लिक करें, या अभी जाँचने या बंद करने के लिए उस पर राइट-क्लिक करें। macOS पर, यही मेन्यू पाने के लिए उस
  पर क्लिक करें; विंडो बंद रहने पर Dock आइकन नहीं होता, और Finder से CraftCraft खोलने पर विंडो वापस आ जाती
  है। ⌘Q अब भी ऐप बंद करता है।

सिर्फ़ एक कॉपी चलती है: CraftCraft को फिर से खोलने पर ट्रे वाली कॉपी की विंडो वापस आ जाती है। Windows
पर, जो कॉपी स्क्रिप्ट या इंस्टॉलर से इंस्टॉल नहीं हुई (मान लीजिए zip से अनपैक की गई), वह ख़ुद को Start
मेन्यू में जोड़ लेती है।

**सेटिंग्स → डीबग मोड** (हर जगह) CraftCraft जो करता है उसे डेटा फ़ोल्डर में `logs/craftcraft.log` में लॉग
करता है, और डाउनलोड किए गए इंस्टॉलर और उनके लॉग हटाने के बजाय रखता है।

## भाषाएँ

CraftCraft अंग्रेज़ी में रहता है, जब तक आप **सेटिंग्स → भाषा** में कोई दूसरी भाषा न चुनें: Français,
Español, 中文（简体）, हिन्दी, العربية, বাংলা या Português। बदलाव तुरंत लागू होता है, ट्रे मेन्यू
समेत। कैटलॉग से आने वाला टेक्स्ट (ऐप्स की टैगलाइन और विवरण, श्रेणियाँ, सूचनाएँ) अंग्रेज़ी में ही रहता है,
और सिस्टम से आने वाले त्रुटि विवरण भी।

अनुवाद [`locales/`](locales) में हैं, हर भाषा के लिए एक JSON फ़ाइल, जो लॉन्चर में ही बिल्ट-इन है; किसी
अनुवाद में जो कुछ छूटा हो, वह अंग्रेज़ी में दिखता है। ये नए हैं, इसलिए मूल भाषियों के सुधारों का स्वागत है:
फ़ाइल एडिट करें और `cargo test` चलाएँ, जो जाँचता है कि हर फ़ाइल में `en.json` जैसी ही keys और placeholders
हैं।

चीनी, हिंदी, अरबी और बांग्ला के लिए ऐसे अक्षर चाहिए जो साथ आने वाले Geist फ़ॉन्ट में नहीं हैं, इसलिए लॉन्चर
कंप्यूटर पर पहले से मौजूद फ़ॉन्ट इस्तेमाल करता है: Windows पर Microsoft YaHei, Nirmala UI और Segoe UI;
macOS पर PingFang या Hiragino, Kohinoor और SF Arabic या Geeza Pro; Linux पर Noto (Debian/Ubuntu:
`fonts-noto-cjk` और `fonts-noto-core`)। अगर कोई नहीं है, तो सेटिंग्स यह बताती है। अरबी टेक्स्ट दाएँ से
बाएँ पढ़ा जाता है, लेकिन विंडो का लेआउट बाएँ से दाएँ ही रहता है।

दूसरी भाषाओं के README इसी (अंग्रेज़ी) README के अनुवाद हैं, जो संदर्भ है।

## सब कुछ कैटलॉग से चलता है

ऐप्स की सूची लॉन्चर में बिल्ट-इन नहीं है। यह इस रिपॉज़िटरी की
[`catalog/catalog.json`](catalog/catalog.json) से आती है, जो `main` पर बदलने पर हर बार
`https://haruny.github.io/craftcraft/catalog.json` पर प्रकाशित होती है। लॉन्चर और वेबसाइट दोनों इसे पढ़ते
हैं। लॉन्चर इसे लाता है:

- शुरू होने पर, आख़िरी ETag भेजकर, ताकि बिना बदले कैटलॉग पर बस एक छोटा-सा अनुरोध लगे;
- चलते समय हर कुछ घंटों में (डिफ़ॉल्ट रूप से 8; **सेटिंग्स** में 1–168 सेट करें), विफलता के 15 मिनट बाद फिर से कोशिश करते हुए;
- जब आप **अपडेट जाँचें** दबाते हैं।

यह आख़िरी सही कॉपी रखता है, इसलिए तुरंत खुलता है और ऑफ़लाइन भी काम करता है। जब कैटलॉग बदलता है:

| `catalog.json` में बदलाव | लॉन्चर क्या दिखाता है |
|---|---|
| `apps` में नया ऐप | "कैटलॉग में नया" बैनर और नया बैज, जब तक यूज़र **समझ गए** पर क्लिक न करे |
| `launcher.version` इस बिल्ड से नया | डाउनलोड लिंक के साथ अपडेट बैनर |
| `notices` में नई एंट्री | हटाए जाने तक एक बैनर |
| कोई प्रोजेक्ट रिलीज़ प्रकाशित करता है | उस ऐप के कार्ड पर **अपडेट करें** |
| कोई ऐप `featured.json` में सूचीबद्ध है | वह डिफ़ॉल्ट "विशेष" क्रम में सबसे आगे रहता है, विशेष बैज के साथ |

**रिलीज़ एक फ़ाइल से आती हैं, GitHub की API से नहीं।** हर घंटे कैटलॉग वर्कफ़्लो हर ऐप की नवीनतम रिलीज़
(फ़ाइलें और SHA-256), स्टार और लाइसेंस को `https://haruny.github.io/craftcraft/releases.json` में इकट्ठा करता
है। लॉन्चर इसे कैटलॉग के साथ ETag के ज़रिए लाता है, इसलिए हर ऐप के अपडेट जाँचने में बस एक अनुरोध लगता है,
कैटलॉग चाहे जितना लंबा हो जाए, और कोई भी GitHub की 60 API अनुरोध प्रति घंटा की सीमा तक नहीं पहुँचता। सिर्फ़
वह ऐप जिसे इंडेक्स अभी कवर नहीं करता (मान लीजिए टेस्ट किया जा रहा कोई लोकल कैटलॉग), सीधे GitHub पर देखा
जाता है। स्टार और रिलीज़ की तारीखें **क्रम** मेन्यू को भी चलाती हैं: विशेष, सबसे लोकप्रिय, नाम और हाल ही
में अपडेट किए गए।

**कोई ऐप जोड़ने के लिए, एक pull request भेजें जो उसे `catalog/catalog.json` में जोड़े**, या किसी AI एजेंट को
ऐप की GitHub रिपॉज़िटरी के साथ [ADD_APP.md](ADD_APP.md) दें। [CONTRIBUTING.md](CONTRIBUTING.md) में नियम हैं:
ऐप को किसी पेड सॉफ़्टवेयर की जगह लेनी चाहिए, और github.com पर ओपन सोर्स होना चाहिए।

कैटलॉग का पता तय है: रिलीज़ बिल्ड हमेशा प्रकाशित पता ही पढ़ते हैं, और इसे बदलने की कोई सेटिंग नहीं है।

## आपके पास पहले से मौजूद ऐप्स

लॉन्चर कंप्यूटर पर पहले से मौजूद हर कैटलॉग ऐप को ढूँढता है: शुरू होने पर, हर कैटलॉग जाँच के बाद, और हर
इंस्टॉल या अनइंस्टॉल के बाद। इस तरह मिला ऐप अपने वर्शन के साथ इंस्टॉल दिखता है, उसे **खोलें** मिलता है, और
उसका प्रोजेक्ट नई रिलीज़ प्रकाशित करने पर **अपडेट करें** मिलता है।

| कहाँ | कैसे मिलता है | वर्शन कहाँ से |
|---|---|---|
| Windows डेस्कटॉप ऐप्स | इंस्टॉल किए गए प्रोग्रामों की सूची (रजिस्ट्री की Uninstall एंट्री), नाम से सख़्ती से मिलाकर: `PhotoCraft`, `PhotoCraft 0.16.0` और `DesignCraft (Craft Apps)` मेल खाते हैं, `ArtCraft Launcher` ArtCraft से मेल नहीं खाता | `DisplayVersion` |
| macOS डेस्कटॉप ऐप्स | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | बंडल की `Info.plist` |
| Linux डेस्कटॉप ऐप्स | कैटलॉग का `launch` पाथ, PATH पर मौजूद कमांड, या `~/Applications` में कोई AppImage | dpkg या rpm, या AppImage की फ़ाइल का नाम |
| कमांड-लाइन और टर्मिनल ऐप्स (हर जगह) | PATH पर मौजूद कमांड | `<command> --version` चलाकर (छिपा हुआ, अधिकतम 3 सेकंड) |

अपने ख़ुद के इंस्टॉलर से इंस्टॉल किए गए ऐप को अपडेट करने पर नई रिलीज़ का इंस्टॉलर चलता है, जो उसे उसी जगह
अपग्रेड करता है। MSI पैकेज `/passive` के साथ चलते हैं, और इंस्टॉल की गई कॉपी सभी के लिए होने पर
`ALLUSERS=1` के साथ, ताकि Windows किसी व्यवस्थापक की अनुमति माँगे; कौन-सी कॉपी ऐसी हैं, यह Windows Installer
के अपने रिकॉर्ड बताते हैं, क्योंकि वह सिर्फ़ एक यूज़र के लिए इंस्टॉल की गई कॉपी भी HKLM के अंतर्गत दिखाता है।
किसी और तरीके से (winget, cargo, कोई स्क्रिप्ट) इंस्टॉल किए गए कमांड-लाइन ऐप को बदला नहीं जा सकता, इसलिए
**नवीनतम इंस्टॉल करें** उसकी जगह लॉन्चर के फ़ोल्डर में एक अप-टू-डेट कॉपी रखता है। **अनइंस्टॉल करें…** ऐप
का अपना अनइंस्टॉलर चलाता है।

जब कोई इंस्टॉल, अपडेट, अनइंस्टॉल या **खोलें** विफल होता है, तो कार्ड बताता है कि कौन-सा चरण विफल हुआ और
क्यों, उदाहरण के लिए *इंस्टॉलर विफल: This app is already installed for everyone…*, और होवर करने पर पूरा
संदेश दिखता है। MSI पैकेज के लिए कारण Windows Installer के लॉग से आता है, जिसे **⋯ → इंस्टॉलर लॉग**
खोलता है; **⋯ → त्रुटि कॉपी करें** पूरा संदेश कॉपी करता है।

`craftcraft --detect <catalog.json>` दिखाता है कि लॉन्चर को क्या मिलता है।

## ऐप्स कैसे इंस्टॉल होते हैं

फ़ाइलें सिर्फ़ हर ऐप की अपनी GitHub रिलीज़ (`https://github.com/<repo>/releases/download/…`) से आती हैं, और
GitHub हर रिलीज़ फ़ाइल के लिए जो SHA-256 प्रकाशित करता है, उससे सत्यापित की जाती हैं। जिस फ़ाइल का SHA-256
नहीं है, वह इंस्टॉल तो होती है लेकिन **असत्यापित** चिह्नित होती है।

| रिलीज़ फ़ाइल | क्या होता है |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | लॉन्चर के `apps/<id>` फ़ोल्डर में अनपैक होती है। अपडेट उसके बगल में अनपैक होते हैं और पूरा होने पर ही उसकी जगह लेते हैं, इसलिए विफल अपडेट के बाद भी पुराना वर्शन काम करता रहता है |
| `.AppImage`, अकेली `.exe` | `apps/<id>` में एक एक्ज़िक्यूटेबल के रूप में रखी जाती है |
| `.msi`, `.exe` इंस्टॉलर, `.dmg`, `.pkg`, `.deb`, `.rpm` | सिस्टम इंस्टॉलर को सौंपी जाती है। उसे ढूँढने और खोलने के लिए कैटलॉग का `launch` पाथ इस्तेमाल होता है |

टर्मिनल और कमांड-लाइन ऐप्स को `bin` फ़ोल्डर में एक लॉन्चर भी मिलता है (Windows पर `.cmd` फ़ाइल, बाकी जगह
symlink)। **सेटिंग्स → इसे मेरे PATH में जोड़ें** Windows पर उस फ़ोल्डर को आपके PATH में जोड़ता है; macOS
और Linux पर, सेटिंग्स वह लाइन दिखाती है जो आपको अपनी शेल प्रोफ़ाइल में जोड़नी है। **टर्मिनल में खोलें** एक
ऐसा टर्मिनल खोलता है जिसके PATH में वह फ़ोल्डर पहले से हो।

सब कुछ एक डेटा फ़ोल्डर में रहता है: Windows पर `%LOCALAPPDATA%\CraftCraft\CraftCraft\data`, macOS पर
`~/Library/Application Support/io.CraftCraft.CraftCraft`, Linux पर `~/.local/share/craftcraft`, या सेट होने
पर `$CRAFTCRAFT_HOME`। जब तक आप **इंस्टॉल करें** या **अपडेट करें** पर क्लिक नहीं करते, कुछ भी डाउनलोड नहीं
होता। Windows पर, इंस्टॉलर चलने के बाद हटा दिए जाते हैं और MSI का लॉग सफल होने के बाद; macOS और Linux पर,
जहाँ लॉन्चर के सौंपने के समय सिस्टम इंस्टॉलर अभी खुला होता है, वे एक हफ़्ते बाद हटते हैं। डीबग मोड इन सबको
रखता है।

## बिल्ड करें और चलाएँ

आपको Rust 1.90 या नया चाहिए। Linux पर, पहले विंडोइंग हेडर इंस्टॉल करें (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`)।

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| एनवायरनमेंट वेरिएबल | उपयोग |
|---|---|
| `CRAFTCRAFT_FEED` | **सिर्फ़ डीबग बिल्ड:** कैटलॉग किसी दूसरे पते या लोकल फ़ाइल से पढ़ें, जैसे एडिट करते समय `catalog/catalog.json`। रिलीज़ बिल्ड इसे अनदेखा करते हैं |
| `CRAFTCRAFT_HOME` | कोई दूसरा डेटा फ़ोल्डर इस्तेमाल करें (बिल्कुल साफ़ शुरुआत से टेस्ट करने के लिए काम का) |
| `GITHUB_TOKEN` | GitHub की सीमा को 60 से बढ़ाकर 5,000 अनुरोध प्रति घंटा करता है |

`craftcraft --check-catalog <catalog.json>` किसी कैटलॉग को ठीक वैसे ही पार्स करता है जैसे लॉन्चर करेगा, और जो
कुछ वह छोड़ देगा उसकी सूची दिखाता है।

## रिलीज़

`v0.2.0` जैसा टैग पुश करने पर (`Cargo.toml` के `version` से मेल खाता हुआ)
[release.yml](.github/workflows/release.yml) चलता है। यह Windows (x86_64, Arm), macOS (एक universal `.app`,
`.dmg` के रूप में भी) और Linux (x86_64, Arm) को उनके अपने runners पर बिल्ड करता है, हर बिल्ड को टेस्ट करने
के लिए इंस्टॉल स्क्रिप्ट से इंस्टॉल करता है, साइन किए गए Windows बिल्ड को `.msi` इंस्टॉलर के रूप में पैकेज
करके उन्हें भी टेस्ट करता है, और पैकेज और एक `SHA256SUMS` फ़ाइल के साथ GitHub रिलीज़ प्रकाशित करता है।
वर्कफ़्लो को हाथ से चलाने पर प्रकाशित करने के अलावा सब कुछ होता है: टेस्टिंग के लिए बिल्ड उस रन के साथ जुड़
जाते हैं। रिलीज़ के बाद, कैटलॉग में `launcher.version` बढ़ाएँ ताकि पुराने लॉन्चर अपडेट ऑफ़र करें।

किसी पैकेज को लोकल रूप से वैसे ही बिल्ड करने और आज़माने के लिए, जैसे यूज़र उसे पाते हैं:

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

### साइनिंग

रिपॉज़िटरी की **Settings → Secrets and variables → Actions** में ये सेट हो जाने पर रिलीज़ वर्कफ़्लो बिल्ड को
साइन करता है। तब तक, यह बिल्ड को बिना साइन किए प्रकाशित करता है।

**macOS (Developer ID, फिर notarization)।** पाँचों secrets हैं:

| Secret | यह क्या है |
|---|---|
| `APPLE_CERTIFICATE` | अपनी private key के साथ एक **Developer ID Application** सर्टिफ़िकेट, Keychain Access से `.p12` के रूप में एक्सपोर्ट किया गया, फिर base64-encoded (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | वह पासवर्ड जो आपने `.p12` को दिया |
| `APPLE_API_KEY` | एक App Store Connect API key (Users and Access → Integrations → Team Keys, role Developer): `.p8` फ़ाइल, base64-encoded |
| `APPLE_API_KEY_ID` | उस key की ID |
| `APPLE_API_ISSUER` | keys के ऊपर दिखाई गई Issuer ID |

सिर्फ़ पहले दो के साथ, ऐप साइन तो होता है लेकिन notarized नहीं, और ब्राउज़र में डाउनलोड होने पर Gatekeeper
फिर भी चेतावनी देता है। लोकल रूप से साइन करने के लिए, `scripts/dist.sh` चलाने से पहले
`APPLE_SIGNING_IDENTITY` सेट करें (और `APPLE_API_KEY_*` वेरिएबल, जिनमें `APPLE_API_KEY_PATH` `.p8` की ओर
इशारा करे)।

**Windows (Microsoft Artifact Signing, पहले Trusted Signing)।** कोई secrets नहीं: वर्कफ़्लो GitHub के OpenID
Connect से Azure में साइन इन करता है, जिस पर Azure सिर्फ़ इस रिपॉज़िटरी के `release` एनवायरनमेंट से भरोसा
करता है।

1. Azure portal में, एक **Artifact Signing account** बनाएँ (Basic tier काफ़ी है) और उसका endpoint नोट करें,
   जैसे `https://eus.codesigning.azure.net/`।
2. अकाउंट में, **identity validation** (Public) पूरा करें, फिर उससे Public Trust प्रकार का एक **certificate
   profile** बनाएँ।
3. Microsoft Entra ID में, एक ऐप रजिस्टर करें। Certificates & secrets → Federated credentials के अंतर्गत,
   GitHub Actions के लिए एक जोड़ें: organization `haruny`, repository `craftcraft`, entity type
   **Environment**, name `release`।
4. साइनिंग अकाउंट पर, उस ऐप को **Artifact Signing Certificate Profile Signer** role दें
   (Access control → Add role assignment)।
5. GitHub में, `release` एनवायरनमेंट बनाएँ (Settings → Environments)। इसे `main` ब्रांच और `v*` टैग तक
   सीमित करें, ताकि सिर्फ़ रिव्यू किया गया कोड साइन हो।
6. ये repository variables जोड़ें:

| वेरिएबल | वैल्यू |
|---|---|
| `AZURE_CLIENT_ID` | app registration की Application (client) ID |
| `AZURE_TENANT_ID` | उसकी Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | वह subscription जिसमें साइनिंग अकाउंट है |
| `ARTIFACT_SIGNING_ENDPOINT` | चरण 1 वाला endpoint |
| `ARTIFACT_SIGNING_ACCOUNT` | साइनिंग अकाउंट का नाम |
| `ARTIFACT_SIGNING_PROFILE` | certificate profile का नाम |

तब से, रिलीज़ वर्कफ़्लो का हर रन दोनों Windows zips, x86_64 और Arm, में `craftcraft.exe` को साइन करता है, और
zips प्रकाशित होने से पहले हर सिग्नेचर और टाइमस्टैम्प जाँचता है। हाथ से शुरू किया गया रन भी साइन करता है,
ताकि आप टैग करने से पहले इसे आज़मा सकें।

## गोपनीयता

पूरी जानकारी [Privacy_Statement.md](Privacy_Statement.md) में है। संक्षेप में:

- **गिनती।** रिलीज़ बिल्ड कैटलॉग को CraftCraft के अपने endpoint के ज़रिए लाते हैं, जो प्रति दिन कैटलॉग
  जाँचों की गिनती लॉन्चर वर्शन, ऑपरेटिंग सिस्टम और उसके वर्शन, प्रोसेसर और देश के हिसाब से करता है। यह कोई
  IP पता या id नहीं रखता। इसी से हमें पता चलता है कि CraftCraft किन कंप्यूटरों पर चलता है।
- **उपयोग आँकड़े**, जिन्हें आप बंद कर सकते हैं। लॉन्च, और आप कौन-से ऐप्स इंस्टॉल, अपडेट करते, खोलते और
  हटाते हैं, एक रैंडम id के तहत भेजे जाते हैं जो आपसे या आपके कंप्यूटर से जुड़ी नहीं है। जब तक आप इसकी
  सूचना देख नहीं लेते, कुछ नहीं भेजा जाता। इसे वहीं या सेटिंग्स > गोपनीयता में बंद करें।
- `DO_NOT_TRACK=1` या `CRAFTCRAFT_NO_TELEMETRY=1` दोनों को बंद कर देता है: तब कैटलॉग सीधे GitHub Pages से
  आता है।

सोर्स से बने बिल्ड में इनमें से कुछ भी नहीं होता (देखें [telemetry/README.md](telemetry/README.md))। इसके
अलावा CraftCraft सिर्फ़ `haruny.github.io` और `api.github.com` से रिलीज़ की जानकारी, कैटलॉग में सूचीबद्ध ऐप
आइकन, और `github.com` से वे रिलीज़ फ़ाइलें लाने के लिए कनेक्ट होता है जिन्हें आप इंस्टॉल करना चुनते हैं। किसी
भी वेब अनुरोध की तरह, ये सेवाएँ आपका IP पता देखती हैं।

## संरचना

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

## स्पॉन्सर करें

CraftCraft मुफ़्त है और मेरे खाली समय में बनाया जाता है। अगर इससे आपकी एक-दो लाइसेंस फ़ीस बचती है, तो आप
[GitHub Sponsors](https://github.com/sponsors/haruny) के ज़रिए इसका समर्थन कर सकते हैं।

## लाइसेंस

MIT OR Apache-2.0। साथ आने वाले Geist फ़ॉन्ट (`assets/fonts`) SIL Open Font License
(`assets/fonts/OFL.txt`) के अंतर्गत हैं। लॉन्चर जो भी ऐप इंस्टॉल करता है, वह अपने ख़ुद के लाइसेंस के अंतर्गत है।
