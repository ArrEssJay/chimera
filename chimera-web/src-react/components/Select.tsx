import { useState, useRef, useEffect, KeyboardEvent } from 'react';
import './Select.css';

/**
 * Option for Select component
 */
export interface SelectOption {
  /**
   * Option value
   */
  value: string;
  
  /**
   * Option display label
   */
  label: string;
  
  /**
   * Whether option is disabled
   */
  disabled?: boolean;
}

/**
 * Select component props
 */
export interface SelectProps {
  /**
   * Array of options
   */
  options: SelectOption[];
  
  /**
   * Currently selected value
   */
  value?: string;
  
  /**
   * Callback when value changes
   */
  onChange?: (value: string) => void;
  
  /**
   * Placeholder text when no value selected
   */
  placeholder?: string;
  
  /**
   * Whether select is disabled
   */
  disabled?: boolean;
  
  /**
   * Optional custom class name
   */
  className?: string;
}

/**
 * Select dropdown component with keyboard navigation support.
 * Supports Arrow Up/Down, Enter, and Escape keys.
 * Fully accessible with ARIA attributes.
 * 
 * @example
 * ```tsx
 * <Select
 *   options={[
 *     { value: '1', label: 'Option 1' },
 *     { value: '2', label: 'Option 2' },
 *   ]}
 *   value={selected}
 *   onChange={setSelected}
 *   placeholder="Select an option"
 * />
 * ```
 */
export function Select({
  options,
  value,
  onChange,
  placeholder = 'Select...',
  disabled = false,
  className = '',
}: SelectProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const containerRef = useRef<HTMLDivElement>(null);
  const listboxRef = useRef<HTMLUListElement>(null);

  const selectedOption = options.find((opt) => opt.value === value);
  const enabledOptions = options.filter((opt) => !opt.disabled);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
  }, [isOpen]);

  // Reset highlighted index when dropdown opens
  useEffect(() => {
    if (isOpen) {
      const selectedIndex = enabledOptions.findIndex((opt) => opt.value === value);
      setHighlightedIndex(selectedIndex >= 0 ? selectedIndex : 0);
    }
  }, [isOpen, value, enabledOptions]);

  const handleToggle = () => {
    if (!disabled) {
      setIsOpen(!isOpen);
    }
  };

  const handleSelect = (optionValue: string) => {
    onChange?.(optionValue);
    setIsOpen(false);
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return;

    switch (event.key) {
      case 'Enter':
      case ' ':
        event.preventDefault();
        if (!isOpen) {
          setIsOpen(true);
        } else if (highlightedIndex >= 0) {
          handleSelect(enabledOptions[highlightedIndex].value);
        }
        break;

      case 'ArrowDown':
        event.preventDefault();
        if (!isOpen) {
          setIsOpen(true);
        } else {
          setHighlightedIndex((prev) =>
            prev < enabledOptions.length - 1 ? prev + 1 : prev
          );
        }
        break;

      case 'ArrowUp':
        event.preventDefault();
        if (isOpen) {
          setHighlightedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        }
        break;

      case 'Escape':
        event.preventDefault();
        setIsOpen(false);
        break;

      case 'Home':
        if (isOpen) {
          event.preventDefault();
          setHighlightedIndex(0);
        }
        break;

      case 'End':
        if (isOpen) {
          event.preventDefault();
          setHighlightedIndex(enabledOptions.length - 1);
        }
        break;
    }
  };

  // Scroll highlighted option into view
  useEffect(() => {
    if (isOpen && listboxRef.current && highlightedIndex >= 0) {
      const highlightedElement = listboxRef.current.children[highlightedIndex] as HTMLElement;
      if (highlightedElement && typeof highlightedElement.scrollIntoView === 'function') {
        highlightedElement.scrollIntoView({ block: 'nearest' });
      }
    }
  }, [highlightedIndex, isOpen]);

  const classes = [
    'chimera-select',
    isOpen && 'chimera-select--open',
    disabled && 'chimera-select--disabled',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div
      ref={containerRef}
      className={classes}
      onKeyDown={handleKeyDown}
      role="combobox"
      aria-expanded={isOpen}
      aria-haspopup="listbox"
      aria-disabled={disabled}
      tabIndex={disabled ? -1 : 0}
    >
      <div className="chimera-select__trigger" onClick={handleToggle}>
        <span className="chimera-select__value">
          {selectedOption ? selectedOption.label : placeholder}
        </span>
        <span className="chimera-select__arrow" aria-hidden="true">
          {isOpen ? '▲' : '▼'}
        </span>
      </div>

      {isOpen && (
        <ul
          ref={listboxRef}
          className="chimera-select__dropdown"
          role="listbox"
          aria-label="Options"
        >
          {options.map((option) => {
            const enabledIndex = enabledOptions.indexOf(option);
            const isHighlighted = enabledIndex === highlightedIndex;
            const isSelected = option.value === value;

            return (
              <li
                key={option.value}
                className={[
                  'chimera-select__option',
                  isHighlighted && 'chimera-select__option--highlighted',
                  isSelected && 'chimera-select__option--selected',
                  option.disabled && 'chimera-select__option--disabled',
                ]
                  .filter(Boolean)
                  .join(' ')}
                role="option"
                aria-selected={isSelected}
                aria-disabled={option.disabled}
                onClick={() => !option.disabled && handleSelect(option.value)}
                onMouseEnter={() => !option.disabled && setHighlightedIndex(enabledIndex)}
              >
                {option.label}
              </li>
            );
          })}
        </ul>
      )}
    </div>
  );
}
