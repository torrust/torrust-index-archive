module.exports = {
    env: {
        es2021: true,
    },
    extends: [
        'eslint:recommended',
        'plugin:vue/vue-recommended',
        "prettier"
    ],
    rules: {
        // override/add rules settings here, such as:
        // 'vue/no-unused-vars': 'error'
    }
}
