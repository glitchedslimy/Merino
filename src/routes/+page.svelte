<script lang="ts">
  import CommandBar from "../components/organisms/commandbar/commandbar.svelte";
  import TitleBar from "../components/CustomTitleBar/TitleBar.svelte";
  import SettingsModal from "../components/organisms/settingsmodal/settingsmodal.svelte";
  import Sidebar from "../components/organisms/sidebar/sidebar.svelte";
  import Utilsbar from "../components/organisms/utilsbar/utilsbar.svelte";
  import Editor from "../components/editor/editor.svelte";

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
    if (readOnly) {
      editor.readOnly.toggle();
    } else {
      editor.readOnly.toggle();
    }
  }

  import EditorJS from "@editorjs/editorjs";
  const editor = new EditorJS({
    placeholder: "Vamos a empezar con algo guay?",
    holder: "editorjs",
    i18n: {
      messages: {
        ui: {
          inlineToolbar: {
            "Convert to": "Convertir a",
          },
          toolbar: {
            toolbox: {
              Add: "Añadir",
            },
          },
          blockTunes: {
            toggler: {
              "Click to tune": "Click para modificar",
            },
          },
        },
      },
    },
  });
</script>

<main>
  <TitleBar />
  <SettingsModal bind:showSettingsModal handleClose={handleModalClose} />
  <section class="flex">
    <CommandBar onOpenSettings={openSettings} />
    <Sidebar {closeMySpace} {handleCloseMySpace}/>
    <div class="relative max-w-full w-full">
      <Utilsbar {editor} {readOnly} {changeToReadOnlymode} {closeMySpace} {handleOpenMySpace}/>
      <Editor />
    </div>
  </section>
</main>
