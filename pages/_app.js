import '../styles/global.css'
import ThemeContext from '../contexts/ThemeContext';
import { useState, useEffect } from 'react';
import dynamic from 'next/dynamic';

// Lazy load PostHog provider - only loads when needed
const PostHogProvider = dynamic(
  () => import('posthog-js/react').then(mod => mod.PostHogProvider),
  { ssr: false }
);

// Lazy initialize PostHog
let posthogInstance = null;
const getPostHog = async () => {
  if (typeof window === 'undefined') return null;

  if (!posthogInstance) {
    const posthog = (await import('posthog-js')).default;
    posthog.init('phc_9XPlyPALuefIMAMSfsvBk4jVuSelJyjl7xwhXigkHAP', {
      api_host: 'https://us.i.posthog.com',
      person_profiles: 'identified_only',
      loaded: (posthog) => {
        if (process.env.NODE_ENV === 'development') posthog.debug()
      }
    });
    posthogInstance = posthog;
  }
  return posthogInstance;
};

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
  const [posthog, setPosthog] = useState(null);

  useEffect(() => {
    if (theme === 'light') {
      document.body.classList.add('light-theme');
    } else {
      document.body.classList.remove('light-theme');
    }
    // Cookie is now set only on manual toggle
  }, [theme]);

  useEffect(() => {
    // Initialize PostHog after component mounts
    getPostHog().then(setPosthog);
  }, []);

  const toggleTheme = () => {
    setTheme(prevTheme => {
      const newTheme = prevTheme === 'dark' ? 'light' : 'dark';
      // Save theme to cookie only when user manually toggles
      document.cookie = `theme=${newTheme}; path=/; max-age=31536000; SameSite=Lax`;
      return newTheme;
    });
  };

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {posthog ? (
        <PostHogProvider client={posthog}>
          <Component {...pageProps} />
        </PostHogProvider>
      ) : (
        <Component {...pageProps} />
      )}
    </ThemeContext.Provider>
  )
}
