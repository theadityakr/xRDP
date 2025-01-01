import { createContext, useState, useEffect } from "react";
import { appWindow } from "@tauri-apps/api/window"; // Access the Tauri app window

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
    // Fetch system theme from Tauri or fallback to "light"
    const fetchSystemTheme = async () => {
      try {
        const systemTheme = (await appWindow.theme()) || "light"; // Fallback to "light" if null
        setTheme(systemTheme);
        document.documentElement.setAttribute("data-theme", systemTheme);
      } catch (error) {
        console.error("Error fetching system theme:", error);
        setTheme("light"); // Fallback to "light" in case of an error
        document.documentElement.setAttribute("data-theme", "light");
      }
    };

    // Check for saved theme in localStorage
    const savedTheme = localStorage.getItem("theme");
    if (savedTheme) {
      setTheme(savedTheme);
      document.documentElement.setAttribute("data-theme", savedTheme);
    } else {
      fetchSystemTheme();
    }
  }, []);

  // Toggle between light and dark themes
  const toggleTheme = () => {
    const newTheme = theme === "light" ? "dark" : "light";
    setTheme(newTheme);
    localStorage.setItem("theme", newTheme);
    document.documentElement.setAttribute("data-theme", newTheme);
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};
