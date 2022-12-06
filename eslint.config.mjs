import paazmaya from 'eslint-config-paazmaya';
import globals from 'globals';

export default [
  paazmaya, 
  {
    languageOptions: {
      globals: {
        ...globals.browser
      }
    }
  }
];
