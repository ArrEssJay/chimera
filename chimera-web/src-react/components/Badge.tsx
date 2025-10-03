import { ReactNode } from 'react';
import './Badge.css';

/**
 * Badge status variants
 */
export type BadgeStatus = 'success' | 'warning' | 'error';

/**
 * Badge component props
 */
export interface BadgeProps {
  /**
   * Badge status variant
   * @default 'success'
   */
  status?: BadgeStatus;
  
  /**
   * Badge content (text)
   */
  children: ReactNode;
  
  /**
   * Optional icon to display
   */
  icon?: ReactNode;
  
  /**
   * Optional custom class name
   */
  className?: string;
}

/**
 * Badge component with 3 status variants.
 * Small, inline display for status indicators.
 * 
 * @example
 * ```tsx
 * <Badge status="success">Active</Badge>
 * <Badge status="warning" icon="⚠️">Warning</Badge>
 * <Badge status="error">Failed</Badge>
 * ```
 */
export function Badge({
  status = 'success',
  children,
  icon,
  className = '',
}: BadgeProps) {
  const classes = [
    'chimera-badge',
    `chimera-badge--${status}`,
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <span className={classes} role="status">
      {icon && <span className="chimera-badge__icon" aria-hidden="true">{icon}</span>}
      <span className="chimera-badge__text">{children}</span>
    </span>
  );
}
