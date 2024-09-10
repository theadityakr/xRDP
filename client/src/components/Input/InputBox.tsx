import React, { useState } from 'react';

import '../../styles/input.css'

interface InputBoxProps {
  placeholderName?: string;
  onChange: (value: string) => void; 
  size?: 'small' | 'medium' | 'large';
}

const InputBox: React.FC<InputBoxProps> = ({onChange, size = 'medium', placeholderName = '....' }) => {
  const [name, setName] = useState<string>(''); 
  
  const inputBoxClassName = `input ${size}`.trim();

  return (
    <input
      value={name}
      onChange={(e) => {
        setName(e.target.value); 
        onChange(e.target.value); 
      }}
      className={inputBoxClassName}
      placeholder={placeholderName}
    />
  );
};

export default InputBox;
