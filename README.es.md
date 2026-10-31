# CraftCraft

[English](README.md) · [Français](README.fr.md) · **Español** · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · [বাংলা](README.bn.md) · [Português](README.pt.md)

*Esto es una traducción; el [README en inglés](README.md) es la referencia.*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**Software conocido, reescrito en Rust: un solo lanzador para instalarlo, actualizarlo y abrirlo.**

PhotoCraft en lugar de Photoshop, GridCraft en lugar de Excel, BIMCraft en lugar de Revit, RustDesk
en lugar de TeamViewer. CraftCraft es una pequeña app nativa (Rust y egui, sin Electron) que muestra
alternativas de código abierto escritas en Rust para el software de pago que ya usas, instala la compilación adecuada para tu equipo, comprueba cada descarga
con su SHA-256 publicado y te avisa cuando hay una actualización o una app nueva en el catálogo.

## Instalación

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

El script descarga la última versión para tu sistema desde
[Releases](https://github.com/haruny/craftcraft/releases), comprueba su SHA-256, la instala solo
para tu usuario (sin permisos de administrador) y la abre. Vuelve a ejecutarlo para actualizar.

| | Se instala en | Además |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Acceso directo en el menú Inicio; aparece en Configuración → Aplicaciones, desde donde se desinstala; un icono en la bandeja mientras se ejecuta |
| macOS | `~/Applications/CraftCraft.app` | Una sola compilación para Apple silicon e Intel |
| Linux | `~/.local/bin/craftcraft` | Entrada e icono en el menú de aplicaciones |

O descarga un instalador desde [Releases](https://github.com/haruny/craftcraft/releases/latest):

- **Windows**: `craftcraft-windows-x86_64.msi` (o `-aarch64.msi` para Arm). Pregunta para quién es
  CraftCraft: **Only for me** lo instala igual que el script (sin permisos de administrador), **Everyone**
  lo instala en `%ProgramFiles%\CraftCraft` y en el menú Inicio de todos los usuarios, después de que
  Windows pida la aprobación de un administrador. Luego ofrece abrir CraftCraft. Ejecuta uno más
  reciente para actualizar; tiene que hacer la misma elección que la copia a la que sustituye
  (desinstala esa copia para cambiar). Sin su ventana (`msiexec /i … /qn`) instala para el usuario
  actual, o para todos con `ALLUSERS=1`. En una copia solo para ti, el script y el instalador pueden
  sustituirse mutuamente: el último que se ejecutó es el dueño de la instalación, y Configuración →
  Aplicaciones la muestra una sola vez.
- **macOS**: `craftcraft-macos-universal.dmg`. Ábrelo y arrastra CraftCraft a Aplicaciones.

En Windows y macOS, **Ajustes** también tiene:

- **Abrir al iniciar sesión**: CraftCraft se inicia en la bandeja (la barra de menús en macOS) al
  iniciar sesión, como `craftcraft --background`. Windows lo ejecuta desde el valor `CraftCraft` en
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`; macOS, desde
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist`.
- **Minimizar a la bandeja al cerrar** (activado por defecto): al cerrar la ventana, CraftCraft sigue
  en la bandeja buscando actualizaciones; la información sobre herramientas del icono las cuenta. En
  Windows, haz clic en el icono para abrir la ventana, o clic derecho para buscar ahora o salir. En
  macOS, haz clic en él para ver el mismo menú; mientras la ventana está cerrada no hay icono en el
  Dock, y abrir CraftCraft desde el Finder vuelve a mostrar la ventana. ⌘Q sigue cerrando la app.

Solo se ejecuta una copia: abrir CraftCraft de nuevo trae de vuelta la ventana de la que está en la
bandeja. En Windows, una copia que no instaló el script ni el instalador (descomprimida del zip, por
ejemplo) se añade sola al menú Inicio.

CraftCraft se actualiza solo. Cuando el catálogo indica que hay una versión más reciente, la
descarga de [Releases](https://github.com/haruny/craftcraft/releases), comprueba el SHA-256 que
GitHub publica para ella y ofrece **Reiniciar para actualizar**. Si está esperando en la bandeja,
se reinicia por sí solo y vuelve a la bandeja. Una copia instalada con el `.msi` se actualiza con el
nuevo `.msi` (para todos, solo cuando haces clic, porque Windows pide la aprobación de un
administrador). Las demás copias cambian el archivo viejo por el nuevo en el mismo sitio. Con la
comprobación automática desactivada, espera a que hagas clic en **Actualizar**. Una copia en una
carpeta en la que CraftCraft no puede escribir solo avisa de que hay una versión nueva y enlaza a ella.

**Ajustes → Modo de depuración** (en todas las plataformas) registra lo que hace CraftCraft en
`logs/craftcraft.log`, dentro de la carpeta de datos, y conserva los instaladores descargados y sus
registros en lugar de borrarlos.

## Idiomas

CraftCraft está en inglés salvo que elijas otro idioma en **Ajustes → Idioma**: Français,
Español, 中文（简体）, हिन्दी, العربية, বাংলা o Português. El cambio se aplica al momento, menú de la
bandeja incluido. El texto que viene del catálogo (lemas y descripciones de las apps, categorías,
avisos) se queda en inglés, igual que los detalles de error del sistema.

Las traducciones están en [`locales/`](locales), un archivo JSON por idioma, integrado en el
lanzador; lo que le falte a una traducción se muestra en inglés. Son nuevas, así que las
correcciones de hablantes nativos son bienvenidas: edita el archivo y ejecuta `cargo test`, que
comprueba que cada archivo tenga las mismas claves y marcadores de posición que `en.json`.

El chino, el hindi, el árabe y el bengalí necesitan caracteres que la fuente Geist incluida no
tiene, así que el lanzador usa una fuente que ya esté en el equipo: Microsoft YaHei, Nirmala UI y
Segoe UI en Windows; PingFang o Hiragino, Kohinoor y SF Arabic o Geeza Pro en macOS; Noto en Linux
(Debian/Ubuntu: `fonts-noto-cjk` y `fonts-noto-core`). Si no hay ninguna, Ajustes lo indica. El texto
en árabe se lee de derecha a izquierda, pero la disposición de la ventana sigue siendo de izquierda a
derecha.

Los README en otros idiomas son traducciones del README en inglés, que es la referencia.

## El catálogo lo dirige todo

La lista de apps no está integrada en el lanzador. Viene de
[`catalog/catalog.json`](catalog/catalog.json) en este repositorio, que se publica en
`https://haruny.github.io/craftcraft/catalog.json` cada vez que cambia en `main`. Tanto el
lanzador como el sitio web lo leen. El lanzador lo descarga:

- al iniciar, enviando el último ETag para que un catálogo sin cambios cueste una única solicitud mínima;
- cada pocas horas mientras se ejecuta (8 por defecto; **Ajustes** permite de 1 a 168), y reintenta 15 minutos después de un fallo;
- cuando pulsas **Buscar actualizaciones**.

Guarda la última copia válida, así que se abre al instante y funciona sin conexión. Cuando el catálogo cambia:

| Cambio en `catalog.json` | Lo que muestra el lanzador |
|---|---|
| Una app nueva en `apps` | Un aviso de «nuevo en el catálogo» y una insignia NUEVA, hasta que el usuario pulsa **Entendido** |
| `launcher.version` más reciente que esta compilación | Un aviso de actualización con un enlace de descarga |
| Una entrada nueva en `notices` | Un aviso hasta que se descarta |
| Un proyecto publica una versión | **Actualizar** en la tarjeta de esa app |
| Una app figura en `featured.json` | Encabeza el orden Destacadas por defecto, con una insignia DESTACADA |

**Las versiones vienen de un archivo, no de la API de GitHub.** Cada hora, el flujo de trabajo del
catálogo reúne la última versión de cada app (archivos y SHA-256), sus estrellas y su licencia en
`https://haruny.github.io/craftcraft/releases.json`. El lanzador lo descarga con un ETag junto al
catálogo, así que buscar actualizaciones de todas las apps cuesta una sola solicitud, por largo que
sea el catálogo, y nadie choca con el límite de GitHub de 60 solicitudes a la API por hora. Solo una
app que el índice aún no cubre (un catálogo local en pruebas, por ejemplo) se consulta directamente
en GitHub. Las estrellas y las fechas de publicación también alimentan el menú **Ordenar**:
Destacadas, Más populares, Nombre y Actualizadas recientemente.

**Para añadir una app, envía un pull request que la añada a `catalog/catalog.json`**, o indica a un
agente de IA [ADD_APP.md](ADD_APP.md) junto con el repositorio de GitHub de la app.
[CONTRIBUTING.md](CONTRIBUTING.md) tiene las reglas: la app debe reemplazar software de pago y ser
de código abierto en github.com.

La dirección del catálogo es fija: las compilaciones de versión siempre leen la publicada, y no hay
ningún ajuste para cambiarla.

## Apps que ya tienes

El lanzador busca cada app del catálogo que ya esté en el equipo al iniciar, después de cada
consulta del catálogo y después de cada instalación o desinstalación. Una app encontrada así aparece
como instalada con su versión, recibe **Abrir** y recibe **Actualizar** cuando su proyecto publica
una versión más reciente.

| Dónde | Cómo se encuentra | Versión obtenida de |
|---|---|---|
| Apps de escritorio de Windows | La lista de programas instalados (las entradas Uninstall del registro), con coincidencia estricta por nombre: `PhotoCraft`, `PhotoCraft 0.16.0` y `DesignCraft (Craft Apps)` coinciden, `ArtCraft Launcher` no coincide con ArtCraft | `DisplayVersion` |
| Apps de escritorio de macOS | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | El `Info.plist` del paquete |
| Apps de escritorio de Linux | La ruta `launch` del catálogo, el comando en el PATH o una AppImage en `~/Applications` | dpkg o rpm, o el nombre de archivo de la AppImage |
| Apps de línea de comandos y de terminal (en todas partes) | El comando en el PATH | Ejecutando `<command> --version` (oculto, como máximo 3 segundos) |

Actualizar una app instalada con su propio instalador ejecuta el instalador de la nueva versión, que
la actualiza en el sitio. Los paquetes MSI se ejecutan con `/passive`, y con `ALLUSERS=1` cuando la
copia instalada es para todos, así que Windows pide la aprobación de un administrador; los propios
registros de Windows Installer indican qué copias son esas, ya que también lista en HKLM las copias
instaladas para un solo usuario. Una app de línea de comandos instalada de otra forma (winget, cargo,
un script) no se puede reemplazar, así que **Instalar la última** pone una copia actualizada en la
carpeta del lanzador. **Desinstalar…** ejecuta el desinstalador propio de la app.

Cuando falla una instalación, actualización, desinstalación o **Abrir**, la tarjeta indica qué paso
falló y por qué, por ejemplo *Error del instalador: This app is already installed for everyone…*,
con el texto completo al pasar el cursor. Para un paquete MSI, el motivo viene del registro de
Windows Installer, que se abre con **⋯ → Registro del instalador**; **⋯ → Copiar error** lo copia
todo.

`craftcraft --detect <catalog.json>` muestra lo que encuentra el lanzador.

## Cómo se instalan las apps

Los archivos vienen solo de la propia versión de GitHub de cada app (`https://github.com/<repo>/releases/download/…`)
y se verifican con el SHA-256 que GitHub publica para cada archivo de la versión. Un archivo sin él
se instala, pero se marca como **sin verificar**.

| Archivo de la versión | Qué ocurre |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | Se descomprime en la carpeta `apps/<id>` del lanzador. Las actualizaciones se descomprimen al lado y solo se sustituyen cuando están completas, así que una actualización fallida deja la versión anterior funcionando |
| `.AppImage`, `.exe` único | Se guarda como un solo ejecutable en `apps/<id>` |
| Instalador `.msi`, `.exe`, `.dmg`, `.pkg`, `.deb`, `.rpm` | Se entrega al instalador del sistema. La ruta `launch` del catálogo se usa para encontrarla y abrirla |

Las apps de terminal y de línea de comandos también reciben un lanzador en la carpeta `bin` (un
archivo `.cmd` en Windows, un enlace simbólico en los demás sistemas). **Ajustes → Añadirla a mi PATH**
pone esa carpeta en tu PATH en Windows; en macOS y Linux, Ajustes muestra la línea que debes añadir
al perfil de tu shell. **Abrir en terminal** abre un terminal con la carpeta ya en su PATH.

Todo está en una sola carpeta de datos: `%LOCALAPPDATA%\CraftCraft\CraftCraft\data` en Windows,
`~/Library/Application Support/io.CraftCraft.CraftCraft` en macOS, `~/.local/share/craftcraft` en
Linux, o `$CRAFTCRAFT_HOME` si está definida. No se descarga nada hasta que pulsas **Instalar** o
**Actualizar**. En Windows, los instaladores se borran una vez ejecutados, y el registro de un MSI
cuando tiene éxito; en macOS y Linux, donde el instalador del sistema sigue abierto cuando el
lanzador le pasa el relevo, se borran al cabo de una semana. El modo de depuración los conserva
todos.

## Compilar y ejecutar

Necesitas Rust 1.90 o posterior. En Linux, instala primero las cabeceras del sistema de ventanas
(Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| Variable de entorno | Uso |
|---|---|
| `CRAFTCRAFT_FEED` | **Solo compilaciones de depuración:** lee el catálogo desde otra dirección o un archivo local, p. ej. `catalog/catalog.json` mientras lo editas. Las compilaciones de versión la ignoran |
| `CRAFTCRAFT_HOME` | Usa otra carpeta de datos (práctico para probar desde cero) |
| `GITHUB_TOKEN` | Sube el límite de GitHub de 60 a 5000 solicitudes por hora |

`craftcraft --check-catalog <catalog.json>` analiza un catálogo exactamente como lo hará el lanzador
y lista todo lo que omitiría.

## Versiones

Publicar una etiqueta como `v0.2.0` (que coincida con `version` en `Cargo.toml`) ejecuta
[release.yml](.github/workflows/release.yml). Compila para Windows (x86_64, Arm), macOS (una `.app`
universal, también como `.dmg`) y Linux (x86_64, Arm) en sus propios runners, instala cada
compilación con el script de instalación para probarla, empaqueta las compilaciones firmadas de
Windows como instaladores `.msi` y también los prueba, y publica una versión en GitHub con los
paquetes y un archivo `SHA256SUMS`. Ejecutar el flujo de trabajo a mano lo hace todo salvo publicar:
las compilaciones se adjuntan a la ejecución, para pruebas. Su último trabajo pone entonces `launcher.version` del catálogo en la
nueva versión y publica el catálogo, que es como los lanzadores en ejecución se enteran de la
versión.

Para compilar y probar un paquete localmente, tal como lo reciben los usuarios:

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

### Firma

El flujo de trabajo de versiones firma las compilaciones una vez que estos valores están definidos en
**Settings → Secrets and variables → Actions** del repositorio. Hasta entonces, publica las
compilaciones sin firmar.

**macOS (Developer ID y después notarización).** Los cinco son secretos:

| Secreto | Qué es |
|---|---|
| `APPLE_CERTIFICATE` | Un certificado **Developer ID Application** con su clave privada, exportado desde Acceso a Llaveros como `.p12` y luego codificado en base64 (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | La contraseña que le pusiste al `.p12` |
| `APPLE_API_KEY` | Una clave de API de App Store Connect (Users and Access → Integrations → Team Keys, rol Developer): el archivo `.p8`, codificado en base64 |
| `APPLE_API_KEY_ID` | El ID de esa clave |
| `APPLE_API_ISSUER` | El Issuer ID que aparece encima de las claves |

Con solo los dos primeros, la app queda firmada pero no notarizada, y Gatekeeper sigue avisando
cuando se descarga desde un navegador. Para firmar localmente, define `APPLE_SIGNING_IDENTITY` (y
las variables `APPLE_API_KEY_*`, con `APPLE_API_KEY_PATH` apuntando al `.p8`) antes de ejecutar
`scripts/dist.sh`.

**Windows (Microsoft Artifact Signing, antes Trusted Signing).** Sin secretos: el flujo de trabajo
inicia sesión en Azure con OpenID Connect de GitHub, en el que Azure solo confía desde el entorno
`release` de este repositorio.

1. En Azure portal, crea una **Artifact Signing account** (el nivel Basic basta) y anota su
   endpoint, p. ej. `https://eus.codesigning.azure.net/`.
2. En la cuenta, completa la **identity validation** (Public) y luego crea a partir de ella un
   **certificate profile** de tipo Public Trust.
3. En Microsoft Entra ID, registra una app. En Certificates & secrets → Federated credentials,
   añade una para GitHub Actions: organización `haruny`, repositorio `craftcraft`, entity type
   **Environment**, nombre `release`.
4. En la cuenta de firma, asigna a esa app el rol **Artifact Signing Certificate Profile Signer**
   (Access control → Add role assignment).
5. En GitHub, crea el entorno `release` (Settings → Environments). Limítalo a la rama `main` y a las
   etiquetas `v*`, para que solo se firme código revisado.
6. Añade estas variables de repositorio:

| Variable | Valor |
|---|---|
| `AZURE_CLIENT_ID` | El Application (client) ID del registro de la app |
| `AZURE_TENANT_ID` | Su Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | La suscripción en la que está la cuenta de firma |
| `ARTIFACT_SIGNING_ENDPOINT` | El endpoint del paso 1 |
| `ARTIFACT_SIGNING_ACCOUNT` | El nombre de la cuenta de firma |
| `ARTIFACT_SIGNING_PROFILE` | El nombre del certificate profile |

A partir de entonces, cada ejecución del flujo de trabajo de versiones firma `craftcraft.exe` en
ambos zips de Windows, x86_64 y Arm, y comprueba cada firma y marca de tiempo antes de publicar los
zips. Una ejecución iniciada a mano también firma, así que puedes probarlo antes de etiquetar.

## Privacidad

Todos los detalles están en [Privacy_Statement.md](Privacy_Statement.md). En resumen:

- **Recuentos.** Las compilaciones de versión descargan el catálogo a través del propio endpoint de
  CraftCraft, que cuenta las consultas del catálogo por día según la versión del lanzador, el sistema
  operativo y su versión, el procesador y el país. No guarda ninguna dirección IP ni id. Así sabemos
  en qué equipos se ejecuta CraftCraft.
- **Estadísticas de uso**, que puedes desactivar. Los inicios, y qué apps instalas, actualizas, abres
  y eliminas, enviados bajo un id aleatorio que no está vinculado a ti ni a tu equipo. No se envía
  nada hasta que hayas visto el aviso al respecto. Desactívalas ahí o en Ajustes > Privacidad.
- `DO_NOT_TRACK=1` o `CRAFTCRAFT_NO_TELEMETRY=1` desactiva ambos: el catálogo llega entonces
  directamente de GitHub Pages.

Una compilación desde el código fuente no tiene ninguno de los dos (consulta
[telemetry/README.md](telemetry/README.md)). Por lo demás, CraftCraft solo se conecta para obtener
información de versiones de `haruny.github.io` y `api.github.com`, los iconos de apps que lista el
catálogo y los archivos de versión que eliges instalar desde `github.com`. Esos servicios ven tu
dirección IP, como en cualquier solicitud web.

## Estructura

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

## Patrocinio

CraftCraft es gratuito y lo desarrollo en mi tiempo libre. Si te ahorra una o dos licencias, puedes
apoyarlo a través de [GitHub Sponsors](https://github.com/sponsors/haruny).

## Licencia

MIT OR Apache-2.0. Las fuentes Geist incluidas (`assets/fonts`) están bajo la SIL Open Font License
(`assets/fonts/OFL.txt`). Cada app que instala el lanzador tiene su propia licencia.
