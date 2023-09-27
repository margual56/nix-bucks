<script lang="ts">
    import SubscriptionsTable from "./lib/tables/SubscriptionsTable.svelte";
    import FixedExpenseTable from "./lib/tables/FixedExpenseTable.svelte";
    import IncomeTable from "./lib/tables/IncomeTable.svelte";
    import PunctualIncomeTable from "./lib/tables/PunctualIncomeTable.svelte";
    
    import NewSubscriptionDialog from "./lib/dialogs/NewSubscriptionDialog.svelte";
    import NewIncomeDialog from "./lib/dialogs/NewIncomeDialog.svelte";
    import NewFixedExpenseDialog from "./lib/dialogs/NewFixedExpenseDialog.svelte";
    import NewPunctualIncomeDialog from "./lib/dialogs/NewPunctualIncomeDialog.svelte";
    
    import Stats from "./lib/Stats.svelte";
    import TabButton from "./lib/TabButton.svelte";
    
    import {invoke} from "@tauri-apps/api/tauri";
    import { initial_savings } from "./lib/store.ts";
    import { onMount } from "svelte";

    import { t, locale, locales } from "./i18n";
    
    let selected_tab = 0;


    onMount(() => {
        invoke("get_savings").then((value) => {
            $initial_savings = (value as string);
        });

        invoke("get_locale").then((value) => {
            $locale = (value as string);
        });
    });

    $: ($locale) => {
        invoke("set_locale", {locale: $locale});
    };
</script>

<script context="module" lang="ts">
    export interface Subscription {
        uuid: string,
        name: string,
        cost: number,
        recurrence: string,
    };

    export interface Punctual {
        uuid: string,
        name: string,
        cost: number,
        date: string,
    };

    export function formatDate(date: Date): string{
        var d = date.getDate();
        var m = date.getMonth() + 1; //Month from 0 to 11
        var y = date.getFullYear();
        return '' + (d <= 9 ? '0' + d : d) + '/' + (m<=9 ? '0' + m : m) + '/' + y;
    }

    export async function updateValue(function_name: string, element_id: string) {
      invoke(function_name).then(async (value) => {
          let element = document.getElementById(element_id)!;
          element.innerText = value + "";
    
          if(value as number < 0) {
            element.classList.add("negative");
          }else{
            element.classList.remove("negative");
          }
      });
    }
</script>

<main class="container">
  <div class="header">
    <img id="logo" src="/icon-logo.svg" alt="" width="170" />

    <div class="vertical-container-right">
      <TabButton
        id={"subscriptions"}
        text={$t("subscriptions")}
        tab={0}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"fixed_expenses"}
        text={$t("fixed_expenses")}
        tab={1}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"income"}
        text={$t("income")}
        tab={2}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"punctual_income"}
        text={$t("punctual_income")}
        tab={3}
        bind:selected_tab={selected_tab}
      />
    </div>
  </div>

  <div class="main">
    <h3 id="savings">Initial savings:&nbsp;<span class:negative={$initial_savings.includes("-")}>{$initial_savings}</span></h3>
    <div class="tab-content">
      {#if selected_tab === 0}
        <SubscriptionsTable/>
      {:else if selected_tab === 1}
        <FixedExpenseTable/>
      {:else if selected_tab === 2}
        <IncomeTable/>
      {:else if selected_tab === 3}
        <PunctualIncomeTable/>
      {/if}
    </div>
  </div>

  <div class="footer">
    <Stats/>
  </div>

  <NewSubscriptionDialog/>
  <NewIncomeDialog />
  <NewFixedExpenseDialog />
  <NewPunctualIncomeDialog />
</main>

