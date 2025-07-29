<script lang="ts">
  import CommandBar from "../components/organisms/commandbar/commandbar.svelte";
  import TitleBar from "../components/CustomTitleBar/TitleBar.svelte";
  import SettingsModal from "../components/organisms/settingsmodal/settingsmodal.svelte";
  import Sidebar from "../components/organisms/sidebar/sidebar.svelte";
  import Utilsbar from "../components/organisms/utilsbar/utilsbar.svelte";
  import Editor from "../components/editor/editor.svelte";
  import EditorJS from "@editorjs/editorjs";
  import { onMount } from "svelte";
  import Paragraph from "@editorjs/paragraph";
  import List from "@editorjs/list";
  import Header from "../editor-plugins/headers/header";
  import Code from "@editorjs/code";

  // State variables and functions remain the same
  let showSettingsModal = $state(false);
  let readOnly = $state(false);
  let closeMySpace = $state(false);

  function handleCloseMySpace() {
    closeMySpace = true;
  }

  function handleOpenMySpace() {
    closeMySpace = false;
  }

  function openSettings() {
    showSettingsModal = true;
  }

  function handleModalClose() {
    showSettingsModal = false;
  }

  function changeToReadOnlymode() {
    readOnly = !readOnly;
    if (editor && editor.readOnly) {
      editor.readOnly.toggle();
    }
  }

  let editor: EditorJS;

  onMount(() => {
    const holderElement = document.getElementById("editorjs");
    if (!holderElement) {
      console.error("Editor.js holder element not found!");
      return; // Exit if holder is not found
    }

    editor = new EditorJS({
      placeholder: "Vamos a empezar con algo guay?",
      holder: "editorjs",
      tools: {
        paragraph: {
          // Crucial for catching the initial input
          class: Paragraph,
          inlineToolbar: true,
        },
        header: {
          class: Header,
          inlineToolbar: true,
          config: {
            placeholder: "Enter a heading",
            levels: [1, 2, 3, 4, 5, 6],
            defaultLevel: 2,
          },
        },
        list: {
          class: List,
          inlineToolbar: true,
          config: {
            defaultStyle: "unordered",
          },
        },
        code: {
          class: Code,
          config: {
            placeholder: "Enter code here...",
          },
        },
        // ... add any other standard tools you use
      },
      i18n: {
        /* ... your i18n config ... */
      },
    });

    // --- Start: Custom Markdown-like Input Logic ---
    let isComposing = false; // To handle IME input (e.g., for CJK languages)

    holderElement.addEventListener("compositionstart", () => {
      isComposing = true;
    });

    holderElement.addEventListener("compositionend", () => {
      isComposing = false;
      // Trigger conversion check after composition ends
      if (editor && editor.blocks && editor.caret) {
        handleEditorInput();
      }
    });

    holderElement.addEventListener("input", () => {
      if (isComposing) return; // Skip if composing (IME input)
      if (editor && editor.blocks && editor.caret) {
        handleEditorInput();
      }
    });

    async function handleEditorInput() {
      // Get the current block index and the block itself
      const currentBlockIndex = editor.blocks.getCurrentBlockIndex();
      if (currentBlockIndex === -1) return; // No active block
      const currentBlock = editor.blocks.getById(
        editor.blocks.getBlockByIndex(currentBlockIndex)?.id as string,
      );

      // Only process paragraph blocks for conversion
      // Ensure it's a paragraph block and it's the *only* block
      // or the caret is at the start of a new line in an existing paragraph
      if (!currentBlock || currentBlock.name !== "paragraph") {
        return;
      }

      const text = currentBlock.holder.innerText;
      // We only want to convert if the caret is immediately after the markdown characters
      // or if the line has *just* been started with the markdown and then a space.
      // This makes it less aggressive.
      const trimmedText = text.trimStart(); // Check patterns from the beginning of the actual text

      // --- Heading Conversion (#, ##, ...) ---
      // Regex: ^(#{1,6})\s - Matches # to ###### followed by a space at the start of the line.
      const headingMatch = trimmedText.match(/^(#{1,6})\s/);
      if (headingMatch) {
        // Only if caret is at the very end
        const level = headingMatch[1].length;
        const newText = trimmedText.substring(headingMatch[0].length); // Text after "# "

        await editor.blocks.convert(currentBlock.id, "header", {
          text: newText,
          level: Math.min(level, 6), // Ensure level doesn't exceed 6
        });
        editor.caret.setToBlock(currentBlockIndex, "end"); // Place caret at end of new header
        return;
      }

      // --- List Conversion (Unordered: -, *, +) ---
      // Regex: ^([-*+])\s - Matches -, *, or + followed by a space at the start of the line.
      const ulMatch = trimmedText.match(/^([-*+])\s/);
      if (ulMatch) {
        // Only if caret is at the very end
        const newText = trimmedText.substring(ulMatch[0].length); // Text after "- " or "* "

        await editor.blocks.convert(currentBlock.id, "list", {
          style: "unordered",
          items: [{ content: newText }],
        });
        editor.caret.setToBlock(currentBlockIndex, "end");
        return;
      }

      // --- List Conversion (Ordered: 1., 2., ...) ---
      // Regex: ^(\d+\.)\s - Matches one or more digits, a dot, then a space at the start.
      const olMatch = trimmedText.match(/^(\d+\.)\s/);
      if (olMatch) {
        // Only if caret is at the very end
        const newText = trimmedText.substring(olMatch[0].length); // Text after "1. "

        await editor.blocks.convert(currentBlock.id, "list", {
          style: "ordered",
          items: [{ content: newText }],
        });
        editor.caret.setToBlock(currentBlockIndex, "end");
        return;
      }

      // --- Code Block Conversion (```) ---
      // This is still tricky with simple `input` listener, as it often requires handling 'Enter' key.
      // A direct 'input' conversion might be disruptive.
      // For three backticks, it's often better to rely on Editor.js's slash command or a custom shortcut.
      // If you absolutely need it here, you'd check for ` ``` ` at the very beginning of the line
      // when the user types the *last* backtick and then a space or newline.
      // Example (simplified and potentially unstable):
      // if (trimmedText === '```' && caretPosition === text.length) {
      //     await editor.blocks.convert(currentBlockIndex, 'code', {
      //         code: ''
      //     });
      //     editor.caret.setToBlock(currentBlockIndex, 'end');
      //     return;
      // }
    }
    // --- End: Custom Markdown-like Input Logic ---

    // Ensure cleanup of event listeners on component destroy
    return () => {
      if (editor) {
        holderElement.removeEventListener("input", handleEditorInput);
        holderElement.removeEventListener("compositionstart", () => {
          isComposing = true;
        });
        holderElement.removeEventListener("compositionend", () => {
          isComposing = false;
        });
        editor.destroy();
      }
    };
  });
</script>

<main class="min-h-screen flex flex-col bg-black-100 text-white">
  <TitleBar />
  <SettingsModal bind:showSettingsModal handleClose={handleModalClose} />
  <section class="flex flex-1 min-h-0">
    <CommandBar onOpenSettings={openSettings} class="flex-shrink-0 w-12" />
    <Sidebar
      {closeMySpace}
      {handleCloseMySpace}
      className="flex-shrink-0 w-64 md:w-72 lg:w-80"
    />
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden mx-1">
      <Utilsbar
        {editor}
        {readOnly}
        {changeToReadOnlymode}
        {closeMySpace}
        {handleOpenMySpace}
      />
      <Editor />
    </div>
  </section>
</main>
