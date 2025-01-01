import ReactDOM from "react-dom/client";
import App from "./App";
import { Toaster } from "sonner";
import { ThemeProvider } from "./theme/ThemeProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <>
    <ThemeProvider>
      <Toaster
        position="bottom-right"
        richColors
        visibleToasts={9}
        toastOptions={{
          closeButton: true,
          duration: 3000,
        }}
      />
      <App />
    </ThemeProvider>
  </>
);
