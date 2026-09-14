# Edunova Mobile (Capacitor)

Native Android and iOS shell for the Edunova platform. It loads the live published
platform, so every web update reaches phones without a new app build.

## Upload to GitHub

1. Upload **all** files in this folder to your repository (a separate repo, e.g.
   `edunova-mobile`, is cleanest). Make sure the hidden `.github/workflows/` folder
   is included — GitHub's drag-and-drop web upload sometimes skips it. If it does,
   create the file manually at `.github/workflows/mobile-release.yml`.
2. Go to **Actions → Build Edunova Mobile (Android) → Run workflow**, or push a tag:

```bash
git tag mobile-v1.0.0 && git push origin mobile-v1.0.0
```

3. The workflow attaches `Edunova_1.0.0.apk` to a GitHub Release. Send me the
   release link and I will put the download button on the website.

## Build locally

```bash
npm install
npx cap add android     # and/or: npx cap add ios
npx cap sync
npx cap open android    # Android Studio
npx cap open ios        # Xcode, macOS only
```

## Google Play / App Store

- Play Store needs a signed **AAB**: create an upload keystore, add it as repo
  secrets, and run `./gradlew bundleRelease` in the `android` folder.
- App Store builds require macOS with Xcode and an Apple Developer account.

## Permissions

The app needs **Camera** (for QR check-in scanning) and **Location** (for
gps-verified teacher check-ins) permissions. The build workflow automatically
injects these into the Android manifest. For iOS, add these keys to
`ios/App/App/Info.plist` after running `npx cap add ios`:

```xml
<key>NSCameraUsageDescription</key>
<string>Edunova uses the camera to scan school check-in QR codes.</string>
<key>NSLocationWhenInUseUsageDescription</key>
<string>Edunova uses your location to verify you are inside the school geofence.</string>
```

## Configuration

`capacitor.config.json` holds the app id (`com.edunova.app`), app name, splash
colours and the live platform URL. Change the `server.url` if the production
domain changes.
