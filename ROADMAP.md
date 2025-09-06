# Merino roadmap
This roadmap provides a _high-level_ overview of my plans for Merino note-taking app. It is dividedinto three sections:
Recently completed, immediate short-term goals and my long term vision.

## Recently completed
- **Notes**: Implemented core functionalities for notes, including loading, updating, deleting and renaming. The search functionality is now active, and bug fixes for renaming I/O errors have been resolved.
- **Folders**: The ability to create folders to organize notes is now complete. Drag-and-drop functionality for notes and folders has been implemented, and logic to handle nested folders and path updates is now in place.
- **Backend & Frontend**: Core backend architecture has been optimized to use Hexagonal + DDD + Featured-based pattern. The settings functionality has been implemented and the frontend has been refactored for better componetization.
- **AI Integration**: The AI Chat System has a better cancellation mechanism, and the AI backend has been migrated to `ollama-rs` for improved performance, also you can now manage your models directly from the app.
- **UI/UX**: A robust notification system has been created, and the command search and search panels are now fully functional.
- **Theming system**: The theming system has been finished, now you can make your own themes and put them to use in Merino.

## Short-term Goals
- **Expand AI Capabilities**: Integrate external AI models in addition to Ollama.
    - Implement internet access for local models.
    - Investigate multi-modal command processing.
    - Investigate Agent Capabilities for local models.
- **Spaces flexibility**: Allow users to utilize different "spaces" in system paths other than the default.
- **Editor enhancement**: Implement more editor and Markdown functionalities to improve the writting experience.
    - Add history for the editor to allow users go back and forth.
    - Add more markdown shortcuts to the editor.
    - Add external integrations like Figma, Google Drive...
- **Frontend Optimization**: Continue to optimize the frontedn to improve overall performance and responsiveness.


## Future Goals
- **Cloud Sync**: Implement a system for the users to sync notes across devices, probably will be done locally through WI-FI networking to don't have access to any data the user has locally.
- **Plugin System**: Create a comprehensive plugin system with a dedicated store to allow the community to build and share custom extensions.
- **Documentation**: Create the official web docs to help users get started with the project and how to use it, and develop on it.
- **Templates**: Add support for templates to streamline note creation.
- **Read-Only mode**: Implement a read-only mode for notes.
- **Quality Assurance**: Test components and backend to ensure stable and bug-free experience.
- **Quick Notes**: Add a "Quick notes" like microsoft notes functionality for rapid note-taking.
- **Develop a browser extension**: Make a browser extension to capture notes quickly from videos and other sources.