<script lang="ts">
    import { updateValue } from "../App.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from 'svelte';

    let monthly_cost: number;
    let eoy_cost: number;
    let eoy_income: number;
    let eoy_balance: number;
    let eom_balance: number;

    onMount(async () => {
        monthly_cost = await invoke("monthly_cost");
        eoy_cost = await invoke("eoy_cost");
        eoy_income = await invoke("eoy_income");
        eoy_balance = await invoke("eoy_balance");
        eom_balance = await invoke("eom_balance");
    });
</script>


<h1>My Stats</h1>
<div class="horizontal-grid">
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total average cost per month</p>
    {#if monthly_cost === undefined }
      <p>Loading...</p>
    {:else}
      <p class="amount" class:negative={monthly_cost.includes("-")}>{monthly_cost}</p>
    {/if}
  </div>
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total cost by the end of year</p>
   {#if eoy_cost=== undefined }
      <p>Loading...</p>
    {:else}
      <p class="amount" class:negative={eoy_cost.includes("-")}>{eoy_cost}</p>
    {/if} 
  </div>
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total income by the end of the year</p>
   {#if eoy_income=== undefined }
      <p>Loading...</p>
    {:else}
      <p class="amount" class:negative={eoy_income.includes("-")}>{eoy_income}</p>
    {/if} 
  </div>
  <div class="card">
    <img src="/src/assets/icon-money.svg" alt="" width="32" height="32" />
    <p class="title">Total balance by the end of year</p>
   {#if eoy_balance=== undefined }
      <p>Loading...</p>
    {:else}
      <p class="amount" class:negative={eoy_balance.includes("-")}>{eoy_balance}</p>
    {/if} 
  </div>
  <div class="card">
    <img src="/src/assets/icon-money.svg" alt="" width="32" height="32" />
    <p class="title">Total monthly balance</p>
   {#if eom_balance=== undefined }
      <p>Loading...</p>
    {:else}
      <p class="amount" class:negative={eom_balance.includes("-")}>{eom_balance}</p>
    {/if} 
  </div>
</div>
