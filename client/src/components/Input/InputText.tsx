import React, { useState } from 'react';

import '../../styles/input.css'

interface InputTextProps {
  placeholderName?: string;
  onChange: (value: string) => void; 
  size?: 'small' | 'medium' | 'large';
}

const InputText: React.FC<InputTextProps> = ({onChange, size = 'medium', placeholderName = '....' }) => {
  const [name, setName] = useState<string>(''); 
  
  const InputTextClassName = `input ${size}`.trim();

  return (
    <input
      value={name}
      onChange={(e) => {
        setName(e.target.value); 
        onChange(e.target.value); 
      }}
      className={InputTextClassName}
      placeholder={placeholderName}
    />
  );
};

export default InputText;
