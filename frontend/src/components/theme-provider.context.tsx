import { createContext, useContext } from "react"
import type { ThemeProviderState } from "@/components/theme-provider.types"
import { initialThemeState } from "@/components/theme-provider.types"

export const ThemeProviderContext = createContext<ThemeProviderState>(initialThemeState)

export const useTheme = () => {
  const context = useContext(ThemeProviderContext)
  if (context === undefined) {
    throw new Error("useTheme must be used within a ThemeProvider")
  }
  return context
}

export default useTheme
