import { heroui } from '@heroui/react';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';

// @heroui/theme is a transitive dependency of @heroui/react, and npm may either
// hoist it to the top level or nest it under @heroui/react. Resolve it from
// @heroui/react so Tailwind scans HeroUI's component classes wherever it lands.
const require = createRequire(import.meta.url);
const herouiThemeGlob = join(
  dirname(
    require.resolve('@heroui/theme/package.json', {
      paths: [dirname(require.resolve('@heroui/react/package.json'))],
    }),
  ),
  'dist/**/*.{js,ts,jsx,tsx}',
)
  // fast-glob (used by Tailwind's content scanner) requires forward slashes;
  // path.join yields backslashes on Windows, which glob treats as escapes.
  .replace(/\\/g, '/');

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
    herouiThemeGlob,
    // Fallback for the hoisted layout, in case resolution above ever misses.
    './node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {},
  },
  darkMode: 'class',
  plugins: [heroui()],
};
