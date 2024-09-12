import React from 'react';

import '../../styles/input.css';

interface CheckboxInputProps {
  name: string;
  label: string;
  value: string;
  checked: boolean;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

const CheckboxInput: React.FC<CheckboxInputProps> = ({ name, label, value, checked, onChange }) => {
  return (
    <label className='flex-row'>
      <input
        type="checkbox"
        name={name}
        value={value}
        checked={checked}
        onChange={onChange}
      />
      {label}
    </label>
  );
};

export default CheckboxInput;