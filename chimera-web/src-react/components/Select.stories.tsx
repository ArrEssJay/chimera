import { useState } from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import { Select, SelectOption } from './Select';

const meta: Meta<typeof Select> = {
  title: 'Components/Select',
  component: Select,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    disabled: {
      control: 'boolean',
      description: 'Disables the select',
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text',
    },
  },
};

export default meta;
type Story = StoryObj<typeof Select>;

const defaultOptions: SelectOption[] = [
  { value: '1', label: 'Option 1' },
  { value: '2', label: 'Option 2' },
  { value: '3', label: 'Option 3' },
  { value: '4', label: 'Option 4' },
  { value: '5', label: 'Option 5' },
];

// Default story
export const Default: Story = {
  args: {
    options: defaultOptions,
    placeholder: 'Select an option...',
  },
};

// With selected value
export const WithSelectedValue: Story = {
  args: {
    options: defaultOptions,
    value: '2',
    placeholder: 'Select an option...',
  },
};

// Disabled
export const Disabled: Story = {
  args: {
    options: defaultOptions,
    disabled: true,
    placeholder: 'Disabled select',
  },
};

// With disabled options
export const WithDisabledOptions: Story = {
  args: {
    options: [
      { value: '1', label: 'Option 1' },
      { value: '2', label: 'Option 2 (disabled)', disabled: true },
      { value: '3', label: 'Option 3' },
      { value: '4', label: 'Option 4 (disabled)', disabled: true },
      { value: '5', label: 'Option 5' },
    ],
    placeholder: 'Select an option...',
  },
};

// Many options (scrollable)
export const ManyOptions: Story = {
  args: {
    options: Array.from({ length: 20 }, (_, i) => ({
      value: `${i + 1}`,
      label: `Option ${i + 1}`,
    })),
    placeholder: 'Select from many options...',
  },
};

// Interactive example with state
export const Interactive: Story = {
  render: () => {
    const [selected, setSelected] = useState('');
    
    const options: SelectOption[] = [
      { value: 'react', label: 'React' },
      { value: 'vue', label: 'Vue' },
      { value: 'angular', label: 'Angular' },
      { value: 'svelte', label: 'Svelte' },
    ];

    return (
      <div style={{ width: '300px' }}>
        <div style={{ marginBottom: '16px' }}>
          <Select
            options={options}
            value={selected}
            onChange={setSelected}
            placeholder="Choose your framework"
          />
        </div>
        <div style={{ 
          padding: '12px', 
          background: 'var(--bg-overlay)', 
          borderRadius: '4px',
          fontSize: '14px',
          color: 'var(--text-muted)'
        }}>
          Selected: {selected || '(none)'}
        </div>
      </div>
    );
  },
};

// Custom styling
export const CustomWidth: Story = {
  render: () => (
    <div style={{ width: '400px' }}>
      <Select
        options={defaultOptions}
        placeholder="Full width select"
        className="custom-select"
      />
    </div>
  ),
};
