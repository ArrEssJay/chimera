import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { Tooltip } from './Tooltip';

describe('Tooltip', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Rendering', () => {
    it('renders children', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Hover me</button>
        </Tooltip>
      );
      expect(screen.getByRole('button')).toHaveTextContent('Hover me');
    });

    it('renders tooltip content', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Hover me</button>
        </Tooltip>
      );
      expect(screen.getByText('Tooltip text')).toBeInTheDocument();
    });

    it('tooltip is hidden by default', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Hover me</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      expect(tooltip).not.toHaveClass('chimera-tooltip__content--visible');
    });
  });

  describe('Placements', () => {
    it('renders with top placement', () => {
      render(
        <Tooltip content="Tooltip" placement="top">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveClass('chimera-tooltip__content--top');
    });

    it('renders with bottom placement', () => {
      render(
        <Tooltip content="Tooltip" placement="bottom">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveClass('chimera-tooltip__content--bottom');
    });

    it('renders with left placement', () => {
      render(
        <Tooltip content="Tooltip" placement="left">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveClass('chimera-tooltip__content--left');
    });

    it('renders with right placement', () => {
      render(
        <Tooltip content="Tooltip" placement="right">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveClass('chimera-tooltip__content--right');
    });
  });

  describe('Interactions', () => {
    it('responds to mouse enter', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Hover me</button>
        </Tooltip>
      );
      
      const container = screen.getByRole('button').closest('.chimera-tooltip');
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      
      expect(tooltip).not.toHaveClass('chimera-tooltip__content--visible');
      
      fireEvent.mouseEnter(container!);
      act(() => {
        vi.advanceTimersByTime(200);
      });
      
      expect(tooltip).toHaveClass('chimera-tooltip__content--visible');
    });

    it('responds to mouse leave', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Hover me</button>
        </Tooltip>
      );
      
      const container = screen.getByRole('button').closest('.chimera-tooltip');
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      
      fireEvent.mouseEnter(container!);
      act(() => {
        vi.advanceTimersByTime(200);
      });
      expect(tooltip).toHaveClass('chimera-tooltip__content--visible');
      
      fireEvent.mouseLeave(container!);
      expect(tooltip).not.toHaveClass('chimera-tooltip__content--visible');
    });

    it('shows tooltip on focus', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Focus me</button>
        </Tooltip>
      );
      
      const container = screen.getByRole('button').closest('.chimera-tooltip');
      fireEvent.focus(container!);
      
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveClass('chimera-tooltip__content--visible');
    });

    it('hides tooltip on blur', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Focus me</button>
        </Tooltip>
      );
      
      const container = screen.getByRole('button').closest('.chimera-tooltip');
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      
      fireEvent.focus(container!);
      expect(tooltip).toHaveClass('chimera-tooltip__content--visible');
      
      fireEvent.blur(container!);
      expect(tooltip).not.toHaveClass('chimera-tooltip__content--visible');
    });
  });

  describe('Accessibility', () => {
    it('has tooltip role', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveAttribute('role', 'tooltip');
    });

    it('has aria-hidden when not visible', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Button</button>
        </Tooltip>
      );
      const tooltip = screen.getByText('Tooltip text').closest('.chimera-tooltip__content');
      expect(tooltip).toHaveAttribute('aria-hidden', 'true');
    });

    it('has aria-describedby when visible', () => {
      render(
        <Tooltip content="Tooltip text">
          <button>Button</button>
        </Tooltip>
      );
      
      const container = screen.getByRole('button').closest('.chimera-tooltip');
      fireEvent.mouseEnter(container!);
      act(() => {
        vi.advanceTimersByTime(200);
      });
      
      const wrapper = screen.getByRole('button').parentElement;
      expect(wrapper).toHaveAttribute('aria-describedby');
    });
  });

  describe('Custom Props', () => {
    it('accepts custom className', () => {
      const { container } = render(
        <Tooltip content="Tooltip" className="custom-class">
          <button>Button</button>
        </Tooltip>
      );
      expect(container.firstChild).toHaveClass('custom-class');
    });
  });
});
