import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import '../styles/form.css'
import InputText from "./Input/InputText";
import ConnectButton from "./Button/Connect";
import SettingDialogBox from "./Input/SettingDialogBox";

const Form: React.FC<any> = () => {
  
  const handleInputChange = (value: string) => {
    console.log('Input value:', value);
  };

  return (
    <div className="flex-column connection-details-form">
      <div className="flex-row inputfield">
        <p>Computer</p>
        <InputText onChange={handleInputChange} placeholderName="Enter IP address , add any specific port" size="large"/>
      </div>

      <div className="flex-row">
        <p>Username</p>
        <InputText onChange={handleInputChange} placeholderName="Enter username of the VM" size="large"/>
      </div>
      <div className="dialog-box">
      <SettingDialogBox
        sectionName="General Settings"
        options={[
          { label: 'Save Password', value: 'option1' },
          { label: 'Multiple Display', value: 'option2' },
          { label: 'Local Drives Redirection', value: 'option2' },
        ]}
        selectedValue='{formState.section1}'
        onChange={handleInputChange}
      />
    </div>
    <div className="dialog-box">
    <SettingDialogBox
        sectionName="Advanced Settings"
        options={[
          { label: 'Printers', value: 'option1' },
          { label: 'Clipboard', value: 'option2' },
        ]}
        selectedValue='{formState.section1}'
        onChange={handleInputChange}
      />
      </div>
      <ConnectButton/>
    </div>

  );
};

export default Form;