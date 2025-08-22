<script lang="ts">
    import type { IconName } from "../../lib/types/IconName";

  let { iconName, width = "24", height = "24", class: className }: {
    iconName: IconName | undefined,
    width?: string,
    height?: string,
    class?: string
  } = $props();

  const iconModules = import.meta.glob("/src/assets/icons/*.svg", {
    query: "?raw",
    eager: true,
  });

  let svgContent = $derived.by(() => {
    const path = `/src/assets/icons/${iconName}.svg`;
    return iconModules[path] as {
      default: string
    };
  });

  let finalSvg: string = $derived.by(() => {
    if (!svgContent) {
      console.warn(`Icon "${iconName}.svg" not found in assets/icons.`);
      return "";
    }

    let modifiedSvg = svgContent.default.replace(
      /<svg\s*(.*?)>/,
      // For some reason it needs to have the match property even if its not used.
      (_match, attrs) => {
        let newAttrs = attrs || "";
        newAttrs = newAttrs.replace(/\b(width|height|class)="[^"]*"/g, "");
        newAttrs += ` width="${width}" height="${height}" class="${className}"`;
        return `<svg ${newAttrs.trim()} stroke-width="2">`;
      },
    );

    return modifiedSvg;
  });
</script>

<div>
  {@html finalSvg}
</div>
