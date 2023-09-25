<script lang="ts">
    import { openModal } from "../dialogs/NewPunctualIncomeDialog.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Punctual } from "../../App.svelte";
    import { p_incomes, monthly_cost, yearly_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";
    import { onMount } from "svelte";

    onMount(async () => {
        $p_incomes = await invoke("get_punctual_incomes") as Punctual[];
    });

    async function delete_p_income(uuid: string) {
        invoke("delete_uuid", {uuid: uuid}).then(async () => {
            $monthly_cost = await invoke("monthly_cost");
            $yearly_cost= await invoke("yearly_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");

            $p_incomes = $p_incomes.filter((income) => income.uuid !== uuid);
        });
    }
</script>


<h1>Punctual Income</h1>
<table id="table-punctual-income">
  <thead>
    <tr>
      <th>Concept</th>
      <th>Cost</th>
      <th>Date</th>
      <th />
    </tr>
  </thead>
  <tbody>
    {#if $p_incomes === undefined }
       <tr>
            <td>Loading...</td>
            <td>Loading...</td>
            <td>Loading...</td>
            <td>Loading...</td>
       </tr>
    {:else}
        {#each $p_incomes as p_income}
            <tr>
                <td>{p_income.name}</td>
                <td>{p_income.cost}</td>
                <td>{p_income.date}</td>
                <td>
                    <button class="delete-button" data-uuid={p_income.uuid} on:click={() => delete_p_income(p_income.uuid)}>
                        Delete
                    </button>
                </td>
            </tr>
        {/each}
    {/if}
  <tbody />
</table>
<button class="add-entry" id="add-punctual-income" on:click={openModal}>
  <img src="/icon-add.svg" alt="" width="30" />
  Add punctual income
</button>
