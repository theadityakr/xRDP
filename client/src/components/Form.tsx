import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import '../styles/form.css'
import InputBox from "./Input/InputBox";
import ConnectButton from "./Button/Connect";

interface FormProps {
  onGreet: (message: string) => void;
}

const Form: React.FC<FormProps> = () => {
  
  const handleInputChange = (value: string) => {
    console.log('Input value:', value);
  };

  return (
    <div>
      <InputBox onChange={handleInputChange} placeholderName="testing/...." size="large"/>
      <ConnectButton/>
      </div>

  );
};

export default Form;