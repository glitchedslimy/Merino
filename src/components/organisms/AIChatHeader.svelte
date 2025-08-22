<script lang="ts">
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
   
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { onMount } from "svelte";
    import { getOllamaModels } from "../../lib/api/tauri/get/ai-api-get";
    import { getModelLogo } from "../../lib/actions/ai/getModelLogo";

    let models: { name: string; capabilities: string[] }[] = $state([])
    let selectedModel: string = $state("llama3.1");
    let selectedIcon = $derived(getModelLogo(selectedModel));

    function selectAIModel(modelName: string) {
        selectedModel = modelName;
    }

    onMount(async () => {
        const fetchedModels = await getOllamaModels();
        if(fetchedModels.length > 0) {
            models = fetchedModels;
            const defaultModel = models.find((m) => m.name === 'llama3.1')
            selectedModel = defaultModel ? defaultModel.name : models[0].name;
        }
    })
</script>
<header class="flex justify-between gap-x-md py-md px-md">
    <Button intent="icon">
        <Icon iconName="chevron-pipe-right" width="20" />
    </Button>
    <Dropdown options={models} onSelect={selectAIModel} selectedOption={selectedModel} svgData={selectedIcon}/>
</header>