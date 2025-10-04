import type { Meta, StoryObj } from '@storybook/react';
import { Panel } from './Panel';

const meta: Meta<typeof Panel> = {
  title: 'Components/Panel',
  component: Panel,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Panel>;

export const Default: Story = {
  args: {
    children: <p>This is panel content. It can contain any React elements.</p>,
  },
};

export const WithTitle: Story = {
  args: {
    title: 'Panel Title',
    children: <p>Panel content with a title header.</p>,
  },
};

export const WithFooter: Story = {
  args: {
    title: 'Settings Panel',
    children: (
      <div>
        <p>Configure your settings here.</p>
        <label style={{ display: 'block', marginTop: '10px' }}>
          <input type="checkbox" /> Enable notifications
        </label>
      </div>
    ),
    footer: (
      <div style={{ display: 'flex', gap: '8px' }}>
        <button style={{ padding: '6px 12px' }}>Cancel</button>
        <button style={{ padding: '6px 12px', background: 'var(--accent)' }}>Save</button>
      </div>
    ),
  },
};

export const Collapsible: Story = {
  args: {
    title: 'Collapsible Panel',
    collapsible: true,
    children: <p>Click the header to collapse/expand this panel.</p>,
  },
};

export const CollapsedByDefault: Story = {
  args: {
    title: 'Initially Collapsed',
    collapsible: true,
    defaultCollapsed: true,
    children: <p>This panel starts collapsed.</p>,
  },
};

export const ComplexContent: Story = {
  args: {
    title: 'Statistics',
    children: (
      <div>
        <div style={{ marginBottom: '12px' }}>
          <strong>Total Users:</strong> 1,234
        </div>
        <div style={{ marginBottom: '12px' }}>
          <strong>Active Sessions:</strong> 89
        </div>
        <div>
          <strong>Success Rate:</strong> 99.5%
        </div>
      </div>
    ),
  },
};

export const MultiplePanels: Story = {
  render: () => (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', width: '400px' }}>
      <Panel title="Section 1" collapsible>
        <p>Content for section 1</p>
      </Panel>
      <Panel title="Section 2" collapsible>
        <p>Content for section 2</p>
      </Panel>
      <Panel title="Section 3" collapsible defaultCollapsed>
        <p>Content for section 3 (initially collapsed)</p>
      </Panel>
    </div>
  ),
};
