<h1 align="center">Koharu</h1>

<p align="center">Traducteur de manga propulsé par le ML, écrit en <b>Rust</b>.</p>

<p align="center">
<a href="https://github.com/Endymi0n74/Koharu/releases/latest" target="_blank"><img alt="Dernière release" src="https://img.shields.io/github/v/release/Endymi0n74/Koharu?style=for-the-badge"></a>
<a href="https://github.com/Endymi0n74/Koharu/releases" target="_blank"><img alt="Téléchargements GitHub (toutes releases)" src="https://img.shields.io/github/downloads/Endymi0n74/Koharu/total?style=for-the-badge"></a>
<a href="https://github.com/Endymi0n74/Koharu/actions/workflows/koharu-batch.yml" target="_blank"><img alt="CI koharu-batch" src="https://github.com/Endymi0n74/Koharu/actions/workflows/koharu-batch.yml/badge.svg"></a>
</p>

<p align="center">
<a href="https://github.com/Endymi0n74/Koharu/releases" target="_blank">Releases</a> · <a href="https://koharu.rs/" target="_blank">Docs</a> · <a href="packages/docs/en/fork.mdx" target="_blank">Guide de la CLI par lot</a> · <a href="https://github.com/Endymi0n74/Koharu/issues" target="_blank">Rapports de bug</a> · <a href="https://discord.gg/mHvHkxGnUY" target="_blank">Discord</a>
</p>

<p align="center">
<a href="https://koharu.rs/ja" target="_blank">日本語</a> | <a href="https://koharu.rs/zh" target="_blank">简体中文</a>
</p>

**🇫🇷 Français** · [🇬🇧 English](README.en.md)

Koharu introduit un workflow local-first pour la traduction de manga, en exploitant la puissance du ML pour automatiser le processus. Il combine les capacités de détection d'objets, d'OCR, d'inpainting et des LLM afin de créer une expérience de traduction fluide.

> [!NOTE]
> Koharu exécute ses modèles de vision et ses LLM **localement** sur votre machine pour que vos données restent privées et sécurisées.

---

![capture d'écran de l'interface Koharu](packages/docs/screenshot.png)

## Fonctionnalités

