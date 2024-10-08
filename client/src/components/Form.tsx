import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { toast } from 'sonner';

import '../styles/form.css';
import InputText from "./Input/InputText";
import ConnectButton from "./Button/Connect";
import SettingDialogBox from "./Input/SettingDialogBox";


const Form: React.FC<any> = () => {
  const [formState, setFormState] = useState({
    computer: '',
    username: '',
    password: '',
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

  const handleSubmit = async () => {

    const data = {
      computer: formState.computer,
      username: formState.username,
      password: formState.password,
      generalSettings: {
        savePassword: formState.generalSettings.includes('save_password'),
        multipleDisplay: formState.generalSettings.includes('multiple_display'),
        localDrivesRedirection: formState.generalSettings.includes('local_drives_redirection'),
      },
      advancedSettings: {
        printers: formState.advancedSettings.includes('printers'),
        clipboard: formState.advancedSettings.includes('clipboard'),
      },
    };
    try {
      data.computer = "38.126.136.103:3000";
      data.username = "Administrator";
      data.password = "Life@is@2";
      const connectionSettings =  JSON.stringify(data, null, 2);
      console.log(connectionSettings);
      invoke('connect', { connectionSettings })
      .then((message: any) => {
        console.log(message);
        toast.error(String(message));
      })
      .catch((error) => {
        console.error("Error sending form data:", error);
      });

    } catch (error) {
      console.error("Error sending form data:", error);
    }
  };

  return (
    <div className="flex-column form-container">
    <div className="flex-column connection-details-form">
      <div className="flex-col form-container-header">
        <h2>Remote Desktop Connection</h2>
        <p>Add Computer IP and Account username and password for login.</p>
      </div>
      <div className="flex-row inputfield">
        <p>Computer</p>
        <InputText
          onChange={handleInputChange('computer')}
          placeholderName="Enter IP address:port"
          size="large"
        />
      </div>

      <div className="flex-row inputfield">
        <p>Username</p>
        <InputText
          onChange={handleInputChange('username')}
          placeholderName="Enter username of the VM"
          size="large"
        />
      </div>

      <div className="flex-row inputfield">
        <p>Password</p>
        <InputText
          onChange={handleInputChange('password')}
          placeholderName="Enter password of the VM"
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

      <ConnectButton onClick={handleSubmit} />
    </div>
    </div>
  );
};

export default Form;