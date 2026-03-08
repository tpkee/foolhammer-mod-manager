import tailwindcss from '@tailwindcss/vite'
import IconsResolver from 'unplugin-icons/resolver'
import ViteComponents from 'unplugin-vue-components/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2026-02-05',
  modules: ['@nuxt/a11y', '@nuxt/eslint', '@nuxt/hints', 'unplugin-icons/nuxt', '@nuxtjs/i18n', '@pinia/nuxt', '@vueuse/nuxt'],
  srcDir: 'src/',
  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: {
    host: '0',
  },
  app: {
    head: {
      meta: [
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      ],
      htmlAttrs: {
        lang: 'en',
      },
      title: 'Foolhammer Mod Manager',
    },
  },
  eslint: {
    config: {
      standalone: false,
    },
  },
  imports: {
    dirs: ['~/composables/**', '~/stores/**'],
  },
  components: [
    {
      path: '~/components',
      pathPrefix: false,
    },
  ],
  pinia: {
    storesDirs: ['~/src/stores/**'],
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
      // @ts-expect-error - Dunno why it doesn't recognize the plugin, but it works fine
      tailwindcss(),
      // @ts-expect-error - Dunno why it doesn't recognize the plugin, but it works fine
      ViteComponents({
        resolvers: [
          IconsResolver({
            prefix: 'icon',
            strict: true,
          }),
        ],
        dts: true,
      }),
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
  // Avoids error [unhandledRejection] EMFILE: too many open files, watch
  ignore: ['**/src-tauri/**'],
  devtools: { enabled: true },
})
