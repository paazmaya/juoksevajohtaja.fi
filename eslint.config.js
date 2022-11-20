import paazmaya from 'eslint-config-paazmaya';
import globals from 'globals';

export default [
  paazmaya, {
    plugins: {},

    languageOptions: {
      globals: {
        ...globals.browser
      }
    }
  }
];
