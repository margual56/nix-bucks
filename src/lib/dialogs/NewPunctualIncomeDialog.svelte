<script context="module" lang="ts">
    export function openModal() {
        (document.getElementById('dialog-punctual-income')! as HTMLDialogElement).showModal();
    }
</script>

<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { p_incomes, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";
    import { formatDate } from "../../App.svelte";

    let concept = "";
    let amount = 0;
    let date = new Date().toISOString().slice(0, 10);

    function closeModal() {
        (document.getElementById('dialog-punctual-income')! as HTMLDialogElement).close();
    }

    async function submit_income(event: HtmlFormEvent) {
        event.preventDefault();

        let tmp_subscription = {
            name: concept,
            cost: amount,
            date: formatDate(new Date(date))
        };

        invoke('add_punctual_income', {tmp: tmp_subscription}).then(async (new_p_income) => {
            $p_incomes = [...$p_incomes, new_p_income];
            
            $monthly_cost = await invoke("monthly_cost");
            $eoy_cost = await invoke("eoy_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");
        });

        closeModal();
    }
</script>

<dialog id="dialog-punctual-income">
  <h2 class="title">Add new punctual income</h2>
  <button
   class="close-modal"
   on:click={closeModal}>
    X
  </button>
  <form id="new-punctual-income-form" on:submit={submit_income}>
    <div>
      <label for="income-concept">Concept</label>
      <input
        type="text"
        id="income-concept"
        placeholder="Concept"
        style="width: 90%"
        bind:value={concept}
      />
    </div>
    <div class="container" style="margin-bottom: 1em">
      <div>
        <label for="income-cost">Cost</label>
        <input
          type="number"
          id="income-cost"
          placeholder="Cost"
          min="1"
          step="any"
          bind:value={amount}
        />
      </div>
      <div>
        <label for="income-date">Date</label>
        <input
          type="date"
          id="income-date"
          name="income-date"
          style="height: 25px"
          bind:value={date}
        />
      </div>
    </div>
    <button type="submit">Add punctual income</button>
  </form>
</dialog>
