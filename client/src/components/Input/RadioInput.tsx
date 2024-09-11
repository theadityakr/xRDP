import React, { useState } from 'react';

import '../../styles/input.css'


interface RadioInputProps {
  name: string;
  label: string;
  value: string;
  checked: boolean;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

const RadioInput: React.FC<RadioInputProps> = ({ name, label, value, checked, onChange }) => {
  return (
    <label className='flex-row'>
      <input
        type="radio"
        name={name}
        value={value}
        checked={checked}
        onChange={onChange}
      />
      {label}
    </label>
  );
};

export default RadioInput;

