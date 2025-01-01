import { useContext , useEffect} from "react";

import Home from './pages/Home.tsx';
import "./App.css";
import { ThemeContext } from './theme/ThemeProvider.tsx';


function App() {
  const { theme, toggleTheme } = useContext(ThemeContext);
  
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  return (
    <div className={`app-container ${theme}`}>
      <Home />
    </div>
  );
}

export default App;