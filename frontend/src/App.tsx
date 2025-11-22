import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { WagmiProvider } from 'wagmi'
import { config } from './wagmi'
import Dashboard from './components/Dashboard'
import { ThemeProvider } from "@/components/theme-provider"

const queryClient = new QueryClient()

function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
          <Dashboard />
        </ThemeProvider>
      </QueryClientProvider>
    </WagmiProvider>
  )
}

export default App
