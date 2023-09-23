<script context="module" lang="ts">
    export function openModal() {
        (document.getElementById('dialog-income')! as HTMLDialogElement).showModal();
    }
</script>

<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { incomes, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";
    import type { Subscription } from "../../App.svelte";
    import type { Writable } from "svelte/store";

    let selected = 'Month';

    let concept = "";
    let amount = 0;
    let days = 1;
    let months = 1;
    let years = 1;

    function closeModal() {
        (document.getElementById('dialog-income')! as HTMLDialogElement).close();
    }

    async function submit_income(event: SubmitEvent){
        event.preventDefault();

        let tmp_subscription = {
            name: concept,
            cost: amount,
            recurrence: selected,
            days: days,
            months: months,
            years: years
        };

        invoke('add_income', {tmp: tmp_subscription}).then(async (new_income) => {
            $incomes = [...$incomes, (new_income as Subscription)];
            
            $monthly_cost = await invoke("monthly_cost");
            $eoy_cost = await invoke("eoy_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");
        });

        closeModal();
    }
</script>

<dialog id="dialog-income">
  <h2 class="title">Add new Income Source</h2>
  <button
    class="close-modal"
    on:click={closeModal}>X</button
  >
  <form id="new-income-form" on:submit={submit_income}>
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
    <div class="container">
      <div>
        <label for="income-cost">Amount</label>
        <input
          type="number"
          id="income-cost"
          placeholder="Amount"
          min="0"
          step="any"
          bind:value={amount}
        />
      </div>
      <div>
        <label for="income-recurrence">Recurrence</label>
        <select id="income-recurrence" name="recurrence" bind:value={selected}>
          <option value="Day">Daily</option>
          <option value="Month" selected>Monthly</option>
          <option value="Year">Yearly</option>
        </select>
      </div>
    </div>
    <div>
    {#if selected === 'Day'}
      <div class="select-div" id="daily-income">
        <p>
          Every <input
            type="number"
            id="income-days"
            placeholder="days"
            min="1"
            style="width: 60px"
            bind:value={days}
          /> days
        </p>
      </div>
    {:else if selected === 'Month'}
      <div class="select-div" id="monthly-income">
        <p>
          The <input
            type="number"
            id="income-day"
            placeholder="day"
            min="1"
            max="31"
            style="width: 30px"
            bind:value={days}
          /> of each month
        </p>
        <p>
          Every <input
            type="number"
            id="income-months"
            placeholder="months"
            min="1"
            style="width: 60px"
            bind:value={months}
          /> months
        </p>
      </div>
    {:else if selected === 'Year'}
      <div class="select-div" id="yearly-income">
        <p>
          The <input
            type="number"
            id="income-day-2"
            placeholder="day"
            min="1"
            max="31"
            style="width: 30px"
            bind:value={days}
          />
          of month
          <input
            type="number"
            id="income-month"
            placeholder="month"
            min="1"
            style="width: 60px"
            bind:value={months}
          /> month
        </p>
        <p>
          Every <input
            type="number"
            id="income-years"
            placeholder="years"
            min="1"
            style="width: 50px"
            bind:value={years}
          /> years
        </p>
      </div>
    {/if}
    </div>

    <button type="submit">Add income</button>
  </form>
</dialog>
