<script lang="ts">
  import SubscriptionsTable from "./lib/tables/SubscriptionsTable.svelte";
  import FixedExpenseTable from "./lib/tables/FixedExpenseTable.svelte";
  import IncomeTable from "./lib/tables/IncomeTable.svelte";
  import PunctualIncomeTable from "./lib/tables/PunctualIncomeTable.svelte";

  import NewSubscriptionDialog from "./lib/dialogs/NewSubscriptionDialog.svelte";
  import NewIncomeDialog from "./lib/dialogs/NewIncomeDialog.svelte";
  import NewFixedExpenseDialog from "./lib/dialogs/NewFixedExpenseDialog.svelte";

  import Stats from "./lib/Stats.svelte";
  import TabButton from "./lib/TabButton.svelte";

  import {invoke} from "@tauri-apps/api/tauri";

  let selected_tab = 0;
</script>

<script context="module" lang="ts">
  export interface Subscription {
    uuid: string,
    name: string,
    cost: number,
    recurrence: string,
  };

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
  
  export async function updateTable(function_name: string, table_id: string) {
    invoke(function_name).then((value) => {
      let table = (document.getElementById(table_id)! as HTMLTableElement);
  
      table.tBodies[0].innerHTML = value + "";
  
      document.querySelectorAll(".delete-button").forEach((element) => {
        element.addEventListener("click", (event) => {
          let element = (event.target as HTMLButtonElement);
          let uuid = element.dataset.uuid!;
          let function_name = element.dataset.function!;
          let table_id = element.dataset.table!;
  
          invoke("delete_uuid", {uuid: uuid}).then(() => {
            updateTable(function_name, table_id);
          });
        });
      });
    })
  }
</script>

<main class="container">
  <div class="header">
    <img id="logo" src="/src/assets/icon-logo.svg" alt="" width="170" />

    <div class="vertical-container-right">
      <TabButton
        id={"subscriptions"}
        text={"Subscriptions"}
        tab={0}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"fixed_expenses"}
        text={"Fixed Expenses"}
        tab={1}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"income"}
        text={"Income"}
        tab={2}
        bind:selected_tab={selected_tab}
      />
      <TabButton
        id={"punctual_income"}
        text={"Punctual Income"}
        tab={3}
        bind:selected_tab={selected_tab}
      />
    </div>
  </div>

  <div class="main">
    <h3 id="savings">0.0 €</h3>
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
</main>

<NewSubscriptionDialog/>
<NewIncomeDialog />
<NewFixedExpenseDialog />
