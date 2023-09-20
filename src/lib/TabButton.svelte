<script lang="ts">
   export let id: string;
   export let text: string;
   export let tab: number;
   export let selected_tab: number;
</script>

<button
  class="tab-switcher"
  class:selected={selected_tab === tab}
  id={id}
  data-tab={tab}
  on:click={() => selected_tab = tab}
>
    {text}
</button>
