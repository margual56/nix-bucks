<script>
    import { openModal } from "../dialogs/NewSubscriptionDialog.svelte";
    import { updateTable } from "../../App.svelte";

    $: updateTable("table_punctual_income", "table-punctual-income");
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
  <tbody />
</table>
<button class="add-entry" id="add-punctual-income" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add punctual income
</button>
