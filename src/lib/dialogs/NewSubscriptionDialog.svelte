
<script context="module" lang="ts">
    export function openModal() {
        (document.getElementById('dialog-subscriptions')! as HTMLDialogElement).showModal();
    }
</script>

<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    let selected = 'Month';

    let concept = "";
    let amount = 0;
    let days = 0;
    let months = 0;
    let years = 0;

    function closeModal() {
        (document.getElementById('dialog-subscriptions')! as HTMLDialogElement).close();
    }


    function submit_subscription() {
        let tmp_subscription = {
            name: concept,
            cost: amount,
            recurrence: selected,
            days: days,
            months: months,
            years: years
        };

        invoke('add_subscription', tmp_subscription);
    }
</script>

<dialog id="dialog-subscriptions">
  <h2 class="title">Add new subscription</h2>
  <button
    class="close-modal"
    on:click={closeModal}>X</button
  >
  <form id="new-subscription-form" on:submit={submit_subscription}>
    <div>
      <label for="subscription-concept">Concept</label>
      <input
        type="text"
        id="subscription-concept"
        placeholder="Concept"
        style="width: 90%"
        bind:value={concept}
      />
    </div>
    <div class="container">
      <div>
        <label for="subscription-cost">Cost</label>
        <input
          type="number"
          id="subscription-cost"
          placeholder="Cost"
          min="0"
          step="any"
          bind:value={amount}
        />
      </div>
      <div>
        <label for="subscription-recurrence">Recurrence</label>
        <select id="subscription-recurrence" name="recurrence" bind:value={selected}>
          <option value="Day">Daily</option>
          <option value="Month" selected>Monthly</option>
          <option value="Year">Yearly</option>
        </select>
      </div>
    </div>
    <div>
    {#if selected === 'daily'}
      <div class="select-div" id="daily-subscription">
        <p>
          Every <input
            type="number"
            id="subscription-days"
            placeholder="days"
            min="1"
            style="width: 60px"
            bind:value={days}
          /> days
        </p>
      </div>
    {:else if selected === 'monthly'}
      <div class="select-div" id="monthly-subscription">
        <p>
          The <input
            type="number"
            id="subscription-day"
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
            id="subscription-months"
            placeholder="months"
            min="1"
            style="width: 60px"
            bind:value={months}
          /> months
        </p>
      </div>
    {:else if selected === 'yearly'}
      <div class="select-div" id="yearly-subscription">
        <p>
          The <input
            type="number"
            id="subscription-day-2"
            placeholder="day"
            min="1"
            max="31"
            style="width: 30px"
            bind:value={days}
          />
          of month
          <input
            type="number"
            id="subscription-month"
            placeholder="month"
            min="1"
            style="width: 60px"
            bind:value={months}
          /> month
        </p>
        <p>
          Every <input
            type="number"
            id="subscription-years"
            placeholder="years"
            min="1"
            style="width: 50px"
            bind:value={years}
          /> years
        </p>
      </div>
    {/if}
    </div>

    <button type="submit">Add subscription</button>
  </form>
</dialog>
