import { createContext, useContext } from 'react';

const ThemeContext = createContext({
    theme: 'dark', // Default theme
    toggleTheme: () => { },
});

export const useTheme = () => useContext(ThemeContext);

export default ThemeContext;
