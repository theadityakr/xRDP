import React, { useState,useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { toast } from 'sonner';
import {IoMdEye, IoMdEyeOff,IoIosArrowDown } from "react-icons/io";

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

  const [savedLogins, setSavedLogins] = useState<any[]>([]); // Saved previous logins
  const [showDropdown, setShowDropdown] = useState(false); // Controls dropdown visibility
  const [showPassword, setShowPassword] = useState(false);

  const togglePasswordVisibility = () => {
    setShowPassword(!showPassword);
  };
  // Load saved logins from localStorage on component mount
  useEffect(() => {
    const logins = localStorage.getItem("savedLogins");
    if (logins) {
      setSavedLogins(JSON.parse(logins));
    }
  }, []);

  const handleInputChange = (field: string) => (value: string) => {
    setFormState((prevState) => ({
      ...prevState,
      [field]: value,
    }));
    if (field === 'computer') setShowDropdown(true); // Show dropdown when typing in computer field
  };

  const handleLoginSelection = (computer: string) => {
    const loginDetails = savedLogins.find(login => login.computer === computer);
    if (loginDetails) {
      setFormState({
        computer: loginDetails.computer,
        username: loginDetails.username,
        password: loginDetails.password,
        generalSettings: loginDetails.generalSettings,
        advancedSettings: loginDetails.advancedSettings,
      });
    }
    setShowDropdown(false); // Hide dropdown after selection
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
      const connectionSettings = JSON.stringify(data, null, 2);
      console.log(connectionSettings);
      invoke('connect', { connectionSettings })
        .then((message: any) => {
          console.log(message);
          message = String(message);
          if (message.split(' ')[1] === "Successful") toast.success(message);
          else toast.error(message);
        })
        .catch((error) => {
          console.error(error);
          toast.error(error);
        });

      // Save login to localStorage
      const newLogin = {
        computer: formState.computer,
        username: formState.username,
        password: formState.password,
        generalSettings: formState.generalSettings,
        advancedSettings: formState.advancedSettings,
      };

      const updatedLogins = [...savedLogins, newLogin];
      setSavedLogins(updatedLogins);
      localStorage.setItem("savedLogins", JSON.stringify(updatedLogins));

    } catch (error) {
      console.error(error);
      toast.error(String(error));
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
        <div className="dropdown-container">
          <input
                type="text"
                value={formState.computer}
                onChange={(e) => handleInputChange('computer')(e.target.value)}
                onFocus={() => setShowDropdown(true)} // Show dropdown on focus
                placeholder="Enter IP address:port "
                className="input large"
              />
              <span className="dropdown-icon"><IoIosArrowDown /></span>
              {/* Dropdown for previously saved logins */}
              {showDropdown && savedLogins.length > 0 && (
                <ul className="dropdown-list">
                  {savedLogins
                    .filter(login => login.computer.includes(formState.computer))
                    .map((login, index) => (
                      <li
                        key={index}
                        onClick={() => handleLoginSelection(login.computer)}
                        className="dropdown-item"
                      >
                        {login.computer}
                      </li>
                    ))}
                </ul>
              )}
            </div>
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
          <div className="password-container">
          <input
            type={showPassword ? "text" : "password"}
            value={formState.password}
            onChange={(e) => handleInputChange('password')(e.target.value)}
            placeholder="Enter password of the VM"
            className="password-input input large"
          />
          <span className="password-toggle-icon" onClick={togglePasswordVisibility}>
            {showPassword ? <IoMdEyeOff /> : <IoMdEye />}
          </span>
        </div>
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