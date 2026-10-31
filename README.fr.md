# CraftCraft

[English](README.md) · **Français** · [Español](README.es.md) · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · [বাংলা](README.bn.md) · [Português](README.pt.md)

*Ceci est une traduction : le [README en anglais](README.md) fait référence.*

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**Des logiciels bien connus, réécrits en Rust : un seul lanceur pour les installer, les mettre à jour et les ouvrir.**

PhotoCraft à la place de Photoshop, GridCraft à la place d'Excel, BIMCraft à la place de Revit, RustDesk
à la place de TeamViewer. CraftCraft est une petite application native (Rust et egui, sans Electron) qui
répertorie des remplaçants open source écrits en Rust pour les logiciels payants que vous utilisez déjà,
installe la bonne version pour votre ordinateur, vérifie chaque téléchargement par rapport à son SHA-256
publié, et vous prévient quand une mise à jour ou une nouvelle application arrive dans le catalogue.

## Installation

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

Le script télécharge la dernière version pour votre système depuis
[Releases](https://github.com/haruny/craftcraft/releases), vérifie son SHA-256, l'installe pour
votre utilisateur uniquement (sans droits d'administrateur), puis l'ouvre. Relancez-le pour mettre à jour.

| | Installé dans | En plus |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Raccourci dans le menu Démarrer ; listé dans Paramètres → Applications, qui le désinstalle ; une icône dans la zone de notification pendant son exécution |
| macOS | `~/Applications/CraftCraft.app` | Une seule version pour Apple silicon et Intel |
| Linux | `~/.local/bin/craftcraft` | Entrée et icône dans le menu Applications |

Ou téléchargez un programme d'installation depuis [Releases](https://github.com/haruny/craftcraft/releases/latest) :

- **Windows** : `craftcraft-windows-x86_64.msi` (ou `-aarch64.msi` pour Arm). Il demande pour qui
  installer CraftCraft : **Only for me** l'installe de la même façon que le script (sans droits
  d'administrateur), **Everyone** l'installe dans `%ProgramFiles%\CraftCraft` et dans le menu Démarrer
  de chaque utilisateur, après que Windows a demandé l'approbation d'un administrateur. Il propose
  ensuite d'ouvrir CraftCraft. Lancez une version plus récente pour mettre à jour ; elle doit faire le
  même choix que la copie qu'elle remplace (désinstallez cette copie pour changer). Sans sa fenêtre
  (`msiexec /i … /qn`), il installe pour l'utilisateur actuel, ou pour tous avec `ALLUSERS=1`.
  Pour une copie installée pour vous seul, le script et le programme d'installation peuvent prendre
  la place l'un de l'autre : le dernier lancé possède l'installation, et Paramètres → Applications ne
  la liste qu'une fois.
- **macOS** : `craftcraft-macos-universal.dmg`. Ouvrez-le et faites glisser CraftCraft dans Applications.

Sous Windows et macOS, les **Paramètres** proposent aussi :

- **Lancer à l'ouverture de session** : CraftCraft démarre dans la zone de notification (la barre des
  menus sous macOS) quand vous vous connectez, sous la forme `craftcraft --background`. Windows le lance
  depuis la valeur `CraftCraft` sous `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` ; macOS depuis
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist`.
- **Réduire dans la zone de notification à la fermeture** (activé par défaut) : fermer la fenêtre laisse
  CraftCraft dans la zone de notification, où il continue de rechercher les mises à jour ; l'info-bulle
  de l'icône les compte. Sous Windows, cliquez sur l'icône pour ouvrir la fenêtre, ou faites un clic
  droit pour lancer une recherche ou quitter. Sous macOS, cliquez dessus pour obtenir le même menu ;
  tant que la fenêtre est fermée, il n'y a pas d'icône dans le Dock, et ouvrir CraftCraft depuis le
  Finder fait réapparaître la fenêtre. ⌘Q quitte toujours l'application.

Une seule copie s'exécute : rouvrir CraftCraft fait réapparaître la fenêtre de celle qui est dans la
zone de notification. Sous Windows, une copie qui n'a été installée ni par le script ni par le programme
d'installation (extraite du zip, par exemple) s'ajoute elle-même au menu Démarrer.

CraftCraft se met à jour tout seul. Quand le catalogue annonce une version plus récente, il la
télécharge depuis [Releases](https://github.com/haruny/craftcraft/releases), vérifie le SHA-256 que
GitHub publie pour elle, et propose **Redémarrer pour mettre à jour**. S'il attend dans la zone de
notification, il redémarre de lui-même et y retourne. Une copie installée avec le `.msi` se met à
jour avec le nouveau `.msi` (pour tous les utilisateurs, seulement quand vous cliquez, car Windows
demande l'approbation d'un administrateur). Les autres copies remplacent l'ancien fichier par le
nouveau, au même endroit. Si la vérification automatique est désactivée, il attend que vous cliquiez
sur **Mettre à jour**. Une copie dans un dossier où CraftCraft ne peut pas écrire se contente
d'indiquer qu'une nouvelle version existe, avec un lien.

**Paramètres → Mode débogage** (partout) consigne les actions de CraftCraft dans `logs/craftcraft.log`
dans le dossier de données, et conserve les programmes d'installation téléchargés et leurs journaux au
lieu de les supprimer.

## Langues

CraftCraft est en anglais, sauf si vous choisissez une autre langue dans **Paramètres → Langue** :
Français, Español, 中文（简体）, हिन्दी, العربية, বাংলা ou Português. Le changement s'applique
immédiatement, menu de la zone de notification compris. Le texte qui vient du catalogue (slogans et
descriptions des applications, catégories, annonces) reste en anglais, tout comme les détails des
erreurs renvoyées par le système.

Les traductions se trouvent dans [`locales/`](locales), un fichier JSON par langue, intégrés au
lanceur ; tout ce qui manque dans une traduction s'affiche en anglais. Elles sont récentes, donc les
corrections de locuteurs natifs sont les bienvenues : modifiez le fichier et lancez `cargo test`, qui
vérifie que chaque fichier a les mêmes clés et les mêmes variables que `en.json`.

Le chinois, l'hindi, l'arabe et le bengali utilisent des caractères absents de la police Geist
fournie, donc le lanceur utilise une police déjà présente sur l'ordinateur : Microsoft YaHei, Nirmala
UI et Segoe UI sous Windows ; PingFang ou Hiragino, Kohinoor et SF Arabic ou Geeza Pro sous macOS ;
Noto sous Linux (Debian/Ubuntu : `fonts-noto-cjk` et `fonts-noto-core`). S'il n'y en a aucune, les
Paramètres l'indiquent. Le texte arabe se lit de droite à gauche, mais la disposition de la fenêtre
reste de gauche à droite.

Les README dans d'autres langues sont des traductions du README anglais, qui fait référence.

## Le catalogue pilote tout

La liste des applications n'est pas intégrée au lanceur. Elle vient de
[`catalog/catalog.json`](catalog/catalog.json) dans ce dépôt, publié à l'adresse
`https://haruny.github.io/craftcraft/catalog.json` chaque fois qu'il change sur `main`. Le lanceur
et le site web le lisent tous les deux. Le lanceur le récupère :

- au démarrage, en envoyant le dernier ETag, si bien qu'un catalogue inchangé ne coûte qu'une toute petite requête ;
- toutes les quelques heures pendant qu'il tourne (8 par défaut ; les **Paramètres** permettent de 1 à 168), en réessayant 15 minutes après un échec ;
- quand vous cliquez sur **Rechercher des mises à jour**.

Il garde la dernière copie valide, ce qui lui permet de s'ouvrir instantanément et de fonctionner hors
ligne. Quand le catalogue change :

| Changement dans `catalog.json` | Ce qu'affiche le lanceur |
|---|---|
| Une nouvelle application dans `apps` | Une bannière « Nouveau dans le catalogue » et un badge NOUVEAU, jusqu'à ce que l'utilisateur clique sur **J'ai compris** |
| `launcher.version` plus récent que cette version | Une bannière de mise à jour avec un lien de téléchargement |
| Une nouvelle entrée dans `notices` | Une bannière, jusqu'à ce qu'elle soit ignorée |
| Un projet publie une version | **Mettre à jour** sur la carte de cette application |
| Une application figure dans `featured.json` | Elle arrive en tête de l'ordre par défaut, À la une, avec un badge À LA UNE |

**Les versions viennent d'un seul fichier, pas de l'API de GitHub.** Toutes les heures, le workflow du
catalogue rassemble la dernière version de chaque application (fichiers et SHA-256), ses étoiles et sa
licence dans `https://haruny.github.io/craftcraft/releases.json`. Le lanceur récupère ce fichier avec
un ETag en même temps que le catalogue : vérifier les mises à jour de toutes les applications ne coûte
donc qu'une requête, quelle que soit la taille du catalogue, et personne n'atteint la limite de GitHub
de 60 requêtes d'API par heure. Seule une application que l'index ne couvre pas encore (dans un
catalogue local en test, par exemple) est recherchée directement sur GitHub. Les étoiles et les dates
de publication alimentent aussi le menu **Trier** : À la une, Les plus populaires, Nom et Récemment
mises à jour.

**Pour ajouter une application, envoyez une pull request qui l'ajoute à `catalog/catalog.json`**, ou
confiez à un agent d'IA le fichier [ADD_APP.md](ADD_APP.md) avec le dépôt GitHub de l'application.
[CONTRIBUTING.md](CONTRIBUTING.md) donne les règles : l'application doit remplacer un logiciel payant,
et être open source sur github.com.

L'adresse du catalogue est fixe : les versions publiées lisent toujours le catalogue publié, et aucun
paramètre ne permet de la changer.

## Les applications que vous avez déjà

Le lanceur recherche chaque application du catalogue déjà présente sur l'ordinateur, au démarrage,
après chaque vérification du catalogue et après chaque installation ou désinstallation. Une application
trouvée ainsi s'affiche comme installée avec sa version, reçoit **Ouvrir**, et reçoit **Mettre à jour**
quand son projet publie une version plus récente.

| Où | Comment elle est trouvée | Version issue de |
|---|---|---|
| Applications de bureau Windows | La liste des programmes installés (les entrées Uninstall du registre), comparée strictement par nom : `PhotoCraft`, `PhotoCraft 0.16.0` et `DesignCraft (Craft Apps)` correspondent, `ArtCraft Launcher` ne correspond pas à ArtCraft | `DisplayVersion` |
| Applications de bureau macOS | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | Le `Info.plist` du bundle |
| Applications de bureau Linux | Le chemin `launch` du catalogue, la commande dans le PATH, ou une AppImage dans `~/Applications` | dpkg ou rpm, ou le nom de fichier de l'AppImage |
| Applications en ligne de commande et de terminal (partout) | La commande dans le PATH | L'exécution de `<command> --version` (masquée, 3 secondes au maximum) |

Mettre à jour une application installée par son propre programme d'installation lance le programme
d'installation de la nouvelle version, qui la met à niveau sur place. Les paquets MSI s'exécutent avec
`/passive`, et avec `ALLUSERS=1` quand la copie installée l'est pour tous les utilisateurs, si bien que
Windows demande l'approbation d'un administrateur ; ce sont les propres enregistrements de Windows
Installer qui indiquent de quelles copies il s'agit, car il liste aussi sous HKLM les copies installées
pour un seul utilisateur. Une application en ligne de commande installée autrement (winget, cargo, un
script) ne peut pas être remplacée : **Installer la dernière version** place donc une copie à jour dans
le dossier du lanceur. **Désinstaller…** lance le programme de désinstallation de l'application.

Quand une installation, une mise à jour, une désinstallation ou **Ouvrir** échoue, la carte indique
quelle étape a échoué et pourquoi, par exemple *Échec du programme d'installation : This app is already
installed for everyone…*, avec le message complet au survol. Pour un paquet MSI, la raison vient du
journal de Windows Installer, qu'ouvre **⋯ → Journal d'installation** ; **⋯ → Copier l'erreur** copie
le tout.

`craftcraft --detect <catalog.json>` affiche ce que trouve le lanceur.

## Comment les applications sont installées

Les fichiers proviennent uniquement de la version GitHub de chaque application
(`https://github.com/<repo>/releases/download/…`) et sont vérifiés par rapport au SHA-256 que GitHub
publie pour chaque fichier de version. Un fichier qui n'en a pas est installé mais marqué
**non vérifié**.

| Fichier de version | Ce qui se passe |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | Extrait dans le dossier `apps/<id>` du lanceur. Les mises à jour sont extraites à côté et ne prennent la place de l'ancienne version qu'une fois complètes : une mise à jour qui échoue laisse l'ancienne version fonctionnelle |
| `.AppImage`, `.exe` seul | Conservé tel quel comme exécutable dans `apps/<id>` |
| Programme d'installation `.msi` ou `.exe`, `.dmg`, `.pkg`, `.deb`, `.rpm` | Confié au programme d'installation du système. Le chemin `launch` du catalogue sert à la retrouver et à l'ouvrir |

Les applications de terminal et en ligne de commande reçoivent aussi un lanceur dans le dossier `bin`
(un fichier `.cmd` sous Windows, un lien symbolique ailleurs). **Paramètres → L'ajouter à mon PATH**
ajoute ce dossier à votre PATH sous Windows ; sous macOS et Linux, les Paramètres affichent la ligne à
ajouter au profil de votre shell. **Ouvrir dans le terminal** ouvre un terminal dont le PATH contient
déjà ce dossier.

Tout se trouve dans un seul dossier de données : `%LOCALAPPDATA%\CraftCraft\CraftCraft\data` sous
Windows, `~/Library/Application Support/io.CraftCraft.CraftCraft` sous macOS,
`~/.local/share/craftcraft` sous Linux, ou `$CRAFTCRAFT_HOME` s'il est défini. Rien n'est téléchargé
avant que vous cliquiez sur **Installer** ou **Mettre à jour**. Sous Windows, les programmes
d'installation sont supprimés une fois exécutés, et le journal d'un MSI une fois l'installation
réussie ; sous macOS et Linux, où le programme d'installation du système est encore ouvert quand le
lanceur lui passe la main, ils sont supprimés au bout d'une semaine. Le mode débogage les conserve tous.

## Compiler et lancer

Il faut Rust 1.90 ou plus récent. Sous Linux, installez d'abord les en-têtes du système de fenêtrage
(Debian/Ubuntu :
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| Variable d'environnement | Usage |
|---|---|
| `CRAFTCRAFT_FEED` | **Versions de débogage uniquement :** lire le catalogue depuis une autre adresse ou un fichier local, par ex. `catalog/catalog.json` pendant qu'on le modifie. Les versions publiées l'ignorent |
| `CRAFTCRAFT_HOME` | Utiliser un autre dossier de données (pratique pour tester en repartant de zéro) |
| `GITHUB_TOKEN` | Fait passer la limite de GitHub de 60 à 5 000 requêtes par heure |

`craftcraft --check-catalog <catalog.json>` analyse un catalogue exactement comme le fera le lanceur
et liste tout ce qu'il ignorerait.

## Publications

Pousser un tag comme `v0.2.0` (correspondant à `version` dans `Cargo.toml`) lance
[release.yml](.github/workflows/release.yml). Il compile Windows (x86_64, Arm), macOS (une seule
`.app` universelle, aussi sous forme de `.dmg`) et Linux (x86_64, Arm) sur leurs propres runners,
installe chaque version avec le script d'installation pour la tester, empaquette les versions Windows
signées en programmes d'installation `.msi` et les teste aussi, puis publie une version GitHub avec
les paquets et un fichier `SHA256SUMS`. Lancer le workflow à la main fait tout sauf la publication :
les versions compilées sont jointes à l'exécution, pour les tests. Sa dernière tâche règle ensuite `launcher.version` du catalogue sur la
nouvelle version et publie le catalogue : c'est ainsi que les lanceurs en cours d'exécution
apprennent la sortie.

Pour compiler et essayer un paquet en local, tel que les utilisateurs le reçoivent :

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

### Signature

Le workflow de publication signe les versions une fois les éléments suivants définis dans
**Settings → Secrets and variables → Actions** du dépôt. D'ici là, il publie les versions sans
signature.

**macOS (Developer ID, puis notarisation).** Les cinq sont des secrets :

| Secret | Ce que c'est |
|---|---|
| `APPLE_CERTIFICATE` | Un certificat **Developer ID Application** avec sa clé privée, exporté depuis Trousseaux d'accès au format `.p12`, puis encodé en base64 (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | Le mot de passe que vous avez donné au `.p12` |
| `APPLE_API_KEY` | Une clé d'API App Store Connect (Users and Access → Integrations → Team Keys, rôle Developer) : le fichier `.p8`, encodé en base64 |
| `APPLE_API_KEY_ID` | L'ID de cette clé |
| `APPLE_API_ISSUER` | L'Issuer ID affiché au-dessus des clés |

Avec seulement les deux premiers, l'application est signée mais pas notarisée, et Gatekeeper avertit
encore quand elle est téléchargée dans un navigateur. Pour signer en local, définissez
`APPLE_SIGNING_IDENTITY` (et les variables `APPLE_API_KEY_*`, avec `APPLE_API_KEY_PATH` pointant vers
le `.p8`) avant de lancer `scripts/dist.sh`.

**Windows (Microsoft Artifact Signing, anciennement Trusted Signing).** Aucun secret : le workflow se
connecte à Azure avec l'OpenID Connect de GitHub, auquel Azure ne fait confiance que depuis
l'environnement `release` de ce dépôt.

1. Dans le portail Azure, créez un **Artifact Signing account** (le niveau Basic suffit) et notez
   son point de terminaison, par ex. `https://eus.codesigning.azure.net/`.
2. Dans ce compte, effectuez l'**identity validation** (Public), puis créez à partir de celle-ci un
   **certificate profile** de type Public Trust.
3. Dans Microsoft Entra ID, inscrivez une application. Sous Certificates & secrets → Federated
   credentials, ajoutez-en un pour GitHub Actions : organisation `haruny`, dépôt `craftcraft`, type
   d'entité **Environment**, nom `release`.
4. Sur le compte de signature, attribuez à cette application le rôle
   **Artifact Signing Certificate Profile Signer** (Access control → Add role assignment).
5. Dans GitHub, créez l'environnement `release` (Settings → Environments). Limitez-le à la branche
   `main` et aux tags `v*`, afin que seul du code relu soit signé.
6. Ajoutez ces variables de dépôt :

| Variable | Valeur |
|---|---|
| `AZURE_CLIENT_ID` | L'Application (client) ID de l'inscription d'application |
| `AZURE_TENANT_ID` | Son Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | L'abonnement dans lequel se trouve le compte de signature |
| `ARTIFACT_SIGNING_ENDPOINT` | Le point de terminaison de l'étape 1 |
| `ARTIFACT_SIGNING_ACCOUNT` | Le nom du compte de signature |
| `ARTIFACT_SIGNING_PROFILE` | Le nom du certificate profile |

À partir de là, chaque exécution du workflow de publication signe `craftcraft.exe` dans les deux zips
Windows, x86_64 et Arm, et vérifie chaque signature et chaque horodatage avant la publication des zips.
Une exécution lancée à la main signe aussi, ce qui permet d'essayer avant de poser un tag.

## Confidentialité

Tous les détails se trouvent dans [Privacy_Statement.md](Privacy_Statement.md). En bref :

- **Comptages.** Les versions publiées récupèrent le catalogue via le propre point de terminaison de
  CraftCraft, qui compte les vérifications du catalogue par jour, par version du lanceur, système
  d'exploitation et version, processeur et pays. Il ne conserve ni adresse IP ni identifiant. C'est
  ainsi que nous savons sur quels ordinateurs CraftCraft fonctionne.
- **Statistiques d'utilisation**, que vous pouvez désactiver. Les lancements, et les applications que
  vous installez, mettez à jour, ouvrez et supprimez, envoyés sous un identifiant aléatoire qui n'est
  lié ni à vous ni à votre ordinateur. Rien n'est envoyé tant que vous n'avez pas vu l'avis à ce sujet.
  Désactivez-les à cet endroit ou dans Paramètres > Confidentialité.
- `DO_NOT_TRACK=1` ou `CRAFTCRAFT_NO_TELEMETRY=1` désactive les deux : le catalogue vient alors
  directement de GitHub Pages.

Une version compilée depuis les sources n'a ni l'un ni l'autre (voir
[telemetry/README.md](telemetry/README.md)). Pour le reste, CraftCraft ne se connecte que pour
récupérer les informations de version depuis `haruny.github.io` et `api.github.com`, les icônes
d'applications que liste le catalogue, et les fichiers de version que vous choisissez d'installer
depuis `github.com`. Ces services voient votre adresse IP, comme pour toute requête web.

## Organisation

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

## Soutenir le projet

CraftCraft est gratuit et développé sur mon temps libre. S'il vous fait économiser une licence ou deux,
vous pouvez le soutenir via [GitHub Sponsors](https://github.com/sponsors/haruny).

## Licence

MIT OR Apache-2.0. Les polices Geist fournies (`assets/fonts`) sont sous SIL Open Font License
(`assets/fonts/OFL.txt`). Chaque application installée par le lanceur est sous sa propre licence.
