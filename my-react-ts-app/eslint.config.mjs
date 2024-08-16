// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  // ...tseslint.configs.recommended,  // this is the default, but instead we're trying the next two settings to keep things tighter (for now)
  ...tseslint.configs.strict,
  ...tseslint.configs.stylistic,
  {
      ignores: ["build/*"]
  }
);