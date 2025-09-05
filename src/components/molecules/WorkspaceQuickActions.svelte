<script>
    import Button from "@components/atoms/Button.svelte";
    import Icon from "@components/atoms/Icon.svelte";
    import { openAiChat } from "../../lib/stores/ai/ai-store";

</script>
<section class="flex flex-col gap-y-xs">
        <Button intent="text" class="flex w-fit items-center gap-sm px-sm" onClick={() => $openAiChat = !$openAiChat}>
            <Icon iconName="WoolyStroke" width="20"/>
            Ask Wooly
        </Button>
        <Button intent="text" class="flex w-fit items-center gap-sm px-sm">
            <Icon iconName="templates" width="20"/>
            Templates
        </Button>
    </section>