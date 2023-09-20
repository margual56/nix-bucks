<script>
    import { openModal } from "../dialogs/NewFixedExpenseDialog.svelte";
    import { updateTable } from "../../App.svelte";

    $: updateTable("table_fixed_expenses", "table-fixed-expenses");
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
  <tbody />
</table>
<button class="add-entry" id="add-fixed-expense" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add fixed expense
</button>
