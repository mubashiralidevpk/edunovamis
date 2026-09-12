# Edunova for iPhone & iPad (iOS)

Native iOS shell for the Edunova platform. It opens the live Edunova app, so every
platform update appears in the phone app without a new build.

This folder is **iOS only** — keep it separate from the Android and desktop releases.

## Upload to GitHub

1. Upload the contents of this folder to a repository (or a folder in your repo).
   The workflow file must end up at `.github/workflows/ios-release.yml`.
2. Open the repository's **Actions** tab, choose **Build Edunova iOS (iPhone / iPad)**,
   then **Run workflow**. Or push a tag:

   ```bash
   git tag ios-v1.0.0 && git push origin ios-v1.0.0
   ```

3. The build runs on a macOS runner and publishes a release tagged `ios`
   (or your tag) with:
   - `Edunova_1.0.0_unsigned.ipa`
   - `Edunova_1.0.0_Simulator.app.zip`

## Installing on an iPhone

Apple does not allow installing unsigned apps directly. Choose one:

- **Apple Developer account ($99/year)** — sign the `.ipa` and install via Xcode,
  TestFlight, or the App Store.
- **Sideloadly / AltStore** — sign with a free Apple ID for personal use
  (needs re-signing every 7 days).
- **Simulator** — unzip the Simulator build and drag it onto a running simulator on a Mac.

## Local build (Mac only)

```bash
npm install
npx cap add ios
npx cap sync ios
npx cap open ios     # builds and runs in Xcode
```

## Files

```
capacitor.config.json   App id, name and the live URL it loads
www/index.html          Splash / offline fallback screen
resources/              App icon and splash artwork
.github/workflows/      GitHub Actions iOS build
```

Change the live URL in `capacitor.config.json` if the production domain changes.
