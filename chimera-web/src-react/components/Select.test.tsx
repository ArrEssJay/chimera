import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Select } from './Select';

const mockOptions = [
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
  { value: '4', label: 'Disabled Option', disabled: true },
];

describe('Select', () => {
  describe('Rendering', () => {
    it('renders with placeholder', () => {
      render(<Select options={mockOptions} placeholder="Choose one" />);
      expect(screen.getByText('Choose one')).toBeInTheDocument();
    });

    it('renders with selected value', () => {
      render(<Select options={mockOptions} value="2" />);
      expect(screen.getByText('Option 2')).toBeInTheDocument();
    });

    it('renders combobox with correct attributes', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      expect(combobox).toHaveAttribute('aria-expanded', 'false');
      expect(combobox).toHaveAttribute('aria-haspopup', 'listbox');
      expect(combobox).toHaveAttribute('tabIndex', '0');
    });
  });

  describe('Selection', () => {
    it('updates value display when value prop changes', () => {
      const { rerender } = render(<Select options={mockOptions} value="1" />);
      expect(screen.getByText('Option 1')).toBeInTheDocument();
      
      rerender(<Select options={mockOptions} value="2" />);
      expect(screen.getByText('Option 2')).toBeInTheDocument();
    });
  });

  describe('Keyboard Navigation', () => {
    it('responds to ArrowDown key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'ArrowDown' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });

    it('responds to ArrowUp key when open', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'ArrowDown' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
      
      fireEvent.keyDown(combobox, { key: 'ArrowUp' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });

    it('responds to Escape key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'ArrowDown' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
      
      fireEvent.keyDown(combobox, { key: 'Escape' });
      expect(combobox).toHaveAttribute('aria-expanded', 'false');
    });

    it('responds to Enter key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'Enter' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });

    it('responds to Space key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: ' ' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });

    it('responds to Home key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'ArrowDown' });
      fireEvent.keyDown(combobox, { key: 'Home' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });

    it('responds to End key', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      
      fireEvent.keyDown(combobox, { key: 'ArrowDown' });
      fireEvent.keyDown(combobox, { key: 'End' });
      expect(combobox).toHaveAttribute('aria-expanded', 'true');
    });
  });

  describe('Disabled State', () => {
    it('is disabled when disabled prop is true', () => {
      render(<Select options={mockOptions} disabled />);
      const combobox = screen.getByRole('combobox');
      expect(combobox).toHaveAttribute('aria-disabled', 'true');
      expect(combobox).toHaveAttribute('tabIndex', '-1');
    });

    it('does not open dropdown when disabled', () => {
      render(<Select options={mockOptions} disabled />);
      const combobox = screen.getByRole('combobox');
      fireEvent.click(combobox);
      expect(screen.queryByRole('listbox')).not.toBeInTheDocument();
    });

    it('does not respond to keyboard when disabled', () => {
      render(<Select options={mockOptions} disabled />);
      const combobox = screen.getByRole('combobox');
      fireEvent.keyDown(combobox, { key: 'Enter' });
      expect(screen.queryByRole('listbox')).not.toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('has combobox role', () => {
      render(<Select options={mockOptions} />);
      expect(screen.getByRole('combobox')).toBeInTheDocument();
    });

    it('has aria-haspopup attribute', () => {
      render(<Select options={mockOptions} />);
      expect(screen.getByRole('combobox')).toHaveAttribute('aria-haspopup', 'listbox');
    });

    it('is keyboard accessible', () => {
      render(<Select options={mockOptions} />);
      const combobox = screen.getByRole('combobox');
      combobox.focus();
      expect(combobox).toHaveFocus();
    });

    it('has proper aria-disabled when disabled', () => {
      render(<Select options={mockOptions} disabled />);
      expect(screen.getByRole('combobox')).toHaveAttribute('aria-disabled', 'true');
    });
  });

  describe('Custom Props', () => {
    it('accepts custom className', () => {
      render(<Select options={mockOptions} className="custom-class" />);
      expect(screen.getByRole('combobox')).toHaveClass('custom-class');
    });

    it('shows placeholder when no value', () => {
      render(<Select options={mockOptions} placeholder="Pick one" />);
      expect(screen.getByText('Pick one')).toBeInTheDocument();
    });
  });
});
