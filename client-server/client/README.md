# Binary Viewer — Iced 0.14

Interface graphique Rust reproduisant le design "binary tunnel" avec :
- Fond d'écran full-background (fond_app.png)
- Défilement d'items binaires avec flèches ◄ ►
- 4 boutons néon : Find / Load / Save / Solve

## Prérequis

```bash
# Rust stable (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sur Linux, installez aussi les dépendances graphiques :
```bash
sudo apt install libxkbcommon-dev libwayland-dev libvulkan-dev
```

## Lancer le projet

```bash
cd binary_viewer
cargo run --release
```

> ⚠️ Lancez depuis le dossier racine du projet (là où se trouve `Cargo.toml`),
> afin que les chemins `assets/…` soient correctement résolus.

## Ajouter vos propres images

Remplacez les chaînes `BINARY_ITEMS` dans `src/main.rs` par vos vraies données.
Pour afficher des images à la place du texte binaire, utilisez :
```rust
image(image::Handle::from_path("assets/BINimg1.png"))
```

## Personnaliser les boutons

Chaque bouton utilise son image PNG :
- `assets/find.png`
- `assets/load.png`
- `assets/save.png`
- `assets/solve.png`

Implémentez la logique dans `BinaryViewer::update()` dans le bloc `match message`.

## Structure du projet

```
binary_viewer/
├── Cargo.toml
├── src/
│   └── main.rs
└── assets/
    ├── fond_app.png        ← fond d'écran principal
    ├── fond_appbin.png     ← fond alternatif
    ├── blue_arrow.png      ← flèches de navigation
    ├── find.png
    ├── load.png
    ├── save.png
    ├── solve.png
    └── BINimg1.png         ← image exemple
```
