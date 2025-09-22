# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.1.0-beta] - 2025-09-06

### Added
- Project initialization with basic features.
- Added plugins to the Editor.
- Static store to store current used stuff in the app to save the state.
- AI Module
- Notes Module
- Folders Module
- Spaces Module
- Theming Module
- Search Module

### Fixed

### Changed
- Added more depedencies due to the plugins of the editor.
- Modified the frontend to include new features such as drag and drop functionalities-
- Modified the frontend to include note tabs.
- Modified frontend to include AI Chat.
- Modified frontend to include Space Management.
- Modified frontend to include Notes / Folders management.
- Modified frontend to include notification and toasts mechanisms.
- Modified frontend to include the Editor with the default capabilities.
- Modified the default pharagraph plugin on the editor to include markdown shortcuts capabilities.

### Removed

## [v0.2.0-beta] - 2025-09-22

### Added
- Implementation for embeds
    - Youtube
    - Codepen
    - Figma
    - More to come
- Added Quotes
- Added lists
- Added checkboxes
- Updater to the application

### Fixed

### Changed
- Ditched Markdown for JSON as is simpler to control in the app
- Marked is now only used to output in AI chat

### Removed
- Various parses that now don't have any use


## [v0.2.1-beta] - 2025-09-22

### Added
- Added some text colors

### Fixed
- Fixed utils bar name, where there wasn't any active folder there was a double slash, now its not

### Changed

### Removed
- Various parses that now don't have any use

[unreleased]: https://github.com/glitchedslimy/Merino
[0.1.0-beta]: https://github.com/glitchedslimy/Merino/releases/tag/v0.1.0-beta
[0.2.0-beta]: https://github.com/glitchedslimy/Merino/releases/tag/v0.2.0-beta
[0.2.1-beta]: https://github.com/glitchedslimy/Merino/releases/tag/v0.2.1-beta