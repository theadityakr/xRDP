import React, { useState } from 'react';

import '../../styles/input.css'
import RadioInput from './RadioInput';

interface SubsectionProps {
    sectionName: string;
    options: { label: string; value: string }[];
    selectedValue: string;
    onChange: (section: string, value: string) => void;
  }
  
  const SettingDialogBox: React.FC<SubsectionProps> = ({ sectionName, options, selectedValue, onChange }) => {
    const handleRadioChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      onChange(sectionName, event.target.value);
    };
  
    return (
      <div>
        <h4>{sectionName}</h4>
        <div className="settings-grid">
        {options.map(option => (
          <RadioInput
            key={option.value}
            name={sectionName}
            label={option.label}
            value={option.value}
            checked={selectedValue === option.value}
            onChange={handleRadioChange}
          />
        ))}
        </div>
      </div>
    );
  };

  export default SettingDialogBox;
  