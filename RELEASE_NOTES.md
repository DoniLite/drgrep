# 📦 drgrep v0.3.2 – Release Date: 2025-05-15

## 🚀 New Features

- ✨ **.gitignore support**: Support for .gitignore files by excluding these files in the search context.
  1. Add a glob pattern support
  2. Include the .gitignore support in the CLI
- 🚀 **strong regex integration**: Moving to new [regex](https://crates.io/crates/regex) engine with all needed support and more performance!
  1. Add new struct for the regex support `RegexPattern`
  2. New utility functions
- 🎯 **CLI replacement in matched occurrences**: All the matched files can now be replace with any expression using regex
- 💯 **Version checker CLI**: Add command to check your installation and the current version of the program

## 🐛 Bug Fixes

- 🛠️ **Not working command for the `-c/--content` flag**: Add the flag interpretation.
  - Fix the crash when passing the -c/--content flag.
  - Implement the appropriate logic to interpret the flag on CLI
- **Remove content matching with regex in the recursive search**: This decision comes to improve content searching in the recursively the previous implementation causing performance and matching issues

## ⚠️ Breaking Changes

- ❗ **Remove the internal regex engine `SimplePattern`**: The previous regex engine have been removed and the `RegexPattern` struct comes to replace it with significant improvement and functionalities.  
If you're using the previous version of `drgrep` just with the CLI you can upgrade your installation because this don't impact the CLI

> For a API usage consider keeping your current version to not break your production env  
The v0.1.0 will be deprecated!

## 📈 Improvements

- 📊 **Search context precision with .gitignore parsing**: Enhancements to the recursive search features by removing not essential files in the context.
- **Moving to more flexible regex engine**: This will make possible to parse all supported regex expression by the [regex](https://crates.io/crates/regex) crate  
This comes with a new feature that will make possible to replace all matched occurrence in the searching

## 🔄 Upgrade Instructions

1. Remove the existing binary (if your installation is manual).
2. Download the latest release from [GitHub Releases](https://github.com/DoniLite/drgrep/releases).
3. Add the new executable to your bin and export the path.
4. Run `drgrep --version` to confirm the update.

> For more information about installation mode check the [README](https://github.com/DoniLite/drgrep#%EF%B8%8F-installation)

## 📚 Documentation

- Get the documentation [here](https://github.com/DoniLite/drgrep#)
- The [website](https://donilite.github.io/drgrep/) of the project

## 🙌 Acknowledgements

- 👏 thanks to @DoniLite the main contributor to these release
