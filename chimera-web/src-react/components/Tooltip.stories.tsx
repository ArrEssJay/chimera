import type { Meta, StoryObj } from '@storybook/react';
import { Tooltip } from './Tooltip';

const meta: Meta<typeof Tooltip> = {
  title: 'Components/Tooltip',
  component: Tooltip,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Tooltip>;

export const Top: Story = {
  args: {
    content: 'This is a tooltip on top',
    placement: 'top',
    children: <button style={{ padding: '8px 16px' }}>Hover me (top)</button>,
  },
};

export const Bottom: Story = {
  args: {
    content: 'This is a tooltip on bottom',
    placement: 'bottom',
    children: <button style={{ padding: '8px 16px' }}>Hover me (bottom)</button>,
  },
};

export const Left: Story = {
  args: {
    content: 'This is a tooltip on left',
    placement: 'left',
    children: <button style={{ padding: '8px 16px' }}>Hover me (left)</button>,
  },
};

export const Right: Story = {
  args: {
    content: 'This is a tooltip on right',
    placement: 'right',
    children: <button style={{ padding: '8px 16px' }}>Hover me (right)</button>,
  },
};

export const LongContent: Story = {
  args: {
    content: 'This is a longer tooltip with more detailed information',
    placement: 'top',
    children: <button style={{ padding: '8px 16px' }}>Long tooltip</button>,
  },
};

export const AllPlacements: Story = {
  render: () => (
    <div style={{ 
      display: 'grid', 
      gridTemplateColumns: 'repeat(2, 1fr)', 
      gap: '60px',
      padding: '60px'
    }}>
      <Tooltip content="Top tooltip" placement="top">
        <button style={{ padding: '8px 16px' }}>Top</button>
      </Tooltip>
      <Tooltip content="Right tooltip" placement="right">
        <button style={{ padding: '8px 16px' }}>Right</button>
      </Tooltip>
      <Tooltip content="Bottom tooltip" placement="bottom">
        <button style={{ padding: '8px 16px' }}>Bottom</button>
      </Tooltip>
      <Tooltip content="Left tooltip" placement="left">
        <button style={{ padding: '8px 16px' }}>Left</button>
      </Tooltip>
    </div>
  ),
};

export const WithIcon: Story = {
  args: {
    content: 'Click for more information',
    placement: 'top',
    children: (
      <button style={{ 
        padding: '8px', 
        borderRadius: '50%',
        width: '32px',
        height: '32px',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center'
      }}>
        ?
      </button>
    ),
  },
};
