import { createContext, useState, useEffect } from "react";
import { appWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";

export const ThemeContext = createContext<{
  theme: string;
  toggleTheme: () => void;
}>({
  theme: "light",
  toggleTheme: () => {},
});

export const ThemeProvider = ({ children }: { children: React.ReactNode }) => {
  const [theme, setTheme] = useState<string>("light");

  useEffect(() => {
    const fetchSystemTheme = async () => {
      try {
        const systemTheme = (await appWindow.theme()) || "light";
        setTheme(systemTheme);
        document.documentElement.setAttribute("class", systemTheme);
      } catch (error) {
        console.error("Error fetching system theme:", error);
        setTheme("light");
        document.documentElement.setAttribute("class", "light");
      }
    };

    fetchSystemTheme();

    // Listen for system theme changes
    const unlisten = listen<string>("tauri://theme-changed", (event) => {
      const newTheme = event.payload || "light";
      setTheme(newTheme);
      document.documentElement.setAttribute("class", newTheme);
    });

    // Cleanup the listener when the component is unmounted
    return () => {
      unlisten.then((dispose) => dispose());
    };
  }, []);

  const toggleTheme = () => {
    const newTheme = theme === "light" ? "dark" : "light";
    setTheme(newTheme);
    document.documentElement.setAttribute("class", newTheme);
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};
