import { ButtonHTMLAttributes, ReactNode } from 'react';
import './Button.css';

/**
 * Button variant types
 */
export type ButtonVariant = 'primary' | 'secondary' | 'danger';

/**
 * Button size types
 */
export type ButtonSize = 'sm' | 'md' | 'lg';

/**
 * Button component props
 */
export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  /**
   * Button visual variant
   * @default 'primary'
   */
  variant?: ButtonVariant;
  
  /**
   * Button size
   * @default 'md'
   */
  size?: ButtonSize;
  
  /**
   * Shows loading spinner and disables button
   * @default false
   */
  loading?: boolean;
  
  /**
   * Optional icon to display before text
   */
  icon?: ReactNode;
  
  /**
   * Button content
   */
  children: ReactNode;
}

/**
 * Button component with multiple variants and sizes.
 * Supports loading state, icons, and full accessibility.
 * 
 * @example
 * ```tsx
 * <Button variant="primary" size="md" onClick={handleClick}>
 *   Click Me
 * </Button>
 * ```
 */
export function Button({
  variant = 'primary',
  size = 'md',
  loading = false,
  icon,
  children,
  disabled,
  className = '',
  ...props
}: ButtonProps) {
  const classes = [
    'chimera-button',
    `chimera-button--${variant}`,
    `chimera-button--${size}`,
    loading && 'chimera-button--loading',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <button
      className={classes}
      disabled={disabled || loading}
      aria-busy={loading}
      {...props}
    >
      {loading && (
        <span className="chimera-button__spinner" aria-label="Loading">
          ⏳
        </span>
      )}
      {!loading && icon && <span className="chimera-button__icon">{icon}</span>}
      <span className="chimera-button__text">{children}</span>
    </button>
  );
}
