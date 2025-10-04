import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Panel } from './Panel';

describe('Panel', () => {
  describe('Rendering', () => {
    it('renders with content', () => {
      render(<Panel>Test content</Panel>);
      expect(screen.getByText('Test content')).toBeInTheDocument();
    });

    it('renders with title', () => {
      render(<Panel title="Test Title">Content</Panel>);
      expect(screen.getByText('Test Title')).toBeInTheDocument();
    });

    it('renders with footer', () => {
      render(<Panel footer={<div>Footer content</div>}>Content</Panel>);
      expect(screen.getByText('Footer content')).toBeInTheDocument();
    });

    it('renders title, content, and footer together', () => {
      render(
        <Panel title="Title" footer={<div>Footer</div>}>
          Content
        </Panel>
      );
      expect(screen.getByText('Title')).toBeInTheDocument();
      expect(screen.getByText('Content')).toBeInTheDocument();
      expect(screen.getByText('Footer')).toBeInTheDocument();
    });
  });

  describe('Collapsible', () => {
    it('shows toggle icon when collapsible', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      expect(header).toBeInTheDocument();
      expect(header).toHaveAttribute('aria-expanded', 'true');
    });

    it('hides content when collapsed', () => {
      render(<Panel title="Title" collapsible defaultCollapsed>Content</Panel>);
      expect(screen.queryByText('Content')).not.toBeInTheDocument();
    });

    it('shows content initially when not defaultCollapsed', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      expect(screen.getByText('Content')).toBeInTheDocument();
    });

    it('toggles content on header click', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      
      expect(screen.getByText('Content')).toBeInTheDocument();
      
      fireEvent.click(header);
      expect(screen.queryByText('Content')).not.toBeInTheDocument();
      
      fireEvent.click(header);
      expect(screen.getByText('Content')).toBeInTheDocument();
    });

    it('toggles content on Enter key', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      
      fireEvent.keyDown(header, { key: 'Enter' });
      expect(screen.queryByText('Content')).not.toBeInTheDocument();
    });

    it('toggles content on Space key', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      
      fireEvent.keyDown(header, { key: ' ' });
      expect(screen.queryByText('Content')).not.toBeInTheDocument();
    });

    it('does not toggle on non-collapsible panel', () => {
      render(<Panel title="Title">Content</Panel>);
      const header = screen.getByText('Title').closest('.chimera-panel__header');
      
      fireEvent.click(header!);
      expect(screen.getByText('Content')).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('has button role when collapsible', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      expect(screen.getByRole('button')).toBeInTheDocument();
    });

    it('does not have button role when not collapsible', () => {
      render(<Panel title="Title">Content</Panel>);
      expect(screen.queryByRole('button')).not.toBeInTheDocument();
    });

    it('has aria-expanded attribute when collapsible', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      expect(header).toHaveAttribute('aria-expanded', 'true');
      
      fireEvent.click(header);
      expect(header).toHaveAttribute('aria-expanded', 'false');
    });

    it('is keyboard accessible when collapsible', () => {
      render(<Panel title="Title" collapsible>Content</Panel>);
      const header = screen.getByRole('button');
      header.focus();
      expect(header).toHaveFocus();
    });
  });

  describe('Custom Props', () => {
    it('accepts custom className', () => {
      const { container } = render(<Panel className="custom-class">Content</Panel>);
      expect(container.firstChild).toHaveClass('custom-class');
    });
  });
});
