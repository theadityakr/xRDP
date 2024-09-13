import React, { useState } from "react";
import "../App.css";
import Form from "../components/Form.tsx";
import Sidebar from "../components/Sidebar.tsx";
import Devices from "../components/Devices.tsx";

// Define a type for the possible tab values
type Tab = "Home" | "Devices";

const Home: React.FC = () => {
  const [activeTab, setActiveTab] = useState<Tab>("Home");

  // Specify the type of the tab parameter
  const handleTabChange = (tab: Tab) => {
    setActiveTab(tab);
  };

  return (
    <>
      <div className="container">
        <Sidebar onTabChange={handleTabChange} activeTab={activeTab} />
        <div className="content-container">
          {activeTab === "Home" && <Form key="Home" />}
        
        </div>
        
        {activeTab === "Devices" && <Devices key="Devices" />}
      </div>
    </>
  );
};

export default Home;
