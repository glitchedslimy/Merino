# TODO List

### Backend

#### Notes
~~Implement the "read_note_file" method~~ Unnecessary
- [x] Implement the "load_note_content" method
- [x] Implement the "update_note_content" method
- [x] Implement the "delete_note" method
- [x] Implement the "rename_note" method
- [x] Fix the bug in notes: when a note is renamed, the name changes but it throws an I/O error, probably due to improper handling on the FE.
- [x] Search notes in space

---

#### Spaces
- [x] Implement the "create_space" method
- [x] Implement the "delete_space" method
- [ ] Make it possible to use different "Spaces" in different system paths, not just the default one.

---

#### AI
- [ ] Implement all "AI" functionalities of Ollama and try to modify them to also include external models.
- [ ] Implement Internet access for local models (Ollama).
- [ ] See how to implement the sidecar to use MCPs (Multi-modal Command Processing) with AI.
- [X] Implement a better cancellation mechanism for the AI chat.
- [X] Change all genai for ollama-rs only.

---

#### Folders
- [x] Implement drag-and-drop to put notes in a folder; a new function must be implemented to change the note's path to where the folder is dropped.
- [x] Implement "open and close" functionality for folders to get the notes inside or hide them when not needed.
- [x] Fix the update_note_route function for some reason even if I do the drag-and-drop doesn't change the note's route, also doesn't have into consideration the current note route, we need to have that into consideration because folders can have other folders inside it as well and we can move stuff inside there.
- [x] Changed all the app logic to include folders and nested folders, from note creation to renaming them.
- [x] Do the drag and drop functionality inside the folders as well, same as the notes.
- [X] Bug with the notes, the notes can't be dragged and dropped into the folders for some reason, it gives an empty note name.

#### Miscellaneous
- [x] Optimize the backend following the Hexagonal + DDD + **Featured based** pattern.
- [x] Implement the settings functionality by saving the file in JSON or YAML format.
- [ ] Create a plugin system and a store for it.
- [ ] Make it possible for the user to sync notes between devices (future).
- [ ] Make a theming system
- [x] Make it possible to create folders to organize notes.
- [ ] Create the documentation (Web).
- [ ] Test components and backend.
- [ ] Implement the "Quick Notes" functionality
- [ ] Fix theming system not loading the themes for some reason

### Frontend

- [x] Refactor all components and make better use of CVA and componentization.
- [x] Fix and modify functions so that calls between FE and BE work correctly.
- [x] Find a suitable architecture for the frontend for this use case.
- [ ] Optimize the frontend.
- [ ] Add the settings panel.
- [X] Implement the command panel.
- [x] Add functionality to the command panel, it will be a search panel as well at the same time.
- [x] Implement the search panel.
- [ ] Create a plugin store for **plug-and-play** plugins.
- [x] Create a notification system instead of just toasts. If a toast appears and the user doesn't close it, create a notification container for them to remain there (Hybrid System).
- [ ] Implement templates.
- [ ] Implement read-only mode.
- [ ] Implement more editor and **Markdown** functionalities.
- [ ] Implement editor history to get back and forth.
- [x] Create a drag-and-drop mechanism to order notes in folders and whatever else.