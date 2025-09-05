<script>
    import Button from "@components/atoms/Button.svelte";
    import Dropdown from "@components/atoms/Dropdown.svelte";
    import { fade } from "svelte/transition";

    const dropdownOptions = [
        {name: "Spanish"},
        {name: "English"}
    ]
</script>
<div class="w-full">
    <p class="mb-sm">Application</p>
    <div class="flex items-center justify-between mb-md">
        <div>
            <p class="text-sm">Application Version: v0.1.0-beta</p>
            <p class="text-sm text-black-300">Your app is up to date!</p>
            <a href="google.com" class="text-sm text-brand-primary underline">Read the change logs</a>
        </div>
        <div>
            <Button intent="primary">
                Search updates
            </Button>
        </div>
    </div>
    <div class="flex items-center justify-between">
        <div>
            <p class="text-sm">Language</p>
            <p class="text-sm text-black-300">Change the language to display. <b>(Not functional right now.)</b></p>
        </div>
        <div>
           <Dropdown options={dropdownOptions} selectedOption={"English"}/>
        </div>
    </div>
</div>