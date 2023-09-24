<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from 'svelte';
    import { monthly_cost, eoy_cost, eoy_income, eoy_balance, eom_balance } from "./store.ts";

    onMount(async () => {
        $monthly_cost = await invoke("monthly_cost");
        $eoy_cost = await invoke("eoy_cost");
        $eoy_income = await invoke("eoy_income");
        $eoy_balance = await invoke("eoy_balance");
        $eom_balance = await invoke("eom_balance");
    });
</script>


<h1>My Stats</h1>
<div class="horizontal-grid">
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total average cost per month</p>
    <p class="amount" class:negative={$monthly_cost.includes("-")}>{$monthly_cost}</p>
  </div>
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total cost by the end of year</p>
    <p class="amount" class:negative={$eoy_cost.includes("-")}>{$eoy_cost}</p>
  </div>
  <div class="card">
    <img src="/src/assets/icon-arrowup.svg" alt="" width="32" height="32" />
    <p class="title">Total income by the end of the year</p>
    <p class="amount" class:negative={$eoy_income.includes("-")}>{$eoy_income}</p>
  </div>
  <div class="card">
    <img src="/src/assets/icon-money.svg" alt="" width="32" height="32" />
    <p class="title">Total balance by the end of year</p>
    <p class="amount" class:negative={$eoy_balance.includes("-")}>{$eoy_balance}</p>
  </div>
  <div class="card">
    <img src="/src/assets/icon-money.svg" alt="" width="32" height="32" />
    <p class="title">Total monthly balance</p>
    <p class="amount" class:negative={$eom_balance.includes("-")}>{$eom_balance}</p>
  </div>
</div>
