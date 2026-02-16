import antfu from '@antfu/eslint-config'
// @ts-check
import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
  antfu({
    vue: {
      a11y: true,
    },
    formatters: {
      /**
       * Format CSS, LESS, SCSS files, also the `<style>` blocks in Vue
       * By default uses Prettier
       */
      css: true,
      /**
       * Format HTML files
       * By default uses Prettier
       */
      html: true,
    },
    ignores: ['**/src-tauri'],
    stylistic: true,
  }),
  {
    rules: {
      'no-console': 'off',
      'vue/block-order': ['error', {
        order: ['template', 'script', 'style'],
      }],
      'pnpm/yaml-enforce-settings': 'off',
    },
  },
)
