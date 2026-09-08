# Edunova Desktop (Tauri)

Native desktop client for the Edunova Management Intelligence System.
It opens the live Edunova platform (https://edunovamis.lovable.app) in a
fast, lightweight native window, so every platform update appears instantly.

## How to publish the installers

1. Create a new GitHub repository (for example `edunova-desktop`).
2. Upload every file from this zip to the repository root
   (`src-tauri/`, `dist/`, `.github/`, `README.md`, `.gitignore`).
3. Go to the repository **Actions** tab and enable workflows.
4. Create a release tag to trigger the build:
   - Actions → "Build Edunova Desktop (Tauri)" → Run workflow, **or**
   - push a tag: `git tag v1.0.0 && git push origin v1.0.0`
5. When the three jobs finish, a GitHub Release named
   `Edunova Desktop v1.0.0` contains:
   - `Edunova_1.0.0_x64-setup.exe` and `Edunova_1.0.0_x64_en-US.msi` (Windows)
   - `Edunova_1.0.0_universal.dmg` (macOS)
   - `Edunova_1.0.0_amd64.AppImage` and `.deb` (Linux)
6. Send the release URL back so the download page links to real files.

## Build locally (optional)

Requires Rust (https://rustup.rs) and, on Windows, the
"Desktop development with C++" workload plus WebView2.

```bash
cargo install tauri-cli --version "^2"
cargo tauri build
```

Output lands in `src-tauri/target/release/bundle/`.

## Notes

- Windows SmartScreen shows a warning for unsigned installers.
  Use "More info → Run anyway", or buy a code-signing certificate.
- Change the app URL in `src-tauri/tauri.conf.json` → `app.windows[0].url`.
