<script lang="ts">
    import { openModal } from "../dialogs/NewIncomeDialog.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    
    import type { Subscription } from "../../App.svelte";
    import { incomes, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";

    onMount(async () => {
        $incomes = (await invoke("get_incomes") as Subscription[]);
    });

    function delete_income(uuid: string) {
        invoke("delete_uuid", {uuid: uuid}).then(async () => {
            $monthly_cost = await invoke("monthly_cost");
            $eoy_cost = await invoke("eoy_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");

            $incomes = $incomes.filter((income) => income.uuid !== uuid);
        });
    }
</script>

<h1>Income streams</h1>
<table id="table-income">
  <thead>
    <tr>
      <th>Concept</th>
      <th>Cost</th>
      <th>Recurrence</th>
      <th />
    </tr>
  </thead>
  <tbody>
  {#each $incomes as income}
  <tr>
    <td>{income.name}</td>
    <td>{income.cost}</td>
    <td>{income.recurrence}</td>
    <td>
        <button class="delete-button" data-uuid={income.uuid}
                on:click={() => delete_income(income.uuid)}>
            <img src="/src/assets/icon-delete.svg" alt="Delete" width="17" height="17" />
            Delete
        </button>
  </tr>
  {/each}
  <tbody />
</table>
<button class="add-entry" id="add-income" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add income stream
</button>
