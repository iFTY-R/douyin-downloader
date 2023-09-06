import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import ElementPlus from 'unplugin-element-plus/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    vue(), ElementPlus(),
    VueI18nPlugin({
      include: resolve(__dirname, 'src/i18n/locales/**'),
      runtimeOnly: false,
    })
  ],
  server: {
    port: 8000,
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
})
