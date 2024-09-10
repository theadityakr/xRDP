import React from 'react';

import '../../styles/button.css'

interface ButtonProps {
  label: string;
  onClick: () => void;
  type?:'filled' | 'empty' ;
  size?: 'small' | 'medium' | 'large';
  shape?: 'length' | 'circle' ;
}

const Button: React.FC<ButtonProps> = ({ label, onClick ,type = 'filled',size = 'medium',shape ='length'}) => {
  
    const buttonClass = `button ${type} ${size} ${shape}`.trim();
    
  return <button onClick={onClick} className={buttonClass}>{label}</button>;
};

export default Button;