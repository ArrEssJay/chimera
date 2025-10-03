import { ReactNode, useState, useRef, useEffect } from 'react';
import './Tooltip.css';

/**
 * Tooltip placement options
 */
export type TooltipPlacement = 'top' | 'right' | 'bottom' | 'left';

/**
 * Tooltip component props
 */
export interface TooltipProps {
  /**
   * Tooltip content
   */
  content: ReactNode;
  
  /**
   * Tooltip placement
   * @default 'top'
   */
  placement?: TooltipPlacement;
  
  /**
   * Element that triggers the tooltip
   */
  children: ReactNode;
  
  /**
   * Optional custom class name
   */
  className?: string;
}

/**
 * Tooltip component with 4 placement options.
 * Shows on hover and focus.
 * Fully accessible with ARIA attributes.
 * 
 * @example
 * ```tsx
 * <Tooltip content="Helpful information" placement="top">
 *   <button>Hover me</button>
 * </Tooltip>
 * ```
 */
export function Tooltip({
  content,
  placement = 'top',
  children,
  className = '',
}: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [tooltipId] = useState(() => `tooltip-${Math.random().toString(36).substr(2, 9)}`);
  const timeoutRef = useRef<number>();

  const handleMouseEnter = () => {
    clearTimeout(timeoutRef.current);
    timeoutRef.current = window.setTimeout(() => {
      setIsVisible(true);
    }, 200);
  };

  const handleMouseLeave = () => {
    clearTimeout(timeoutRef.current);
    setIsVisible(false);
  };

  const handleFocus = () => {
    setIsVisible(true);
  };

  const handleBlur = () => {
    setIsVisible(false);
  };

  useEffect(() => {
    return () => {
      clearTimeout(timeoutRef.current);
    };
  }, []);

  const classes = [
    'chimera-tooltip',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  const tooltipClasses = [
    'chimera-tooltip__content',
    `chimera-tooltip__content--${placement}`,
    isVisible && 'chimera-tooltip__content--visible',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div
      className={classes}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onFocus={handleFocus}
      onBlur={handleBlur}
    >
      <div aria-describedby={isVisible ? tooltipId : undefined}>
        {children}
      </div>
      <div
        id={tooltipId}
        role="tooltip"
        className={tooltipClasses}
        aria-hidden={!isVisible}
      >
        {content}
      </div>
    </div>
  );
}
