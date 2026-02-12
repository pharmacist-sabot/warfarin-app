// Run this command to generate base config and vs code settings:
// pnpm dlx @antfu/eslint-config@latest

import antfu from '@antfu/eslint-config';

export default antfu({
  type: 'app',
  ignores: [
    'warfarin_logic/**',
  ],
  vue: true,
  typescript: {
    overrides: {
      'ts/no-redeclare': 'off',
      'ts/consistent-type-definitions': ['error', 'type'],
    },
  },
  formatters: true,
  stylistic: {
    indent: 2,
    semi: true,
    quotes: 'single',
  },
}, {
  rules: {
    'no-console': ['warn'],
    'antfu/no-top-level-await': 'off',
    'perfectionist/sort-imports': ['error', {
      tsconfig: {
        rootDir: '.',
      },
    }],
    'unicorn/filename-case': ['error', {
      cases: {
        kebabCase: true,
        pascalCase: true,
      },
      ignore: ['README.md'],
    }],
  },
}).override('antfu/node/rules', {
  rules: {
    'node/prefer-global/process': 'off',
    'node/no-process-env': 'error',
  },
});
