import React from 'react';
import Button from './Button';

interface ConnectButtonProps {
  onClick: () => void; // Ensure it takes onClick as a prop
}

const ConnectButton: React.FC<ConnectButtonProps> = ({ onClick }) => {
  return (
    <Button label="Connect" type="filled" size = "large" onClick={onClick} />
  );
};

export default ConnectButton;
