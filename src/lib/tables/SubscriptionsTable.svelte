<script lang="ts">
    import { openModal } from "../dialogs/NewSubscriptionDialog.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type Subscription from "../../App.svelte";
    import { subscriptions, monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "../store.ts";

    onMount(async () => {
        $subscriptions = (await invoke("get_subscriptions") as Subscription[]);
    })

    async function delete_subscription(uuid: string) {
        await invoke("delete_uuid", {uuid: uuid}).then(async () => {
            $monthly_cost = await invoke("monthly_cost");
            $eoy_cost = await invoke("eoy_cost");
            $eoy_income = await invoke("eoy_income");
            $eoy_balance = await invoke("eoy_balance");
            $eom_balance = await invoke("eom_balance");

            $subscriptions = $subscriptions.filter((s) => s.uuid !== uuid);
        });

    };
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
  <tbody>
  {#if $subscriptions === undefined}
    <tr>
        <td>Loading...</td>
        <td>Loading...</td>
        <td>Loading...</td>
        <td>Loading...</td>
    </tr>
  {:else}
    {#each $subscriptions as subscription}
      <tr>
        <td>{subscription.name}</td>
        <td>{subscription.cost}</td>
        <td>{subscription.recurrence}</td>
        <td>
        <button class="delete-button" data-uuid={subscription.uuid} on:click={() => delete_subscription(subscription.uuid)}>
            <img src="/src/assets/icon-delete.svg" alt="Delete" width="17" height="17" />
            Delete
        </button>
        </td>
      </tr>
    {/each}
  {/if}
  <tbody />
</table>

<button class="add-entry" id="add-subscription" on:click={openModal}>
  <img src="/src/assets/icon-add.svg" alt="" width="30" />
  Add subscription
    </button>
