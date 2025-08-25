export default {
    content: [
        "./index.html",
        "./src/**/*.rs",
    ],
    plugins: [
        function ({ matchUtilities, theme }) {
            const zinc = theme('colors.zinc') || {};
            const values = Object.fromEntries(
                Object.entries(zinc).filter(([, v]) => typeof v === 'string')
            );
            matchUtilities(
                {
                    '_bg': (v) => ({ backgroundColor: v }),
                    '_text': (v) => ({ color: v }),
                    '_border': (v) => ({ borderColor: v }),
                },
                { values }
            );
        },
    ],
}