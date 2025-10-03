import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Badge } from './Badge';

describe('Badge', () => {
  describe('Rendering', () => {
    it('renders with text', () => {
      render(<Badge>Active</Badge>);
      expect(screen.getByText('Active')).toBeInTheDocument();
    });

    it('renders with default success variant', () => {
      render(<Badge>Success</Badge>);
      expect(screen.getByRole('status')).toHaveClass('chimera-badge--success');
    });

    it('renders with success variant', () => {
      render(<Badge status="success">Success</Badge>);
      expect(screen.getByRole('status')).toHaveClass('chimera-badge--success');
    });

    it('renders with warning variant', () => {
      render(<Badge status="warning">Warning</Badge>);
      expect(screen.getByRole('status')).toHaveClass('chimera-badge--warning');
    });

    it('renders with error variant', () => {
      render(<Badge status="error">Error</Badge>);
      expect(screen.getByRole('status')).toHaveClass('chimera-badge--error');
    });

    it('renders with icon', () => {
      render(<Badge icon="✓">Completed</Badge>);
      expect(screen.getByText('✓')).toBeInTheDocument();
      expect(screen.getByText('Completed')).toBeInTheDocument();
    });

    it('renders icon and text together', () => {
      render(<Badge status="warning" icon="⚠️">Warning</Badge>);
      const badge = screen.getByRole('status');
      expect(badge).toHaveTextContent('⚠️');
      expect(badge).toHaveTextContent('Warning');
    });
  });

  describe('Accessibility', () => {
    it('has status role', () => {
      render(<Badge>Status</Badge>);
      expect(screen.getByRole('status')).toBeInTheDocument();
    });

    it('icon has aria-hidden', () => {
      render(<Badge icon={<span data-testid="icon">✓</span>}>Done</Badge>);
      const icon = screen.getByTestId('icon').parentElement;
      expect(icon).toHaveAttribute('aria-hidden', 'true');
    });
  });

  describe('Custom Props', () => {
    it('accepts custom className', () => {
      render(<Badge className="custom-class">Badge</Badge>);
      expect(screen.getByRole('status')).toHaveClass('custom-class');
    });

    it('preserves base classes when custom className is provided', () => {
      render(<Badge className="custom-class" status="error">Badge</Badge>);
      const badge = screen.getByRole('status');
      expect(badge).toHaveClass('chimera-badge');
      expect(badge).toHaveClass('chimera-badge--error');
      expect(badge).toHaveClass('custom-class');
    });
  });

  describe('Content', () => {
    it('renders string content', () => {
      render(<Badge>Simple text</Badge>);
      expect(screen.getByText('Simple text')).toBeInTheDocument();
    });

    it('renders number content', () => {
      render(<Badge>{42}</Badge>);
      expect(screen.getByText('42')).toBeInTheDocument();
    });

    it('renders React element content', () => {
      render(<Badge><span>Complex content</span></Badge>);
      expect(screen.getByText('Complex content')).toBeInTheDocument();
    });
  });
});
