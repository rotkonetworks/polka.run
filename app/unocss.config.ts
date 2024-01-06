import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";

export default defineConfig({
  rules: [],
  shortcuts: {},
  preflights: [
    {
      getCSS: ({ theme: _ }) => `
      `,
    },
  ],
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      customizations: {
        iconCustomizer(_collection, _icon, props) {
          props["stroke-width"] = '1';
        },
      },
    }),
  ],
});
