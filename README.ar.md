<div dir="rtl">

# CraftCraft

[English](README.md) · [Français](README.fr.md) · [Español](README.es.md) · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · **العربية** · [বাংলা](README.bn.md) · [Português](README.pt.md)

*هذه ترجمة، والمرجع المعتمد هو [الملف README باللغة الإنجليزية](README.md).*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**برامج معروفة أُعيدت كتابتها بلغة Rust: مشغّل واحد لتثبيتها وتحديثها وفتحها.**

PhotoCraft بدلًا من Photoshop، وGridCraft بدلًا من Excel، وBIMCraft بدلًا من Revit، وRustDesk
بدلًا من TeamViewer. إن CraftCraft تطبيق أصلي صغير (مكتوب بـ Rust وegui، دون Electron) يعرض
بدائل مفتوحة المصدر مكتوبة بـ Rust للبرامج المدفوعة التي تستخدمها، ويثبّت الإصدار المناسب لجهازك، ويتحقق من كل تنزيل
مقابل قيمة SHA-256 المنشورة له، ويُعلمك عند توفر تحديث أو تطبيق جديد في الكتالوج.

## التثبيت

</div>

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

<div dir="rtl">

ينزّل البرنامج النصي أحدث إصدار لنظامك من
[Releases](https://github.com/haruny/craftcraft/releases)، ويتحقق من قيمة SHA-256 الخاصة به، ويثبّته
لمستخدمك فقط (دون صلاحيات المسؤول)، ثم يفتحه. شغّله مرة أخرى للتحديث.

| | مكان التثبيت | إضافةً إلى ذلك |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | اختصار في قائمة «ابدأ»؛ يظهر في الإعدادات ← التطبيقات، ومنها يمكن إلغاء تثبيته؛ أيقونة في علبة النظام أثناء تشغيله |
| macOS | `~/Applications/CraftCraft.app` | إصدار واحد لمعالجات Apple silicon وIntel |
| Linux | `~/.local/bin/craftcraft` | إدخال وأيقونة في قائمة التطبيقات |

أو نزّل مثبّتًا من [Releases](https://github.com/haruny/craftcraft/releases/latest):

- **Windows**: الملف `craftcraft-windows-x86_64.msi` (أو `-aarch64.msi` لمعالجات Arm). يسألك المثبّت لمن
  يُثبَّت CraftCraft: الخيار **Only for me** يثبّته بالطريقة نفسها التي يتبعها البرنامج النصي (دون صلاحيات المسؤول)، والخيار **Everyone**
  يثبّته في `%ProgramFiles%\CraftCraft` وفي قائمة «ابدأ» لكل مستخدم، بعد أن يطلب Windows
  موافقة المسؤول. ثم يعرض فتح CraftCraft. شغّل مثبّتًا أحدث للتحديث؛ ويجب
  أن يتخذ الخيار نفسه الذي اتُّخذ للنسخة التي يحل محلها (ألغِ تثبيت تلك النسخة للتبديل). وعند تشغيله دون
  نافذته (`msiexec /i … /qn`) يثبّته للمستخدم الحالي، أو لكل المستخدمين مع `ALLUSERS=1`.
  وبالنسبة إلى النسخة المثبّتة لك وحدك، يمكن للبرنامج النصي والمثبّت أن يحل كل منهما محل الآخر:
  فآخر ما شُغّل منهما يملك التثبيت، وتعرضه الإعدادات ← التطبيقات مرة واحدة.
- **macOS**: الملف `craftcraft-macos-universal.dmg`. افتحه واسحب CraftCraft إلى مجلد Applications.

على Windows وmacOS، تتضمن **الإعدادات** أيضًا:

- **البدء عند تسجيل الدخول**: يبدأ CraftCraft في علبة النظام (شريط القوائم على macOS) عند تسجيل الدخول، بالأمر
  `craftcraft --background`. يشغّله Windows من القيمة `CraftCraft` ضمن
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`، ويشغّله macOS من
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist`.
- **التصغير إلى علبة النظام عند الإغلاق** (مفعّل افتراضيًا): إغلاق النافذة يُبقي CraftCraft في علبة النظام،
  مستمرًا في التحقق من وجود تحديثات، ويعرض تلميح الأيقونة عددها. على Windows، انقر الأيقونة لفتح
  النافذة، أو انقر بزر الماوس الأيمن للتحقق الآن أو للإنهاء. على macOS، انقرها لعرض القائمة نفسها؛ وعندما
  تكون النافذة مغلقة لا تظهر أيقونة في Dock، وفتح CraftCraft من Finder يعيد النافذة.
  ولا يزال ⌘Q ينهي التطبيق.

تعمل نسخة واحدة فقط: فتح CraftCraft مرة أخرى يعيد نافذة النسخة الموجودة في علبة النظام. وعلى
Windows، تضيف النسخة التي لم يثبّتها البرنامج النصي أو المثبّت (كأن تكون مفكوكة من ملف zip)
نفسها إلى قائمة «ابدأ».

يحدّث CraftCraft نفسه. عندما يشير الكتالوج إلى صدور إصدار أحدث، ينزّله من
[Releases](https://github.com/haruny/craftcraft/releases)، ويتحقق من SHA-256 الذي ينشره GitHub له،
ثم يعرض **إعادة التشغيل للتحديث**. وإذا كان ينتظر في علبة النظام، فإنه يعيد تشغيل نفسه ويعود إليها.
النسخة المثبّتة بملف `.msi` تتحدّث بملف `.msi` الجديد (وللنسخة المثبّتة لكل المستخدمين، فقط عندما
تنقر، لأن Windows يطلب موافقة المسؤول). أما النسخ الأخرى فتضع الملف الجديد مكان القديم. وعند إيقاف
التحقق التلقائي، ينتظر أن تنقر **تحديث**. والنسخة الموجودة في مجلد لا يستطيع CraftCraft الكتابة فيه
تكتفي بالإشارة إلى وجود إصدار جديد مع رابط إليه.

**الإعدادات ← وضع تصحيح الأخطاء** (على جميع الأنظمة) يسجّل ما يفعله CraftCraft في `logs/craftcraft.log` داخل مجلد
البيانات، ويحتفظ بملفات التثبيت التي نُزّلت وسجلاتها بدلًا من حذفها.

## اللغات

يعمل CraftCraft باللغة الإنجليزية ما لم تختر لغة أخرى في **الإعدادات ← اللغة**: Français،
Español، 中文（简体）، हिन्दी، العربية، বাংলা أو Português. يُطبَّق التغيير فورًا، بما في ذلك
قائمة علبة النظام. أما النص الوارد من الكتالوج (الشعارات النصية للتطبيقات وأوصافها، والفئات،
والإشعارات) فيبقى باللغة الإنجليزية، وكذلك تفاصيل الأخطاء الواردة من النظام.

توجد الترجمات في [`locales/`](locales)، ملف JSON واحد لكل لغة، مضمّنة في
المشغّل؛ وكل ما ينقص الترجمة يُعرض بالإنجليزية. هذه الترجمات جديدة، لذا نرحب بتصحيحات
المتحدثين الأصليين: عدّل الملف وشغّل `cargo test`، الذي يتحقق من أن كل ملف يحتوي على
المفاتيح والعناصر النائبة نفسها الموجودة في `en.json`.

تحتاج الصينية والهندية والعربية والبنغالية إلى أحرف لا يتضمنها خط Geist المرفق، لذا
يستخدم المشغّل خطًا موجودًا على الكمبيوتر: Microsoft YaHei وNirmala UI وSegoe UI على Windows؛
وPingFang أو Hiragino، وKohinoor، وSF Arabic أو Geeza Pro على macOS؛ وNoto على Linux (Debian/Ubuntu:
`fonts-noto-cjk` و`fonts-noto-core`). وإن لم يوجد أي منها، تُشير الإعدادات إلى ذلك. يُقرأ النص العربي من اليمين
إلى اليسار، لكن تخطيط النافذة يبقى من اليسار إلى اليمين.

ملفات README باللغات الأخرى ترجمات لهذا الملف، وهو المرجع المعتمد.

## الكتالوج يحرّك كل شيء

قائمة التطبيقات ليست مضمّنة في المشغّل. بل تأتي من
[`catalog/catalog.json`](catalog/catalog.json) في هذا المستودع، والذي يُنشر على
`https://haruny.github.io/craftcraft/catalog.json` كلما تغيّر على `main`. ويقرؤه كل من
المشغّل وموقع الويب. ويجلبه المشغّل:

- عند بدء التشغيل، مرسلًا آخر ETag بحيث لا يكلّف الكتالوج غير المتغيّر سوى طلب صغير واحد؛
- كل بضع ساعات أثناء تشغيله (8 افتراضيًا؛ ويمكن ضبطها من 1 إلى 168 في **الإعدادات**)، مع إعادة المحاولة بعد 15 دقيقة من أي فشل؛
- عند الضغط على **التحقق من وجود تحديثات**.

ويحتفظ بآخر نسخة سليمة، لذا يفتح فورًا ويعمل دون اتصال. وعندما يتغيّر الكتالوج:

| التغيير في `catalog.json` | ما يعرضه المشغّل |
|---|---|
| تطبيق جديد في `apps` | شريط «جديد في الكتالوج» وشارة «جديد»، إلى أن ينقر المستخدم **فهمت** |
| `launcher.version` أحدث من هذا الإصدار | شريط تحديث مع رابط تنزيل |
| إدخال جديد في `notices` | شريط يبقى إلى أن يُتجاهل |
| مشروع ينشر إصدارًا | زر **تحديث** على بطاقة ذلك التطبيق |
| تطبيق مُدرج في `featured.json` | يتصدّر ترتيب «المميزة» الافتراضي، مع شارة «مميز» |

**تأتي الإصدارات من ملف واحد، لا من واجهة GitHub البرمجية.** كل ساعة، يجمع سير عمل الكتالوج
أحدث إصدار لكل تطبيق (الملفات وقيم SHA-256 الخاصة بها)، وعدد النجوم والترخيص في
`https://haruny.github.io/craftcraft/releases.json`. ويجلب المشغّل هذا الملف مع ETag إلى جانب
الكتالوج، فلا يكلّف التحقق من تحديثات كل التطبيقات سوى طلب واحد، مهما طال الكتالوج،
ولا يصطدم أحد بحد GitHub البالغ 60 طلبًا للواجهة البرمجية في الساعة. ولا يُبحث مباشرةً على GitHub إلا عن التطبيق الذي لا يغطيه
الفهرس بعد (ككتالوج محلي قيد الاختبار مثلًا). وتُستخدم النجوم وتواريخ الإصدارات
أيضًا في قائمة **الفرز**: المميزة، والأكثر شيوعًا، والاسم، والمحدّثة مؤخرًا.

**لإضافة تطبيق، أرسل طلب سحب (pull request) يضيفه إلى `catalog/catalog.json`**، أو وجّه
وكيل ذكاء اصطناعي إلى [ADD_APP.md](ADD_APP.md) مع مستودع GitHub الخاص بالتطبيق.
يتضمن [CONTRIBUTING.md](CONTRIBUTING.md) القواعد: يجب أن يحل التطبيق محل برنامج مدفوع، وأن يكون مفتوح
المصدر على github.com.

عنوان الكتالوج ثابت: إصدارات الإنتاج تقرأ دائمًا العنوان المنشور، ولا يوجد إعداد
لتغييره.

## التطبيقات الموجودة لديك بالفعل

يبحث المشغّل عن كل تطبيق من الكتالوج موجود بالفعل على الكمبيوتر، عند بدء التشغيل، وبعد كل تحقق من
الكتالوج، وبعد كل تثبيت أو إلغاء تثبيت. والتطبيق الذي يُعثر عليه بهذه الطريقة يظهر مثبّتًا مع
إصداره، ويحصل على زر **فتح**، وعلى زر **تحديث** عندما ينشر مشروعه إصدارًا أحدث.

| المكان | طريقة العثور عليه | مصدر الإصدار |
|---|---|---|
| تطبيقات سطح المكتب على Windows | قائمة البرامج المثبّتة (إدخالات Uninstall في السجل)، مع مطابقة صارمة للاسم: `PhotoCraft` و`PhotoCraft 0.16.0` و`DesignCraft (Craft Apps)` تتطابق، بينما `ArtCraft Launcher` لا يطابق ArtCraft | `DisplayVersion` |
| تطبيقات سطح المكتب على macOS | `/Applications/<Name>.app`، `~/Applications/<Name>.app` | الملف `Info.plist` الخاص بالحزمة |
| تطبيقات سطح المكتب على Linux | مسار `launch` في الكتالوج، أو الأمر الموجود في PATH، أو ملف AppImage في `~/Applications` | dpkg أو rpm، أو اسم ملف AppImage |
| تطبيقات سطر الأوامر والوحدة الطرفية (على جميع الأنظمة) | الأمر الموجود في PATH | تشغيل `<command> --version` (بشكل مخفي، 3 ثوانٍ كحد أقصى) |

تحديث تطبيق ثبّته مثبّته الخاص يشغّل مثبّت الإصدار الجديد، الذي يرقّيه
في مكانه. تُشغَّل حزم MSI مع `/passive`، ومع `ALLUSERS=1` عندما تكون النسخة المثبّتة
لكل المستخدمين، فيطلب Windows موافقة المسؤول؛ وتحدد سجلات Windows Installer نفسها
أي النسخ هي كذلك، لأنه يُدرج النسخ المثبّتة لمستخدم واحد فقط ضمن HKLM أيضًا. أما تطبيق سطر الأوامر
المثبّت بطريقة أخرى (winget أو cargo أو برنامج نصي) فلا يمكن استبداله، لذا يضع **تثبيت أحدث إصدار**
نسخة محدّثة في مجلد المشغّل بدلًا من ذلك. ويشغّل **إلغاء التثبيت…** أداة إلغاء التثبيت
الخاصة بالتطبيق.

عندما يفشل تثبيت أو تحديث أو إلغاء تثبيت أو **فتح**، تذكر البطاقة الخطوة التي فشلت وسبب ذلك،
مثل *فشل المثبّت: هذا التطبيق مثبّت بالفعل لكل المستخدمين…*، مع النص الكامل عند التمرير فوقها.
وبالنسبة إلى حزمة MSI، يأتي السبب من سجل Windows Installer، الذي يفتحه **⋯ ← سجل المثبّت**؛
و**⋯ ← نسخ الخطأ** ينسخه كله.

يطبع الأمر `craftcraft --detect <catalog.json>` ما يعثر عليه المشغّل.

## كيف تُثبَّت التطبيقات

تأتي الملفات فقط من إصدار GitHub الخاص بكل تطبيق (`https://github.com/<repo>/releases/download/…`)
ويُتحقق منها مقابل قيمة SHA-256 التي ينشرها GitHub لكل ملف إصدار. والملف الذي ليست له قيمة
يُثبَّت لكن يُوسم بأنه **غير متحقق منه**.

| ملف الإصدار | ما يحدث |
|---|---|
| `.zip`، `.tar.gz`، `.tar.xz` | يُفك في المجلد `apps/<id>` الخاص بالمشغّل. وتُفك التحديثات بجانبه ولا تحل محله إلا عند اكتمالها، لذا يترك التحديث الفاشل الإصدار القديم يعمل |
| `.AppImage`، ملف `.exe` منفرد | يُحفظ كملف تنفيذي واحد في `apps/<id>` |
| مثبّت `.msi` أو `.exe`، `.dmg`، `.pkg`، `.deb`، `.rpm` | يُسلَّم إلى مثبّت النظام. ويُستخدم مسار `launch` في الكتالوج للعثور عليه وفتحه |

تحصل تطبيقات الوحدة الطرفية وسطر الأوامر أيضًا على مشغّل في المجلد `bin` (ملف `.cmd` على Windows،
ورابط رمزي في غيره). يضع **الإعدادات ← إضافة المجلد إلى PATH** ذلك المجلد في PATH على Windows؛ وعلى
macOS وLinux، تعرض الإعدادات السطر الذي يجب إضافته إلى ملف تعريف الواجهة (shell). ويفتح **فتح في الوحدة الطرفية**
وحدة طرفية يكون المجلد موجودًا بالفعل في PATH الخاص بها.

كل شيء موجود في مجلد بيانات واحد: `%LOCALAPPDATA%\CraftCraft\CraftCraft\data` على Windows،
و`~/Library/Application Support/io.CraftCraft.CraftCraft` على macOS، و`~/.local/share/craftcraft` على
Linux، أو `$CRAFTCRAFT_HOME` عند تعيينه. لا يُنزَّل أي شيء حتى تنقر **تثبيت** أو
**تحديث**. على Windows، تُحذف ملفات التثبيت بعد تشغيلها، ويُحذف سجل MSI بعد نجاحه؛
وعلى macOS وLinux، حيث يظل مثبّت النظام مفتوحًا عند تسليم المشغّل له، تُحذف
بعد أسبوع. ويحتفظ وضع تصحيح الأخطاء بها كلها.

## البناء والتشغيل

تحتاج إلى Rust 1.90 أو أحدث. على Linux، ثبّت ترويسات نظام النوافذ أولًا (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

</div>

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

<div dir="rtl">

| متغير البيئة | الاستخدام |
|---|---|
| `CRAFTCRAFT_FEED` | **في إصدارات التصحيح فقط:** قراءة الكتالوج من عنوان آخر أو من ملف محلي، مثل `catalog/catalog.json` أثناء تعديله. تتجاهله إصدارات الإنتاج |
| `CRAFTCRAFT_HOME` | استخدام مجلد بيانات آخر (مفيد للاختبار من بداية نظيفة) |
| `GITHUB_TOKEN` | يرفع حد GitHub من 60 إلى 5000 طلب في الساعة |

يحلّل الأمر `craftcraft --check-catalog <catalog.json>` الكتالوج تمامًا كما سيفعل المشغّل، ويسرد
كل ما سيتخطاه.

## الإصدارات

دفع وسم مثل `v0.2.0` (مطابق لـ `version` في `Cargo.toml`) يشغّل
[release.yml](.github/workflows/release.yml). فيبني إصدارات Windows (x86_64 وArm)، وmacOS (ملف `.app` عالمي
واحد، وأيضًا بصيغة `.dmg`) وLinux (x86_64 وArm) على المشغّلات الخاصة بكل منها، ويثبّت كل إصدار
بالبرنامج النصي للتثبيت لاختباره، ويحزم إصدارات Windows الموقّعة كمثبّتات `.msi` ويختبرها
أيضًا، وينشر إصدار GitHub يتضمن الحزم وملف `SHA256SUMS`. وتشغيل
سير العمل يدويًا يفعل كل شيء ما عدا النشر: تُرفق الإصدارات بعملية التشغيل لأغراض الاختبار.
ثم تضبط آخر مهمة فيه قيمة `launcher.version` في الكتالوج على الإصدار الجديد وتنشر الكتالوج،
وبهذا تعرف المشغّلات العاملة بصدور الإصدار.

لبناء حزمة وتجربتها محليًا، بالطريقة التي يحصل عليها المستخدمون:

</div>

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

<div dir="rtl">

### التوقيع

يوقّع سير عمل الإصدار الإصدارات بمجرد تعيين ما يلي في **Settings → Secrets
and variables → Actions** الخاصة بالمستودع. وحتى ذلك الحين، ينشر الإصدارات دون توقيع.

**macOS (Developer ID، ثم التوثيق notarization).** العناصر الخمسة كلها أسرار (secrets):

| السر | ما هو |
|---|---|
| `APPLE_CERTIFICATE` | شهادة **Developer ID Application** مع مفتاحها الخاص، مُصدَّرة من Keychain Access بصيغة `.p12`، ثم مرمّزة بـ base64 (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | كلمة المرور التي عيّنتها للملف `.p12` |
| `APPLE_API_KEY` | مفتاح App Store Connect API (Users and Access → Integrations → Team Keys، بدور Developer): الملف `.p8`، مرمّزًا بـ base64 |
| `APPLE_API_KEY_ID` | معرّف ذلك المفتاح |
| `APPLE_API_ISSUER` | قيمة Issuer ID المعروضة فوق المفاتيح |

مع أول سرين فقط، يُوقَّع التطبيق لكن لا يُوثَّق، ويظل Gatekeeper يُحذّر عند
تنزيله في متصفح. للتوقيع محليًا، عيّن `APPLE_SIGNING_IDENTITY` (ومتغيرات `APPLE_API_KEY_*`،
مع `APPLE_API_KEY_PATH` مشيرًا إلى الملف `.p8`) قبل تشغيل `scripts/dist.sh`.

**Windows (Microsoft Artifact Signing، المعروف سابقًا باسم Trusted Signing).** لا توجد أسرار: يسجّل سير العمل الدخول
إلى Azure باستخدام OpenID Connect من GitHub، الذي لا يثق به Azure إلا إذا جاء من البيئة `release`
في هذا المستودع.

1. في Azure portal، أنشئ **Artifact Signing account** (المستوى Basic كافٍ) ودوّن
   نقطة النهاية الخاصة به، مثل `https://eus.codesigning.azure.net/`.
2. في الحساب، أكمل **identity validation** (Public)، ثم أنشئ منها **certificate profile**
   من النوع Public Trust.
3. في Microsoft Entra ID، سجّل تطبيقًا. ضمن Certificates & secrets → Federated credentials،
   أضف بيانات اعتماد لـ GitHub Actions: المؤسسة `haruny`، والمستودع `craftcraft`، ونوع الكيان
   **Environment**، والاسم `release`.
4. على حساب التوقيع، امنح ذلك التطبيق الدور **Artifact Signing Certificate Profile Signer**
   (Access control → Add role assignment).
5. في GitHub، أنشئ البيئة `release` (Settings → Environments). وقصِرها على الفرع `main`
   والوسوم `v*`، كي لا يُوقَّع إلا الكود الذي رُوجع.
6. أضف متغيرات المستودع التالية:

| المتغير | القيمة |
|---|---|
| `AZURE_CLIENT_ID` | قيمة Application (client) ID لتسجيل التطبيق |
| `AZURE_TENANT_ID` | قيمة Directory (tenant) ID الخاصة به |
| `AZURE_SUBSCRIPTION_ID` | الاشتراك الذي يوجد فيه حساب التوقيع |
| `ARTIFACT_SIGNING_ENDPOINT` | نقطة النهاية من الخطوة 1 |
| `ARTIFACT_SIGNING_ACCOUNT` | اسم حساب التوقيع |
| `ARTIFACT_SIGNING_PROFILE` | اسم ملف تعريف الشهادة |

ومنذ ذلك الحين، يوقّع كل تشغيل لسير عمل الإصدار الملف `craftcraft.exe` في ملفي zip الخاصين بـ Windows،
x86_64 وArm، ويتحقق من كل توقيع وطابع زمني قبل نشر ملفات zip. والتشغيل
الذي يبدأ يدويًا يوقّع أيضًا، لذا يمكنك تجربته قبل إنشاء الوسم.

## الخصوصية

التفاصيل الكاملة موجودة في [Privacy_Statement.md](Privacy_Statement.md). باختصار:

- **الإحصاءات العددية.** تجلب إصدارات الإنتاج الكتالوج عبر نقطة النهاية الخاصة بـ CraftCraft، والتي تعدّ
  عمليات التحقق من الكتالوج يوميًا حسب إصدار المشغّل، ونظام التشغيل وإصداره، والمعالج والبلد.
  ولا تحتفظ بأي عنوان IP أو معرّف. وبهذا نعرف على أي أجهزة كمبيوتر يعمل CraftCraft.
- **إحصاءات الاستخدام**، ويمكنك إيقافها. مرات التشغيل، والتطبيقات التي تثبّتها وتحدّثها وتفتحها
  وتزيلها، تُرسل تحت معرّف عشوائي غير مرتبط بك أو بجهازك. لا يُرسل أي شيء
  قبل أن ترى الإشعار الخاص بها. أوقفها من هناك أو من الإعدادات ← الخصوصية.
- يوقف `DO_NOT_TRACK=1` أو `CRAFTCRAFT_NO_TELEMETRY=1` كليهما: وعندئذٍ يأتي الكتالوج
  مباشرةً من GitHub Pages.

الإصدار المبني من المصدر لا يتضمن أيًا منهما (راجع [telemetry/README.md](telemetry/README.md)). وفيما عدا ذلك
لا يتصل CraftCraft إلا لجلب معلومات الإصدارات من `haruny.github.io` و`api.github.com`،
وأيقونات التطبيقات التي يسردها الكتالوج، وملفات الإصدارات التي تختار تثبيتها من `github.com`.
وترى هذه الخدمات عنوان IP الخاص بك، كما في أي طلب ويب.

## بنية المشروع

</div>

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

<div dir="rtl">

## الرعاية

CraftCraft مجاني وأبنيه في وقت فراغي. إن وفّر عليك رسوم ترخيص أو اثنتين، يمكنك
دعمه عبر [GitHub Sponsors](https://github.com/sponsors/haruny).

## الترخيص

MIT OR Apache-2.0. خطوط Geist المرفقة (`assets/fonts`) مرخّصة بموجب SIL Open Font License
(`assets/fonts/OFL.txt`). وكل تطبيق يثبّته المشغّل يخضع لترخيصه الخاص.

</div>
