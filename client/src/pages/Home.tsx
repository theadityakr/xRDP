import React, { useState } from "react";
import "../App.css";
import Form from "../components/Form.tsx";
import Tab from "../components/Tab.tsx";

const Home: React.FC = () => {
  const [activeTab, setActiveTab] = useState<string>("tab-1");

  return (
    <>
      <Tab setActiveTab={setActiveTab} />
      <div className="container">
        <div className="flex-column form-container">
          <h1>Remote Desktop Connection</h1>
          {activeTab && <Form key={activeTab} />}
        </div>
      </div>
    </>
  );
};

export default Home;
