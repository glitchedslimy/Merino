![Merino Logo](assets/banner.png)


# What is Merino? 🐑
**The _first_ note taking app where block style editing meets markdown.**

_Merino_ is a note taking app designed to be private and easy-to use, with a focus on markdown support first. It combines the structured, intuitive experience of a block-style editor with the speed and power of Markdown syntax.

> [!WARNING]
> There will be dragons, the app is on beta and there are bugs and missing features, be patience while you use this in beta state.

## Key Features ✨
- **Markdown Support**: Write notes using standard Markdown syntax. You can use headings, bold and italic text, lists, and more, all of which are rendered into rich text.

- **Block Style Editor**: Organize your thoughts with a block-based editor. Each paragraph, heading, and list item is its own block, giving you granular control over your content.

- **Privacy First**: I don't store your data on the cloud. All notes are saved locally on your device by default, giving you complete control over your information.

- **Customizable**: I believe your tools should be your own. With our built-in theming system, you can customize the look and feel of your notes to match your style.

- **AI Integration**: Explore the future of note-taking with AI. Merino has built-in support for using local AI models, with plans to support cloud-based models in the future.

## Getting Started 🍵
Since this is a `source-available` project, you can get started by building it from the source code if you're going to contribute or just like to do so.

If not, I encourage you to use the [Releases](https://github.com/glitchedslimy/Merino/releases) or [Website]() to download it.

_Be ware, and only download Merino from this same repository or the official website, **DO NOT** download it from any other source, I don't know what other people could've done to the code._

### Prerequisites 👁️
-  A modern C++ compiler (e.g, Clang 12+, GCC 10+).
- Node.js (LTS version).
- Rust.
- NPM, although you can use PNPM or yarn and should be working as well.

### Installation 💻
1. Clone the repository

`git clone https://github.com/glitchedslimy/Merino.git`

2. Navigate into the project directory:

`cd merino`

3. Install the depedencies needed:

`npm install`

4. Build the app

`npm run tauri build`

With that you should have an installer inside: `src-tauri/target/release/bundle/msi` or `src-tauri/target/release/bundle/nsi`.

## Roadmap 🛣️
I have an official roadmap for this app and it's future. Please read [ROADMAP](./ROADMAP.md) to get more information about the future of Merino.

## Docs 🗒️
They are under construction on another repository.

## Contributions 😎
This is a Source-Available project, that means, you can fork it and modify it only with the intetion of submitting back to this same repository (official one).

Follow the guidelines specified in [CONTRIBUTING](./CONTRIBUTING.md).

_If you don't follow these rules, your PR or feature request will be denied inmediatly._

## Links ⛓️‍💥
- [License (Source Only)](./LICENSE.md)
- [Contributing](./CONTRIBUTING.md)
- [Website]()