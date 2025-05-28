# 📦 drgrep v0.2.2 – Release Date: 2025-05-15

## 🐛 Bug Fixes

- 🛠️ **confusing with files and directories reading with the flag `-p/--path`**: Enhance the flag interpretation.
  - Fix the crash when passing the -p/--path flag.
  - Implement the appropriate logic to interpret the flag on CLI

## 📈 Improvements

- **Reintegration of the `SimplePattern` struct**: The struct is now available via import

## 🔄 Upgrade Instructions

1. Remove the existing binary (if your installation is manual).
2. Download the latest release from [GitHub Releases](https://github.com/DoniLite/drgrep/releases).
3. Add the new executable to your bin and export the path.
4. Run `drgrep --version / drgrep -v` to confirm the update.

> For more information about installation mode check the [README](https://github.com/DoniLite/drgrep#%EF%B8%8F-installation)

## 📚 Documentation

- Get the documentation [on this page](https://github.com/DoniLite/drgrep#)
- The [website](https://donilite.github.io/drgrep/) of the project

## 🙌 Acknowledgements

- 👏 thanks to [@DoniLite](https://github.com/DoniLite) the main contributor to this release
