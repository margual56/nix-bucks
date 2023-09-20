import "/src/styles/styles.css";
import "/src/styles/app_styles.css";
import "/src/styles/content_styles.css"
import "/src/styles/dialog_styles.css";

import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app")!,
});

export default app;
