import React, { useState } from "react";


import "../App.css";
import Form from "../components/Form.tsx";
import Tab from "../components/Tab.tsx";



const Home: React.FC = () => {

  
  return (
    <>
    <Tab/>
    <div className="container">
      <div className="flex-column">
        <h1>Remote Desktop Connection</h1>
        <Form />
      </div>
    </div>
    </>
  );
};

export default Home;