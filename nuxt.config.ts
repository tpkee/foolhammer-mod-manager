import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2026-02-05',
  modules: ['@nuxt/a11y', '@nuxt/eslint', '@nuxt/hints', '@nuxt/icon', '@nuxtjs/i18n', '@pinia/nuxt'],
  srcDir: 'src/',
  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: {
    host: '0',
  },
  eslint: {
    config: {
      standalone: false,
    },
  },
  imports: {
    dirs: ['~/composables/**'],
  },
  pinia: {
    storesDirs: ['~/stores/**'],
  },
  ssr: false,
  css: ['./src/assets/css/main.css'],
  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
    },
    plugins: [
      tailwindcss(),
    ],
  },
  typescript: {
    typeCheck: true,
  },
  i18n: {
    defaultLocale: 'en',
    locales: [
      { code: 'en', name: 'English', file: 'en.json' },
    ],
  },
  icon: {
    componentName: 'NuxtIcon',
    mode: 'css',
    cssLayer: 'base',
  },
  // Avoids error [unhandledRejection] EMFILE: too many open files, watch
  ignore: ['**/src-tauri/**'],
  devtools: { enabled: true },
})