- [Gestion de projets multi-formats](https://koharu.rs/en/guides/projects) pour images matricielles, archives et PDF, avec séquencement des pages
- [Pipeline sélectif](https://koharu.rs/en/guides/processing) pour la détection, l'OCR, la traduction et l'inpainting au niveau de la page ou du projet
- [Détection et segmentation](https://koharu.rs/en/guides/processing) des zones de texte, des bulles de dialogue et des zones de nettoyage
- [OCR multimodal](https://koharu.rs/en/models/vision) pour les dialogues, les légendes et le texte général de la page
- [Inférence GGUF locale et fournisseurs hébergés](https://koharu.rs/en/models/providers) pour les workflows LLM et de traduction automatique
- [Inpainting génératif](https://koharu.rs/en/guides/cleanup) pour la suppression du texte source et la reconstruction de l'illustration
- [Relecture](https://koharu.rs/en/guides/review) pour corriger les sorties de l'OCR et de la traduction
- [Canvas basé sur WebGPU](https://koharu.rs/en/guides/canvas) pour le nettoyage manuel, le placement du texte et la composition des pages
- [Mise en forme et disposition multilingues du texte](https://koharu.rs/en/guides/typesetting) avec ajustement automatique, repli de polices, CJK vertical et texte de droite à gauche
- [Export PSD en calques](https://koharu.rs/en/guides/export) pour une livraison aplatie et une édition en calques
- [Workflow basé sur des agents](https://koharu.rs/en/agent/projects) pour l'inspection et l'édition des projets, ainsi que le contrôle du pipeline

<<<<<<< HEAD
## Ce que cette branche ajoute
=======
- [Multi-format project management](https://koharu.rs/guides/projects) for raster images, archives, and PDFs with page sequencing
- [Selective pipeline](https://koharu.rs/guides/processing) for detection, OCR, translation, and inpainting at page or project scope
- [Detection and segmentation](https://koharu.rs/guides/processing) for panels, speech bubbles, text, and sound effects
- [OCR](https://koharu.rs/models/vision) for Japanese, Chinese, Korean, and English manga text
- [Local GGUF inference and hosted providers](https://koharu.rs/models/providers) for LLM translation, including ChatGPT and Grok sign-in
- [Generative inpainting](https://koharu.rs/guides/cleanup) for source-text removal and artwork reconstruction
- [Proofreading](https://koharu.rs/guides/review) for correcting OCR and translation output
- [GPU-based canvas](https://koharu.rs/guides/canvas) for manual cleanup, text placement, and page composition
- [Multilingual text shaping and layout](https://koharu.rs/guides/typesetting) with automatic fitting, font fallback, vertical CJK, and right-to-left text
- [PNG, CBZ, and layered PSD export](https://koharu.rs/guides/export) for flattened delivery and layered editing
- [Agent-driven workflow](https://koharu.rs/agent/projects) with an [MCP server](https://koharu.rs/agent/setup) for AI agents
>>>>>>> upstream/main

Ce dépôt est un fork de [koharu-rs/koharu](https://github.com/koharu-rs/koharu) : l'application d'origine reste intacte et gagne l'outillage autour d'elle.

<<<<<<< HEAD
- **La CLI par lot `koharu-batch`** — traduit des chapitres entiers depuis le terminal : un dossier de scans ou une archive CBZ en entrée, des pages traduites en sortie (`--lang` pour la cible), ou un **volume entier** où chaque sous-dossier ou `.cbz` devient son propre chapitre. Exécution **phase-major** (les modèles se chargent une seule fois au lieu d'une fois par page), reprise page par page y compris depuis une archive CBZ de sortie existante, `--no-vision` pour les exécutions en texte seul, rapport HTML/Markdown de fin d'exécution avec aperçus avant/après, résumé `--json` lisible par les machines, et code de sortie 0 seulement si toutes les pages ont réussi (prêt pour la CI).
- **Un choix automatique de modèle calibré** — priorité aux modèles qui tiennent entièrement dans le budget de VRAM (Qwen 3.5 4B sur 8 Go), plafonné à la classe 8 Go (un 12B ou plus ne s'obtient que par un `--llm` explicite), puis repli sur le meilleur split viable ; calibration mesurée (`~/.koharu/vram-calibration.toml` enregistre les pics réels) et avertissement avant un premier gros téléchargement. `koharu-batch models` liste le catalogue avec les estimations de VRAM.
- **La fiche de chapitre apprise** — avant de traduire un chapitre, l'outil lit son texte OCR **une fois** et en tire une fiche (œuvre, synopsis, registre, personnages, glossaire) qu'il joint ensuite à la traduction de **chaque page**. Les noms et les termes cessent de dériver d'une page à l'autre : mesuré sur un chapitre réel de 34 pages, la tenue des noms/termes passe de **41,7 %** (contexte glissant) à **97,2 %** (fiche + glossaire) — élargir la fenêtre de contexte n'apporte rien, une fiche si. La fiche est un composant du projet : elle se relit et se corrige à la main, et se réutilise aux runs suivants. Le glossaire est restreint aux termes réellement présents, et le prompt interdit d'en introduire un absent du segment.
- **La reproductibilité** — `--deterministic` et `--torch-fp32` pour des sessions identiques d'un run à l'autre, `--ocr-cache` pour figer les lectures d'OCR, `--verify` pour contrôler les sorties, `--retries` pour relancer les segments qu'une réponse a laissés non traduits.
- **Le mode dossier dans l'application** — l'interface lance `koharu-batch` avec un sélecteur de modèle et de fournisseur hébergé, affiche le modèle en cours dans le centre d'activité, et remonte les erreurs en ligne.
- **Les fournisseurs hébergés dans le lot** — `--provider openai_compatible`, `lm_studio`, et les points d'accès compatibles OpenAI.
- **La maintenance du store** — `koharu-batch prune` repère les modèles orphelins, les datasets et les runtimes (ajoutez `--delete` pour les retirer) ; `KOHARU_STORE` déplace le store hors du disque système.
- **Un catalogue étendu** — les variantes non censurées Gemma 4 et Qwen (HauhauCS) en plus des modèles amont.

```bash
# Prévisualiser ce qui serait exécuté (pages, modèle, VRAM) sans rien lancer
koharu-batch --input ./chapter-12 --output ./chapter-12-fr --dry-run

# Un dossier de scans → un dossier de pages traduites
koharu-batch --input ./chapter-12 --output ./chapter-12-fr

# Une archive CBZ → une archive CBZ, avec le modèle choisi pour votre GPU
koharu-batch --input ./chapter-13.cbz --output ./chapter-13-fr.cbz

# Un volume entier : chaque sous-dossier et .cbz de ./series devient un chapitre
koharu-batch --input ./series --output ./series-fr

# Résumé lisible par les machines (fonctionne aussi avec --dry-run)
koharu-batch --input ./chapter-12 --output ./chapter-12-fr --json run.json

# Exécution texte seul : saute détection, OCR et inpainting
koharu-batch --input ./chapter-14 --output ./chapter-14-fr --no-vision

# Exécution reproductible : traduction greedy + étapes Torch en fp32 (détection, inpainting)
koharu-batch --input ./chapter-12 --output ./chapter-12-fr --deterministic --torch-fp32

# Lister les modèles locaux avec leurs estimations de VRAM (pics mesurés compris)
koharu-batch models
```

Voir [packages/docs/en/fork.mdx](packages/docs/en/fork.mdx) pour la documentation complète de la CLI.

## Accélération matérielle

Koharu prend en charge CUDA et ROCm / HIP sur Windows et Linux, Metal sur les puces Apple, et Vulkan sur Windows et Linux. Gardez votre pilote graphique à jour ; une installation complète du SDK CUDA ou ROCm n'est pas requise. Consultez [Exigences d'exécution et matérielles](https://koharu.rs/en/hardware) pour des recommandations propres à chaque modèle.
=======
Koharu supports CUDA on Windows and Linux, ROCm / HIP on x86-64 Windows and Linux, Metal on Apple silicon, and Vulkan on Windows and Linux out of the box. See [Runtime and hardware requirements](https://koharu.rs/hardware) for model-specific guidance.
>>>>>>> upstream/main

### CUDA

CUDA 13.3 nécessite un GPU NVIDIA de classe Turing ou plus récent, ainsi qu'un pilote R610 ou plus récent. Consultez la [matrice CUDA toolkit, pilotes et architectures](https://docs.nvidia.com/datacenter/tesla/drivers/cuda-toolkit-driver-and-architecture-matrix.html) officielle d'NVIDIA et installez le [dernier pilote NVIDIA](https://www.nvidia.com/en-us/drivers/).

### ROCm / HIP

<<<<<<< HEAD
La prise en charge de ROCm 10.0 dépend de la combinaison exacte du GPU AMD, du système d'exploitation et du pilote. Consultez la [matrice de compatibilité ROCm 10.0.0](https://rocm.docs.amd.com/en/docs-10.0.0/compatibility/compatibility-matrix.html) officielle d'AMD et installez un [pilote AMD](https://www.amd.com/en/support) compatible.
=======
ROCm 10.1 support depends on the exact AMD GPU, operating system, and driver combination. Check AMD's official [ROCm 10.1.0 compatibility matrix](https://rocm.docs.amd.com/en/docs-10.1.0/compatibility/compatibility-matrix.html) and install a compatible [AMD driver](https://www.amd.com/en/support).
>>>>>>> upstream/main

### Metal

Metal est disponible sur les Mac à puces Apple.

### Vulkan

<<<<<<< HEAD
Vulkan est disponible sur Windows et Linux en alternative à CUDA et ROCm / HIP.

### WebGPU

Le canvas de l'éditeur utilise WebGPU et nécessite un pilote graphique à jour, même lorsque l'inférence s'exécute sur le CPU.

### CPU

L'inférence sur CPU est disponible pour les charges de travail prises en charge, mais elle est nettement plus lente.
=======
Vulkan is available on Windows and Linux as an alternative to CUDA and ROCm / HIP. With Vulkan, some models still fall back to the CPU, which is slower.
>>>>>>> upstream/main

## Modèles d'apprentissage automatique

<<<<<<< HEAD
Koharu utilise des modèles distincts pour la détection, l'OCR, l'inpainting et la traduction. [Vision et inpainting](https://koharu.rs/en/models/vision) et [traduction et génération](https://koharu.rs/en/models/translation) ont des paramètres de modèle séparés.
=======
Koharu uses separate models for detection, OCR, inpainting, and translation. [Vision and inpainting](https://koharu.rs/models/vision) and [translation](https://koharu.rs/models/translation) have separate model settings.
>>>>>>> upstream/main

### Modèles de vision par ordinateur

Les modèles de détection, d'OCR et d'inpainting sont sélectionnés séparément.

#### Détection et disposition

<<<<<<< HEAD
Le modèle de détection trouve les zones de texte, les bulles de dialogue et les masques de segmentation.
=======
The detection model finds panels, speech bubbles, text, and sound effects, with a mask over each text's glyphs.
>>>>>>> upstream/main

- [Koharu Layout HighRes](https://huggingface.co/mayocream/koharu-layout-highres)

#### OCR

L'OCR lit le texte source à partir des zones détectées.

- [manga-ocr-ar-experimental](https://huggingface.co/muscgab/manga-ocr-ar-experimental) (CC BY-NC-SA 4.0)
- [Baberu OCR](https://huggingface.co/genshiai-daichi/baberu-ocr)
- [Hayai OCR](https://huggingface.co/JustANormalTinkerer/hayai-ocr-v2.7-supernova)

#### Inpainting

L'inpainting reconstruit l'image derrière le texte source avant que la traduction ne soit rendue.

<<<<<<< HEAD
- [FLUX.2 Klein](https://huggingface.co/unsloth/FLUX.2-klein-4B-GGUF)
- [Qwen Image 2.1](https://huggingface.co/leejet/Qwen-Image-2.1-GGUF)
- [Qwen Image 2.1 Uncensored](https://huggingface.co/abenzerps/Qwen-Image-2.1-Uncensored-GGUF)
- [RORem mixed](https://huggingface.co/mayocream/RORem-mixed-GGUF)
=======
>>>>>>> upstream/main
- [LaMa](https://huggingface.co/mayocream/lama-manga)
- [FLUX.3 Image Klein](https://huggingface.co/unsloth/flux-3-image-klein-GGUF)
- [Qwen-Image 2.1](https://huggingface.co/leejet/Qwen-Image-2.1-GGUF) (Qwen Research License, research and evaluation only)

### Grands modèles de langage

La traduction peut utiliser un modèle de langage local ou une API distante.

<<<<<<< HEAD
#### Modèles locaux généralistes

- Gemma 4 (QAT) : [gemma4-e2b-it](https://huggingface.co/unsloth/gemma-4-E2B-it-qat-GGUF), [gemma4-12b-it](https://huggingface.co/unsloth/gemma-4-12B-it-qat-GGUF), [gemma4-26b-a4b-it](https://huggingface.co/unsloth/gemma-4-26B-A4B-it-qat-GGUF), [gemma4-31b-it](https://huggingface.co/unsloth/gemma-4-31B-it-qat-GGUF)
- Qwen 3.5 : [qwen3.5-2b](https://huggingface.co/unsloth/Qwen3.5-2B-GGUF), [qwen3.5-4b](https://huggingface.co/unsloth/Qwen3.5-4B-GGUF), [qwen3.5-9b](https://huggingface.co/unsloth/Qwen3.5-9B-GGUF), [qwen3.5-27b](https://huggingface.co/unsloth/Qwen3.5-27B-GGUF), [qwen3.5-35b-a3b](https://huggingface.co/unsloth/Qwen3.5-35B-A3B-GGUF)
- Qwen 3.6 : [qwen3.6-27b](https://huggingface.co/unsloth/Qwen3.6-27B-GGUF), [qwen3.6-35b-a3b](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-GGUF)
- Qwen 3.8 : [qwen3.8-27b](https://huggingface.co/unsloth/Qwen3.8-27B-GGUF)
=======
#### Translation Models

- Index-Translate: [index-translate-2b](https://huggingface.co/IndexTeam/Index-Translate-2B-GGUF), [index-translate-9b](https://huggingface.co/IndexTeam/Index-Translate-9B-GGUF), [index-translate-35b-a3b-preview](https://huggingface.co/IndexTeam/Index-Translate-35B-A3B-preview-GGUF)

#### General-Purpose Local Models

- LFM3: [lfm3-1.7b](https://huggingface.co/LiquidAI/LFM3-1.7B-GGUF)
- Gemma 4: [gemma4-e2b-it](https://huggingface.co/unsloth/gemma-4-E2B-it-qat-GGUF), [gemma4-e4b-it](https://huggingface.co/unsloth/gemma-4-E4B-it-qat-GGUF), [gemma4-12b-it](https://huggingface.co/unsloth/gemma-4-12B-it-qat-GGUF), [gemma4-26b-a4b-it](https://huggingface.co/unsloth/gemma-4-26B-A4B-it-qat-GGUF), [gemma4-31b-it](https://huggingface.co/unsloth/gemma-4-31B-it-qat-GGUF)
- Qwen 3.5: [qwen3.5-0.8b](https://huggingface.co/unsloth/Qwen3.5-0.8B-GGUF), [qwen3.5-2b](https://huggingface.co/unsloth/Qwen3.5-2B-GGUF), [qwen3.5-4b](https://huggingface.co/unsloth/Qwen3.5-4B-GGUF), [qwen3.5-9b](https://huggingface.co/unsloth/Qwen3.5-9B-GGUF), [qwen3.5-27b](https://huggingface.co/unsloth/Qwen3.5-27B-GGUF), [qwen3.5-35b-a3b](https://huggingface.co/unsloth/Qwen3.5-35B-A3B-GGUF)
- Qwen 3.6: [qwen3.6-27b](https://huggingface.co/unsloth/Qwen3.6-27B-GGUF), [qwen3.6-35b-a3b](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-GGUF)
- Qwen 3.8: [qwen3.8-27b](https://huggingface.co/unsloth/Qwen3.8-27B-GGUF)
>>>>>>> upstream/main

#### Modèles locaux non censurés

- Gemma 4 non censuré : [gemma4-e4b-uncensored](https://huggingface.co/HauhauCS/Gemma-4-E4B-Uncensored-HauhauCS-Aggressive), [gemma4-12b-uncensored](https://huggingface.co/HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced), [gemma4-26b-a4b-uncensored](https://huggingface.co/HauhauCS/Gemma4-26B-A4B-QAT-Uncensored-HauhauCS-Balanced-MTP), [gemma4-31b-uncensored](https://huggingface.co/HauhauCS/Gemma4-31B-QAT-Uncensored-HauhauCS-Balanced-MTP)
- Qwen 3.5 non censuré : [qwen3.5-2b-uncensored](https://huggingface.co/HauhauCS/Qwen3.5-2B-Uncensored-HauhauCS-Aggressive), [qwen3.5-4b-uncensored](https://huggingface.co/HauhauCS/Qwen3.5-4B-Uncensored-HauhauCS-Aggressive), [qwen3.5-9b-uncensored](https://huggingface.co/HauhauCS/Qwen3.5-9B-Uncensored-HauhauCS-Aggressive)
- Qwen 3.6 non censuré : [qwen3.6-27b-uncensored](https://huggingface.co/HauhauCS/Qwen3.6-27B-Uncensored-HauhauCS-Balanced), [qwen3.6-35b-a3b-uncensored](https://huggingface.co/HauhauCS/Qwen3.6-35B-A3B-Uncensored-HauhauCS-Aggressive)
- Qwen 3.8 non censuré : [qwen3.8-27b-uncensored](https://huggingface.co/HauhauCS/Qwen3.8-27B-Uncensored-HauhauCS-Aggressive-MTP-GGUF)

<<<<<<< HEAD
#### Moteur de traduction spécialisé (texte seul)

- Hy-MT2 : famille de modèles de traduction de Tencent (Apache-2.0), texte seul — ni vision (pas de projector), ni mode reasoning — cibles limitées aux 38 langues de son entraînement (pas de pt-BR, de bulgare, de biélorusse ni de hongrois) :
  - [hy-mt2-7b](https://huggingface.co/tencent/Hy-MT2-7B-GGUF) — 7 Md de paramètres, **le défaut d'une installation neuve** : ~4,3 Gio de poids, donc entier sur une carte 8 Go, sans déport.
  - [hy-mt2-1.8b](https://huggingface.co/tencent/Hy-MT2-1.8B-GGUF) — 1,8 Md de paramètres, rapide ; choix explicite `--llm hy-mt2-1.8b` seulement (`--llm auto` ne le prend jamais, l'échelle s'arrête à la classe 2 B).
  Le 30B-A3B de la famille reste hors catalogue : son GGUF déclare `general.architecture = hy_v3`, que le llama.cpp épinglé (b9982) ne connaît pas (`llama-arch.cpp` résout `hunyuan-dense`, `hunyuan-moe`, `hunyuan_vl`), donc il ne peut pas se charger.

#### Fournisseurs cloud

Fournisseurs LLM hébergés : [OpenAI](https://platform.openai.com/), [Gemini](https://ai.google.dev/), [Claude](https://www.anthropic.com/api), [Grok](https://docs.x.ai/developers), [MiniMax](https://platform.minimax.io/), [DeepSeek](https://platform.deepseek.com/) et [OpenRouter](https://openrouter.ai/).

#### Fournisseurs de traduction automatique

Fournisseurs de traduction automatique : [DeepL](https://www.deepl.com/), [Google Cloud Translation](https://cloud.google.com/translate) et [Caiyun](https://fanyi.caiyunapp.com/).

#### Fournisseurs compatibles OpenAI

Les points d'accès compatibles OpenAI sont également pris en charge.
=======
#### Cloud Providers (BYOK)

Hosted LLM providers with an API key: [OpenAI](https://platform.openai.com/), [Anthropic](https://www.anthropic.com/api), [Google Gemini](https://ai.google.dev/), [xAI](https://docs.x.ai/developers), [DeepSeek](https://platform.deepseek.com/), [MiniMax](https://platform.minimax.io/), and [OpenRouter](https://openrouter.ai/).

Subscriptions with sign-in: ChatGPT, and Grok with a SuperGrok or X Premium plan.

#### Self-Hosted Providers

Ollama and LM Studio, at their local addresses or a base URL you set, and one OpenAI-compatible endpoint at its base URL.
>>>>>>> upstream/main

## Installation

Téléchargez les builds depuis la [page des releases de ce dépôt](https://github.com/Endymi0n74/Koharu/releases/latest) :

<<<<<<< HEAD
- **Windows** — `koharu_*_x64-setup.exe` (installateur), plus `koharu-batch.exe` en binaire autonome ;
- **Linux** — AppImage, paquets `.deb` et `.rpm` (amd64 et arm64).
=======
Builds are available for Windows (x64), macOS (Apple silicon), and Linux (x86-64 and ARM64).
>>>>>>> upstream/main

Chaque binaire est accompagné de sa signature `.sig` ; `latest.json` référence la version pour l'updater intégré. [Les prérequis d'installation et le premier lancement](https://koharu.rs/en/installation) varient selon le système d'exploitation.

> [!NOTE]
> Cette branche publie ses builds pour **Windows et Linux**. Sur macOS, construisez depuis les sources (voir **Développement** plus bas) : la jambe macOS de la CI est désactivée, mais Metal reste pris en charge.

## Dépannage

<<<<<<< HEAD
Les erreurs de démarrage, d'exécution, de modèle et de fournisseur sont traitées dans [Dépannage](https://koharu.rs/en/reference/troubleshooting). Définissez `RUST_LOG` sur `debug` ou `trace` pour des journaux verbeux :
=======
### Homebrew

Install on macOS with [Homebrew](https://brew.sh/):

```bash
brew install --cask koharu
```

## Troubleshooting

Startup, runtime, model, and provider errors are covered in [Troubleshooting](https://koharu.rs/reference/troubleshooting). Koharu writes a log file of its INFO messages and above; its location is listed in [Formats and data](https://koharu.rs/reference/data#logs). Set `RUST_LOG` to `debug` or `trace` for verbose logs in a terminal:
>>>>>>> upstream/main

```bash
# macOS / Linux
RUST_LOG=debug koharu
# Windows (PowerShell)
$env:RUST_LOG="debug"; koharu.exe
```

## Développement

Les dépendances de plateforme et les commandes de validation pour les builds locaux sont listées dans [Configuration du développement](https://koharu.rs/en/development/setup).

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) 1.97.1 ou version supérieure (édition Rust 2024)
- [Bun](https://bun.sh/) 1.3.14 ou version supérieure
- [LLVM](https://llvm.org/) 22.1.8 ou version supérieure

### Installer les dépendances

```bash
bun install
```

### Lancer en développement

```bash
bun dev
```

### Compiler

```bash
bun run build
```

L'exécutable est écrit dans `target/release`.

### Validation

Commandes de validation du dépôt :

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace --tests
bun run lint
bun run test
bun run --filter '@koharu/*' typecheck
bun scripts/check-path-portability.ts
```

## Sponsoring

Si Koharu est utile dans votre flux de travail, envisagez de soutenir le projet.

- [GitHub Sponsors](https://github.com/sponsors/mayocream)
- [Patreon](https://www.patreon.com/mayocream)

![sponsors](./.github/sponsorkit/sponsors.svg)

## Contributeurs ❤️

Merci à toutes les personnes qui ont aidé à rendre Koharu meilleur !

<a href="https://github.com/Endymi0n74/Koharu/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Endymi0n74/Koharu" />
</a>

## Licence

Copyright 2025-2026 Mayo Takanashi et les contributeurs de Koharu.

Koharu est sous licence double : la [Licence MIT](LICENSE-MIT) ou la
[Apache License, version 2.0](LICENSE-APACHE), à votre convenance.
