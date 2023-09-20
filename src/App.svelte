<script lang="ts">
  import SubscriptionsTable from "./lib/tables/SubscriptionsTable.svelte";
  import FixedExpenseTable from "./lib/tables/FixedExpenseTable.svelte";
  import IncomeTable from "./lib/tables/IncomeTable.svelte";
  import PunctualIncomeTable from "./lib/tables/PunctualIncomeTable.svelte";

  import NewSubscriptionDialog from "./lib/dialogs/NewSubscriptionDialog.svelte";

  import Stats from "./lib/Stats.svelte";
  import TabButton from "./lib/TabButton.svelte";

  let selected_tab = 0;
</script>

<script context="module" lang="ts">
  async function updateValue(function_name: string, element_id: string) {
    invoke(function_name).then(async (amount) => {
      invoke("format_money", { amount: amount }).then(value => {
        let element = document.getElementById(element_id)!;
        element.innerText = value + "";
  
        if(amount as number < 0) {
          element.classList.add("negative");
        }else{
          element.classList.remove("negative");
        }
      });
    });
  }
  
  async function updateTable(function_name: string, table_id: string) {
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
<dialog id="dialog-fixed-expense">
  <h2 class="title">Add new fixed expense</h2>
  <button
    class="close-modal"
    onclick="document.getElementById('dialog-fixed-expense').close()">X</button
  >
  <form id="new-fixed-expense-form">
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
