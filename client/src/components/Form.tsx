import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import '../styles/form.css'
import InputText from "./Input/InputText";
import ConnectButton from "./Button/Connect";
import SettingDialogBox from "./Input/SettingDialogBox";

const Form: React.FC<any> = () => {
  const [formState, setFormState] = useState({
    computer: '',
    username: '',
    generalSettings: [] as string[],
    advancedSettings: [] as string[],
  });
  const handleInputChange = (field: string) => (value: string) => {
    setFormState((prevState) => ({
      ...prevState,
      [field]: value,
    }));
  };
  const handleCheckboxChange = (section: string, values: string[]) => {
    setFormState((prevState) => ({
      ...prevState,
      [section]: values,
    }));
  };

  return (
    <div className="flex-column connection-details-form">
      <div className="flex-row inputfield">
        <p>Computer</p>
        <InputText
          onChange={handleInputChange('computer')}
          placeholderName="Enter IP address, add any specific port"
          size="large"
        />
      </div>

      <div className="flex-row">
        <p>Username</p>
        <InputText
          onChange={handleInputChange('username')}
          placeholderName="Enter username of the VM"
          size="large"
        />
      </div>

      <div className="dialog-box">
        <SettingDialogBox
          sectionName="General Settings"
          options={[
            { label: 'Save Password', value: 'save_password' },
            { label: 'Multiple Display', value: 'multiple_display' },
            { label: 'Local Drives Redirection', value: 'local_drives_redirection' },
          ]}
          selectedValues={formState.generalSettings}
          onChange={(section, values) => handleCheckboxChange('generalSettings', values)}
        />
      </div>

      <div className="dialog-box">
        <SettingDialogBox
          sectionName="Advanced Settings"
          options={[
            { label: 'Printers', value: 'printers' },
            { label: 'Clipboard', value: 'clipboard' },
          ]}
          selectedValues={formState.advancedSettings}
          onChange={(section, values) => handleCheckboxChange('advancedSettings', values)}
        />
      </div>

      <ConnectButton />
    </div>
  );
};

export default Form;
