import { ReactNode, useState } from 'react';
import './Panel.css';

/**
 * Panel component props
 */
export interface PanelProps {
  /**
   * Panel title (header)
   */
  title?: string;
  
  /**
   * Panel content
   */
  children: ReactNode;
  
  /**
   * Optional footer content
   */
  footer?: ReactNode;
  
  /**
   * Whether panel is collapsible
   * @default false
   */
  collapsible?: boolean;
  
  /**
   * Initial collapsed state (only used if collapsible is true)
   * @default false
   */
  defaultCollapsed?: boolean;
  
  /**
   * Optional custom class name
   */
  className?: string;
}

/**
 * Panel container component with optional header and footer.
 * Supports collapsible functionality.
 * 
 * @example
 * ```tsx
 * <Panel title="Settings" footer={<Button>Save</Button>}>
 *   <p>Panel content here</p>
 * </Panel>
 * ```
 */
export function Panel({
  title,
  children,
  footer,
  collapsible = false,
  defaultCollapsed = false,
  className = '',
}: PanelProps) {
  const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);

  const handleToggle = () => {
    if (collapsible) {
      setIsCollapsed(!isCollapsed);
    }
  };

  const classes = [
    'chimera-panel',
    isCollapsed && 'chimera-panel--collapsed',
    className,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div className={classes}>
      {title && (
        <div
          className="chimera-panel__header"
          onClick={handleToggle}
          role={collapsible ? 'button' : undefined}
          tabIndex={collapsible ? 0 : undefined}
          onKeyDown={collapsible ? (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              handleToggle();
            }
          } : undefined}
          aria-expanded={collapsible ? !isCollapsed : undefined}
        >
          <h3 className="chimera-panel__title">{title}</h3>
          {collapsible && (
            <span className="chimera-panel__toggle" aria-hidden="true">
              {isCollapsed ? '▶' : '▼'}
            </span>
          )}
        </div>
      )}
      
      {!isCollapsed && (
        <>
          <div className="chimera-panel__content">{children}</div>
          {footer && <div className="chimera-panel__footer">{footer}</div>}
        </>
      )}
    </div>
  );
}
