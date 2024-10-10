import React from 'react';
import '../../styles/input.css';

interface InputTextProps {
  placeholderName?: string;
  onChange: (value: string) => void; 
  size?: 'small' | 'medium' | 'large';
  type?: string; // Add type prop for different input types
  value?: string; // Add value prop for controlled input
}

const InputText: React.FC<InputTextProps> = ({
  onChange,
  size = 'medium',
  placeholderName = '....',
  type = 'text', // Default type is text
  value = '', // Default value is empty string
}) => {
  const InputTextClassName = `input ${size}`.trim();

  return (
    <input
      type={type} // Use the type prop
      value={value} // Use the value prop for controlled input
      onChange={(e) => {
        onChange(e.target.value); // Call onChange with the new value
      }}
      className={InputTextClassName}
      placeholder={placeholderName}
    />
  );
};

export default InputText;
