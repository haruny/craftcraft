# CraftCraft

[English](README.md) · [Français](README.fr.md) · [Español](README.es.md) · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · [বাংলা](README.bn.md) · **Português**

*Este arquivo é uma tradução; o [README em inglês](README.md) é a referência.*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**Softwares conhecidos, reescritos em Rust: um só inicializador para instalar, atualizar e abrir todos eles.**

PhotoCraft em vez de Photoshop, GridCraft em vez de Excel, BIMCraft em vez de Revit, RustDesk
em vez de TeamViewer. O CraftCraft é um pequeno app nativo (Rust e egui, sem Electron) que lista
substitutos de código aberto em Rust para os softwares pagos que você já usa, instala a build certa para o seu computador, confere cada download
com o SHA-256 publicado e avisa quando há uma atualização ou um app novo no catálogo.

## Instalação

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

O script baixa a versão mais recente para o seu sistema em
[Releases](https://github.com/haruny/craftcraft/releases), confere o SHA-256, instala só para
o seu usuário (sem direitos de administrador) e abre o app. Execute-o de novo para atualizar.

| | Instalado em | Além disso |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Atalho no menu Iniciar; aparece em Configurações → Aplicativos, que o desinstala; um ícone na bandeja enquanto está em execução |
| macOS | `~/Applications/CraftCraft.app` | Uma única build para Apple silicon e Intel |
| Linux | `~/.local/bin/craftcraft` | Entrada e ícone no menu de aplicativos |

Ou baixe um instalador em [Releases](https://github.com/haruny/craftcraft/releases/latest):

- **Windows**: `craftcraft-windows-x86_64.msi` (ou `-aarch64.msi` para Arm). Ele pergunta para quem
  é o CraftCraft: **Only for me** (só para mim) instala do mesmo jeito que o script (sem direitos de
  administrador); **Everyone** (todos) instala em `%ProgramFiles%\CraftCraft` e no menu Iniciar de
  todos os usuários, depois que o Windows pede a aprovação de um administrador. Em seguida, oferece
  abrir o CraftCraft. Execute um mais novo para atualizar; ele precisa fazer a mesma escolha que a
  cópia que substitui (desinstale essa cópia para trocar). Sem a janela (`msiexec /i … /qn`), ele
  instala para o usuário atual, ou para todos com `ALLUSERS=1`.
  Para uma cópia só sua, o script e o instalador podem ocupar o lugar um do outro:
  o que foi executado por último é o dono da instalação, e Configurações → Aplicativos a lista uma vez só.
- **macOS**: `craftcraft-macos-universal.dmg`. Abra-o e arraste o CraftCraft para Aplicativos.

No Windows e no macOS, **Configurações** também tem:

- **Iniciar ao entrar**: o CraftCraft inicia na bandeja (na barra de menus no macOS) quando você entra,
  como `craftcraft --background`. O Windows o executa a partir do valor `CraftCraft` em
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`; o macOS, a partir de
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist`.
- **Minimizar para a bandeja ao fechar** (ativado por padrão): fechar a janela deixa o CraftCraft na
  bandeja, ainda verificando atualizações; a dica do ícone mostra quantas há. No Windows, clique no
  ícone para abrir a janela ou clique com o botão direito para verificar agora ou sair. No macOS,
  clique nele para ver o mesmo menu; com a janela fechada não há ícone no Dock, e abrir o CraftCraft
  pelo Finder traz a janela de volta. ⌘Q continua saindo do app.

Só uma cópia fica em execução: abrir o CraftCraft de novo traz de volta a janela da que está na
bandeja. No Windows, uma cópia que não foi instalada pelo script nem pelo instalador (descompactada
do zip, por exemplo) se adiciona ao menu Iniciar.

O CraftCraft se atualiza sozinho. Quando o catálogo diz que há uma versão mais nova, ele a baixa de
[Releases](https://github.com/haruny/craftcraft/releases), confere o SHA-256 que o GitHub publica
para ela e oferece **Reiniciar para atualizar**. Se estiver esperando na bandeja, ele reinicia por
conta própria e volta para a bandeja. Uma cópia instalada com o `.msi` é atualizada com o novo
`.msi` (para todos, só quando você clica, já que o Windows pede a aprovação de um administrador). As
outras cópias trocam o arquivo antigo pelo novo no mesmo lugar. Com a verificação automática
desligada, ele espera você clicar em **Atualizar**. Uma cópia em uma pasta onde o CraftCraft não
consegue gravar apenas avisa que há uma versão nova e mostra o link para ela.

**Configurações → Modo de depuração** (em todos os sistemas) registra o que o CraftCraft faz em
`logs/craftcraft.log` na pasta de dados e mantém os instaladores baixados e os logs deles em vez de
apagá-los.

## Idiomas

O CraftCraft fica em inglês, a menos que você escolha outro idioma em **Configurações → Idioma**:
Français, Español, 中文（简体）, हिन्दी, العربية, বাংলা ou Português. A mudança vale na hora,
inclusive no menu da bandeja. Os textos que vêm do catálogo (slogans e descrições dos apps,
categorias, avisos) ficam em inglês, assim como os detalhes de erro do sistema.

As traduções ficam em [`locales/`](locales), um arquivo JSON por idioma, embutidos no
inicializador; o que faltar em uma tradução aparece em inglês. Elas são novas, então correções de
falantes nativos são bem-vindas: edite o arquivo e execute `cargo test`, que confere se cada arquivo
tem as mesmas chaves e os mesmos placeholders que `en.json`.

Chinês, hindi, árabe e bengali precisam de letras que a fonte Geist embutida não tem, então o
inicializador usa uma fonte que já está no computador: Microsoft YaHei, Nirmala UI e Segoe UI no
Windows; PingFang ou Hiragino, Kohinoor e SF Arabic ou Geeza Pro no macOS; Noto no Linux
(Debian/Ubuntu: `fonts-noto-cjk` e `fonts-noto-core`). Se não houver nenhuma, as Configurações
avisam. O texto em árabe é lido da direita para a esquerda, mas o layout da janela continua da
esquerda para a direita.

Os READMEs em outros idiomas são traduções do README em inglês, que é a referência.

## O catálogo comanda tudo

A lista de apps não fica embutida no inicializador. Ela vem de
[`catalog/catalog.json`](catalog/catalog.json) neste repositório, que é publicado em
`https://haruny.github.io/craftcraft/catalog.json` sempre que muda na `main`. Tanto o
inicializador quanto o site o leem. O inicializador o busca:

- ao iniciar, enviando o último ETag, para que um catálogo sem mudanças custe uma única solicitação minúscula;
- a cada poucas horas enquanto está em execução (8 por padrão; em **Configurações**, de 1 a 168), tentando de novo 15 minutos depois de uma falha;
- quando você clica em **Verificar atualizações**.

Ele guarda a última cópia válida, então abre na hora e funciona offline. Quando o catálogo muda:

| Mudança em `catalog.json` | O que o inicializador mostra |
|---|---|
| Um app novo em `apps` | Um banner "novo no catálogo" e um selo NOVO, até o usuário clicar em **Entendi** |
| `launcher.version` mais nova que esta build | Um banner de atualização com um link de download |
| Uma entrada nova em `notices` | Um banner até ser dispensado |
| Um projeto publica uma versão | **Atualizar** no cartão desse app |
| Um app está listado em `featured.json` | Ele aparece primeiro na ordem padrão Destaques, com um selo DESTAQUE |

**As versões vêm de um único arquivo, não da API do GitHub.** A cada hora, o workflow do catálogo
reúne a versão mais recente de cada app (arquivos e SHA-256), as estrelas e a licença em
`https://haruny.github.io/craftcraft/releases.json`. O inicializador busca esse arquivo com um ETag
junto com o catálogo, então verificar atualizações de todos os apps custa uma solicitação, por maior
que o catálogo fique, e ninguém esbarra no limite do GitHub de 60 solicitações de API por hora. Só um
app que o índice ainda não cobre (um catálogo local em teste, por exemplo) é consultado diretamente
no GitHub. As estrelas e as datas das versões também alimentam o menu **Ordenar**: Destaques, Mais
populares, Nome e Atualizados recentemente.

**Para adicionar um app, envie um pull request que o adicione a `catalog/catalog.json`**, ou indique
a um agente de IA o arquivo [ADD_APP.md](ADD_APP.md) com o repositório do app no GitHub.
O [CONTRIBUTING.md](CONTRIBUTING.md) traz as regras: o app precisa substituir um software pago e ter
código aberto no github.com.

O endereço do catálogo é fixo: as builds de release sempre leem o publicado, e não há configuração
para alterá-lo.

## Apps que você já tem

O inicializador procura cada app do catálogo que já esteja no computador ao iniciar, depois de cada
verificação do catálogo e depois de cada instalação ou desinstalação. Um app encontrado assim aparece
como instalado com a versão dele, ganha **Abrir** e ganha **Atualizar** quando o projeto publica uma
versão mais nova.

| Onde | Como é encontrado | Versão a partir de |
|---|---|---|
| Apps de desktop do Windows | A lista de programas instalados (as entradas Uninstall do registro), comparada estritamente pelo nome: `PhotoCraft`, `PhotoCraft 0.16.0` e `DesignCraft (Craft Apps)` correspondem, `ArtCraft Launcher` não corresponde a ArtCraft | `DisplayVersion` |
| Apps de desktop do macOS | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | O `Info.plist` do pacote |
| Apps de desktop do Linux | O caminho `launch` do catálogo, o comando no PATH ou um AppImage em `~/Applications` | dpkg ou rpm, ou o nome do arquivo do AppImage |
| Apps de linha de comando e de terminal (em todos os sistemas) | O comando no PATH | Executando `<command> --version` (oculto, no máximo 3 segundos) |

Atualizar um app instalado pelo próprio instalador executa o instalador da nova versão, que o
atualiza no lugar. Pacotes MSI são executados com `/passive`, e com `ALLUSERS=1` quando a cópia
instalada é para todos, então o Windows pede a aprovação de um administrador; os registros do próprio
Windows Installer dizem quais cópias são essas, já que ele lista também em HKLM as cópias instaladas
só para um usuário. Um app de linha de comando instalado de outra forma (winget, cargo, um script)
não pode ser substituído, então **Instalar a mais recente** coloca uma cópia atualizada na pasta do
inicializador. **Desinstalar…** executa o desinstalador do próprio app.

Quando uma instalação, atualização, desinstalação ou **Abrir** falha, o cartão diz qual etapa falhou
e por quê, por exemplo *Falha no instalador: This app is already installed for everyone…*, com o
texto completo ao passar o mouse. Para um pacote MSI, o motivo vem do log do Windows Installer, que
**⋯ → Log do instalador** abre; **⋯ → Copiar erro** copia tudo.

`craftcraft --detect <catalog.json>` mostra o que o inicializador encontra.

## Como os apps são instalados

Os arquivos vêm só da release do próprio app no GitHub (`https://github.com/<repo>/releases/download/…`)
e são verificados com o SHA-256 que o GitHub publica para cada arquivo de release. Um arquivo sem
SHA-256 é instalado, mas marcado como **não verificado**.

| Arquivo da release | O que acontece |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | Descompactado na pasta `apps/<id>` do inicializador. As atualizações são descompactadas ao lado e só entram no lugar quando estão completas, então uma atualização com falha deixa a versão antiga funcionando |
| `.AppImage`, `.exe` avulso | Mantido como um único executável em `apps/<id>` |
| Instalador `.msi`, `.exe`, `.dmg`, `.pkg`, `.deb`, `.rpm` | Entregue ao instalador do sistema. O caminho `launch` do catálogo é usado para encontrar e abrir o app |

Os apps de terminal e de linha de comando também ganham um inicializador na pasta `bin` (um arquivo
`.cmd` no Windows, um link simbólico nos outros sistemas). **Configurações → Adicionar ao meu PATH**
coloca essa pasta no seu PATH no Windows; no macOS e no Linux, as Configurações mostram a linha a
adicionar ao perfil do seu shell. **Abrir no terminal** abre um terminal com a pasta já no PATH.

Tudo fica em uma única pasta de dados: `%LOCALAPPDATA%\CraftCraft\CraftCraft\data` no Windows,
`~/Library/Application Support/io.CraftCraft.CraftCraft` no macOS, `~/.local/share/craftcraft` no
Linux, ou `$CRAFTCRAFT_HOME` quando definido. Nada é baixado até você clicar em **Instalar** ou
**Atualizar**. No Windows, os instaladores são apagados depois de executados, e o log de um MSI
depois que ele dá certo; no macOS e no Linux, onde o instalador do sistema ainda está aberto quando
o inicializador passa a vez, eles são apagados depois de uma semana. O modo de depuração mantém todos.

## Compilar e executar

Você precisa do Rust 1.90 ou mais recente. No Linux, instale primeiro os cabeçalhos de janelas (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| Variável de ambiente | Uso |
|---|---|
| `CRAFTCRAFT_FEED` | **Só em builds de depuração:** ler o catálogo de outro endereço ou de um arquivo local, por exemplo `catalog/catalog.json` enquanto você o edita. As builds de release a ignoram |
| `CRAFTCRAFT_HOME` | Usar outra pasta de dados (útil para testar do zero) |
| `GITHUB_TOKEN` | Aumenta o limite do GitHub de 60 para 5.000 solicitações por hora |

`craftcraft --check-catalog <catalog.json>` analisa um catálogo exatamente como o inicializador fará
e lista tudo o que ele ignoraria.

## Versões

Enviar uma tag como `v0.2.0` (igual a `version` em `Cargo.toml`) executa
[release.yml](.github/workflows/release.yml). Ele compila para Windows (x86_64, Arm), macOS (um
`.app` universal, também como `.dmg`) e Linux (x86_64, Arm) nos próprios runners, instala cada build
com o script de instalação para testá-la, empacota as builds assinadas do Windows como instaladores
`.msi` e também os testa, e publica uma release no GitHub com os pacotes e um arquivo `SHA256SUMS`.
Executar o workflow manualmente faz tudo, exceto publicar: as builds ficam anexadas à execução, para
testes. A última tarefa dele então define `launcher.version` no catálogo como a nova versão
e publica o catálogo, e é assim que os inicializadores em execução ficam sabendo da release.

Para compilar e testar um pacote localmente, do jeito que os usuários o recebem:

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

### Assinatura

O workflow de release assina as builds quando os itens abaixo estão definidos em **Settings → Secrets
and variables → Actions** do repositório. Até lá, ele publica as builds sem assinatura.

**macOS (Developer ID e, depois, notarização).** Todos os cinco são secrets:

| Secret | O que é |
|---|---|
| `APPLE_CERTIFICATE` | Um certificado **Developer ID Application** com a chave privada, exportado do Keychain Access como `.p12` e depois codificado em base64 (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | A senha que você deu ao `.p12` |
| `APPLE_API_KEY` | Uma chave de API do App Store Connect (Users and Access → Integrations → Team Keys, função Developer): o arquivo `.p8`, codificado em base64 |
| `APPLE_API_KEY_ID` | O ID dessa chave |
| `APPLE_API_ISSUER` | O Issuer ID mostrado acima das chaves |

Só com os dois primeiros, o app é assinado mas não notarizado, e o Gatekeeper ainda mostra um aviso
quando ele é baixado em um navegador. Para assinar localmente, defina `APPLE_SIGNING_IDENTITY` (e as
variáveis `APPLE_API_KEY_*`, com `APPLE_API_KEY_PATH` apontando para o `.p8`) antes de executar
`scripts/dist.sh`.

**Windows (Microsoft Artifact Signing, antigo Trusted Signing).** Sem secrets: o workflow entra no
Azure com o OpenID Connect do GitHub, no qual o Azure só confia a partir do ambiente `release`
deste repositório.

1. No Azure portal, crie uma **Artifact Signing account** (o nível Basic basta) e anote
   o endpoint dela, por exemplo `https://eus.codesigning.azure.net/`.
2. Na conta, conclua a **identity validation** (Public) e depois crie a partir dela um **certificate profile**
   do tipo Public Trust.
3. No Microsoft Entra ID, registre um app. Em Certificates & secrets → Federated credentials,
   adicione uma para o GitHub Actions: organização `haruny`, repositório `craftcraft`, tipo de entidade
   **Environment**, nome `release`.
4. Na conta de assinatura, dê a esse app a função **Artifact Signing Certificate Profile Signer**
   (Access control → Add role assignment).
5. No GitHub, crie o ambiente `release` (Settings → Environments). Limite-o à branch `main`
   e às tags `v*`, para que só código revisado seja assinado.
6. Adicione estas variáveis do repositório:

| Variável | Valor |
|---|---|
| `AZURE_CLIENT_ID` | O Application (client) ID do registro do app |
| `AZURE_TENANT_ID` | O Directory (tenant) ID dele |
| `AZURE_SUBSCRIPTION_ID` | A assinatura em que a conta de assinatura está |
| `ARTIFACT_SIGNING_ENDPOINT` | O endpoint da etapa 1 |
| `ARTIFACT_SIGNING_ACCOUNT` | O nome da conta de assinatura |
| `ARTIFACT_SIGNING_PROFILE` | O nome do certificate profile |

A partir daí, cada execução do workflow de release assina o `craftcraft.exe` nos dois zips do Windows,
x86_64 e Arm, e confere cada assinatura e carimbo de data/hora antes de publicar os zips. Uma execução
iniciada manualmente também assina, então você pode testar antes de criar a tag.

## Privacidade

Todos os detalhes estão em [Privacy_Statement.md](Privacy_Statement.md). Em resumo:

- **Contagens.** As builds de release buscam o catálogo pelo endpoint do próprio CraftCraft, que conta
  as verificações do catálogo por dia, por versão do inicializador, sistema operacional e versão,
  processador e país. Ele não guarda endereço IP nem id. É assim que sabemos em quais computadores o
  CraftCraft é executado.
- **Estatísticas de uso**, que você pode desativar. Inicializações e quais apps você instala, atualiza,
  abre e remove, enviadas sob um id aleatório que não está ligado a você nem ao seu computador. Nada é
  enviado até você ver o aviso sobre isso. Desative ali ou em Configurações > Privacidade.
- `DO_NOT_TRACK=1` ou `CRAFTCRAFT_NO_TELEMETRY=1` desativa as duas coisas: o catálogo passa a vir
  direto do GitHub Pages.

Uma build compilada a partir do código-fonte não tem nenhuma das duas (veja
[telemetry/README.md](telemetry/README.md)). Fora isso, o CraftCraft só se conecta para buscar
informações de versões em `haruny.github.io` e `api.github.com`, os ícones dos apps listados no
catálogo e os arquivos de release que você escolhe instalar de `github.com`. Esses serviços veem seu
endereço IP, como em qualquer solicitação na web.

## Estrutura

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

## Patrocínio

O CraftCraft é gratuito e feito no meu tempo livre. Se ele economizar uma ou duas licenças para você,
você pode apoiá-lo pelo [GitHub Sponsors](https://github.com/sponsors/haruny).

## Licença

MIT OR Apache-2.0. As fontes Geist embutidas (`assets/fonts`) estão sob a SIL Open Font License
(`assets/fonts/OFL.txt`). Cada app que o inicializador instala tem a própria licença.
