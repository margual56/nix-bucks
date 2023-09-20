<script>
    import { openModal } from "../dialogs/NewIncomeDialog.svelte";
    import { updateTable } from "../../App.svelte";
    
    $: updateTable("table_income", "table-income");
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
  <tbody />
</table>
<button class="add-entry" id="add-income" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add income stream
</button>
