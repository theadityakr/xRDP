import '../../styles/input.css'
import CheckboxInput from './CheckboxInput';

interface SubsectionProps {
    sectionName: string;
    options: { label: string; value: string }[];
    selectedValues: string[];
    onChange: (section: string, values: string[]) => void;
  }
  
  const SettingDialogBox: React.FC<SubsectionProps> = ({
    sectionName,
    options,
    selectedValues,
    onChange,
  }) => {
    const handleCheckboxChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      const value = event.target.value;
      let updatedValues;

      if (event.target.checked) {
        updatedValues = [...selectedValues, value];
      } else {
        updatedValues = selectedValues.filter((selected) => selected !== value);
      }
      onChange(sectionName, updatedValues); 
    };
  
    return (
      <div>
        <h4>{sectionName}</h4>
        <div className="settings-grid">
        {options.map(option => (
          <CheckboxInput
            key={option.value}
            name={sectionName}
            label={option.label}
            value={option.value}
            checked={selectedValues.includes(option.value)} 
            onChange={handleCheckboxChange}
          />
        ))}
        </div>
      </div>
    );
  };

  export default SettingDialogBox;
  