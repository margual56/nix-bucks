<script context="module" lang="ts">
    export function openModal() {
        (document.getElementById('dialog-fixed-expense')! as HTMLDialogElement).showModal();
    }
</script>

<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { p_expenses, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";
    import { formatDate } from "../../App.svelte";

    let concept = "";
    let amount = 0;
    let date = new Date().toISOString().slice(0, 10);

    function closeModal() {
        (document.getElementById('dialog-fixed-expense')! as HTMLDialogElement).close();
    }


    async function submit_expense(event: HtmlFormEvent) {
        event.preventDefault();

        let tmp_subscription = {
            name: concept,
            cost: amount,
            date: formatDate(new Date(date))
        };

        invoke('add_punctual_expense', {tmp: tmp_subscription}).then(async (new_p_expense) => {
            $p_expenses = [...$p_expenses, new_p_expense];
            
            $monthly_cost = await invoke("monthly_cost");
            $eoy_cost = await invoke("eoy_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");
        });

        closeModal();
    }
</script>

<dialog id="dialog-fixed-expense">
  <h2 class="title">Add new fixed expense</h2>
  <button
   class="close-modal"
   on:click={closeModal}>
    X
  </button>
  <form id="new-fixed-expense-form" on:submit={submit_expense}>
    <div>
      <label for="expense-concept">Concept</label>
      <input
        type="text"
        id="expense-concept"
        placeholder="Concept"
        style="width: 90%"
        bind:value={concept}
      />
    </div>
    <div class="container" style="margin-bottom: 1em">
      <div>
        <label for="expense-cost">Cost</label>
        <input
          type="number"
          id="expense-cost"
          placeholder="Cost"
          min="1"
          step="any"
          bind:value={amount}
        />
      </div>
      <div>
        <label for="expense-date">Date</label>
        <input
          type="date"
          id="expense-date"
          name="expense-date"
          style="height: 25px"
          bind:value={date}
        />
      </div>
    </div>
    <button type="submit" on:click={openModal}>Add fixed expense</button>
  </form>
</dialog>
