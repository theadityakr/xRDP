import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import '../styles/form.css'
import InputBox from "./Input/InputBox";
import ConnectButton from "./Button/Connect";


const Form: React.FC<any> = () => {
  
  const handleInputChange = (value: string) => {
    console.log('Input value:', value);
  };

  return (
    <div className="flex-column connection-details-form">
      <div className="flex-row inputfield">
        <p>Computer</p>
        <InputBox onChange={handleInputChange} placeholderName="Enter IP address , add any specific port" size="large"/>
      </div>

      <div className="flex-row">
        <p>Username</p>
        <InputBox onChange={handleInputChange} placeholderName="Enter username of the VM" size="large"/>
      </div>
      
      <ConnectButton/>
    </div>

  );
};

export default Form;