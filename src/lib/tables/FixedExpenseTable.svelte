<script lang="ts">
    import { openModal } from "../dialogs/NewFixedExpenseDialog.svelte";
    import { p_expenses, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";
    import { onMount } from "svelte";
    import { invoke }  from "@tauri-apps/api/tauri";
    import type { Punctual } from "../../App.svelte";

    onMount(async () => {
        $p_expenses = await invoke("get_punctual_expenses") as Punctual[];
    });

    async function delete_p_expense(uuid: string) {
        invoke("delete_uuid", {uuid: uuid}).then(() => {
            invoke("monthly_cost").then((value) => {
                $monthly_cost = (value as string);
            });
            invoke("eoy_cost").then((value) => {
                $eoy_cost = (value as string);
            });

            invoke("eoy_income").then((value) => {
                $eoy_income = (value as string);
            });

            invoke("eoy_balance").then((value) => {
                $eoy_balance = (value as string);
            });

            invoke("eom_balance").then((value) => {
                $eom_balance = (value as string);
            });

            $p_expenses = $p_expenses.filter((expense) => expense.uuid !== uuid);
        });
    }
</script>

<h1>Fixed Expenses</h1>
<table id="table-fixed-expenses">
  <thead>
    <tr>
      <th>Concept</th>
      <th>Cost</th>
      <th>Date</th>
      <th />
    </tr>
  </thead>
  <tbody>
    {#if $p_expenses === undefined }
       <tr>
            <td>Loading...</td>
            <td>Loading...</td>
            <td>Loading...</td>
            <td>Loading...</td>
       </tr>
    {:else}
        {#each $p_expenses as p_expense}
            <tr>
                <td>{p_expense.name}</td>
                <td>{p_expense.cost}</td>
                <td>{p_expense.date}</td>
                <td>
                    <button class="delete-button" data-uuid={p_expense.uuid} on:click={() => delete_p_expense(p_expense.uuid)}>
                        <img src="/icon-delete.svg" alt="Delete" width="17" height="17" />
                        Delete
                    </button>
                </td>
            </tr>
        {/each}
    {/if}
  <tbody />
</table>
<button class="add-entry" id="add-fixed-expense" on:click={openModal}>
  <img src="/icon-add.svg" alt="" width="30" />
  Add fixed expense
</button>
