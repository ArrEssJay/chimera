import type { Preview } from '@storybook/react';
import '../style.css'; // Import existing styles

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: '^on[A-Z].*' },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    backgrounds: {
      default: 'dark',
      values: [
        {
          name: 'dark',
          value: 'lch(10% 0 0)', // Match existing background
        },
        {
          name: 'light',
          value: 'lch(95% 0 0)',
        },
      ],
    },
  },
};

export default preview;
