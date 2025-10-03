import type { Meta, StoryObj } from '@storybook/react';
import { Badge } from './Badge';

const meta: Meta<typeof Badge> = {
  title: 'Components/Badge',
  component: Badge,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Badge>;

export const Success: Story = {
  args: {
    status: 'success',
    children: 'Active',
  },
};

export const Warning: Story = {
  args: {
    status: 'warning',
    children: 'Pending',
  },
};

export const Error: Story = {
  args: {
    status: 'error',
    children: 'Failed',
  },
};

export const WithIcon: Story = {
  args: {
    status: 'success',
    icon: '✓',
    children: 'Completed',
  },
};

export const WarningWithIcon: Story = {
  args: {
    status: 'warning',
    icon: '⚠️',
    children: 'Warning',
  },
};

export const ErrorWithIcon: Story = {
  args: {
    status: 'error',
    icon: '✗',
    children: 'Error',
  },
};

export const AllVariants: Story = {
  render: () => (
    <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
      <Badge status="success">Success</Badge>
      <Badge status="warning">Warning</Badge>
      <Badge status="error">Error</Badge>
    </div>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
      <Badge status="success" icon="✓">Active</Badge>
      <Badge status="warning" icon="⚠️">Pending</Badge>
      <Badge status="error" icon="✗">Failed</Badge>
    </div>
  ),
};

export const StatusIndicators: Story = {
  render: () => (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
        <span>Service Status:</span>
        <Badge status="success" icon="✓">Online</Badge>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
        <span>Build Status:</span>
        <Badge status="warning" icon="⏳">In Progress</Badge>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
        <span>Test Results:</span>
        <Badge status="error" icon="✗">Failed</Badge>
      </div>
    </div>
  ),
};

export const Numbers: Story = {
  render: () => (
    <div style={{ display: 'flex', gap: '12px' }}>
      <Badge status="success">99%</Badge>
      <Badge status="warning">12</Badge>
      <Badge status="error">0</Badge>
    </div>
  ),
};
