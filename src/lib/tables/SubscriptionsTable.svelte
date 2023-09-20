<script>
    import { openModal } from "../dialogs/NewSubscriptionDialog.svelte";
</script>

<h1>Subscriptions</h1>
<table id="table-subscriptions">
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

<button class="add-entry" id="add-subscription" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add subscription
</button>
