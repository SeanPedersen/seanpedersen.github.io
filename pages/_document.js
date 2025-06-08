import Document, { Html, Head, Main, NextScript } from 'next/document'

export default class MyDocument extends Document {
  render() {
    return (
      <Html lang="en">
        <Head />
        <body>
          <script
            dangerouslySetInnerHTML={{
              __html: `
                (function() {
                  try {
                    var storedTheme = document.cookie
                      .split('; ')
                      .find(function(row) { return row.startsWith('theme='); });
                    
                    var theme = storedTheme ? storedTheme.split('=')[1] : 
                      (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');
                    
                    if (theme === 'light') {
                      document.body.classList.add('light-theme');
                    }
                  } catch (e) {}
                })();
              `
            }}
          />
          <Main />
          <NextScript />
        </body>
      </Html>
    )
  }
}
