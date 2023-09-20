<script context="module" lang="ts">
    export function openModal() {
        (document.getElementById('dialog-fixed-expense')! as HTMLDialogElement).showModal();
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
        (document.getElementById('dialog-fixed-expense')! as HTMLDialogElement).close();
    }


    function submit_expense() {
        let tmp_subscription = {
            name: concept,
            cost: amount,
            recurrence: selected,
            days: days,
            months: months,
            years: years
        };

        invoke('add_income', tmp_subscription);
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
        />
      </div>
      <div>
        <label for="expense-date">Date</label>
        <input
          type="date"
          id="expense-date"
          name="expense-date"
          style="height: 25px"
        />
      </div>
    </div>
    <button type="submit">Add fixed expense</button>
  </form>
</dialog>
