import '../styles/global.css'
import 'prismjs/themes/prism-tomorrow.css'; // Import Prism theme

import posthog from 'posthog-js'
import { PostHogProvider } from 'posthog-js/react'
import ThemeContext from '../contexts/ThemeContext';
import { useState, useEffect } from 'react';

// Check that PostHog is client-side (used to handle Next.js SSR)
if (typeof window !== 'undefined') {
  posthog.init('phc_9XPlyPALuefIMAMSfsvBk4jVuSelJyjl7xwhXigkHAP', {
    api_host: 'https://us.i.posthog.com',
    person_profiles: 'identified_only',
    // Enable debug mode in development
    loaded: (posthog) => {
      if (process.env.NODE_ENV === 'development') posthog.debug()
    }
  })
}

const getInitialTheme = () => {
  if (typeof window === 'undefined') {
    return 'dark'; // Default for SSR, actual class application happens client-side
  }
  const storedTheme = document.cookie
    .split('; ')
    .find(row => row.startsWith('theme='))
    ?.split('=')[1];

  if (storedTheme) {
    return storedTheme;
  }

  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches) {
    return 'light';
  }
  return 'dark'; // Default if no cookie and no system preference for light
};

export default function App(
  { Component, pageProps: { session, ...pageProps } }
) {
  const [theme, setTheme] = useState(getInitialTheme);

  // Effect to apply theme class to body and save to cookie whenever theme changes
  useEffect(() => {
    // This effect runs on the client after the initial render and whenever theme changes.
    // The 'theme' state is already correctly initialized by getInitialTheme on the client.
    if (theme === 'light') {
      document.body.classList.add('light-theme');
    } else {
      document.body.classList.remove('light-theme');
    }
    // Save theme to cookie for 1 year
    document.cookie = `theme=${theme}; path=/; max-age=31536000; SameSite=Lax`;
  }, [theme]);

  const toggleTheme = () => {
    setTheme(prevTheme => (prevTheme === 'dark' ? 'light' : 'dark'));
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      <PostHogProvider client={posthog}>
        <Component {...pageProps} />
      </PostHogProvider>
    </ThemeContext.Provider>
  )
}
