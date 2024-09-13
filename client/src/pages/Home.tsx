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
      <div className="flex-row container">
        <Sidebar onTabChange={handleTabChange} activeTab={activeTab} />
        <div className="flex-column form-container">
          <h1>Remote Desktop Connection</h1>
          {/* Render Form only based on activeTab */}
          {activeTab === "Home" && <Form key="Home" />}
          {activeTab === "Devices" && <Devices key="Devices" />}
        </div>
      </div>
    </>
  );
};

export default Home;
