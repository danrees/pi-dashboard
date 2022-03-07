module.exports = {
  "stories": [
    "../src/**/*.stories.mdx",
    "../src/**/*.stories.@(js|jsx|ts|tsx|svelte)"
  ],
  "addons": [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-svelte-csf",
    {
      "name": "@storybook/addon-postcss",
      "options": {
        "cssLoaderOptions": {
          "importLoaders": 1,
        },
        "postcssLoaderOptions": {
          "implementation": require("postcss")
        }
      }
    }
  ],
  "framework": "@storybook/svelte",
  "svelteOptions": {
    "preprocess": require("../svelte.config.cjs").preprocess
  }
}