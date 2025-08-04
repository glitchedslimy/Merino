<script lang="ts">
  let { iconName, width = "24", height = "24", className = "" } = $props();
  const iconModules = import.meta.glob("/src/assets/icons/*.svg", {
    query: "?raw",
    eager: true,
  });

  let svgContent: string | undefined = $derived.by(() => {
    const path = `/src/assets/icons/${iconName}.svg`;
    return iconModules[path] as string;
  });

  let finalSvg: string = $derived.by(() => {
    if (!svgContent) {
      console.warn(`Icon "${iconName}.svg" not found in assets/icons.`);
      return "";
    }

    let modifiedSvg = svgContent.default.replace(
      /<svg\s*(.*?)>/,
      (match, attrs) => {
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
