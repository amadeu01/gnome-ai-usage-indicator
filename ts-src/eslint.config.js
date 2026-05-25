import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['src/**/*.ts'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      'no-console': 'off',
      'prefer-const': 'error',
      'no-var': 'error',
      '@typescript-eslint/no-empty-object-type': 'off',
      '@typescript-eslint/no-misused-new': 'off',
    },
  },
  {
    ignores: ['node_modules/', 'dist/', '*.d.ts'],
  }
);
