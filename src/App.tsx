import { useState, useEffect } from 'react'

function App() {
  const [temp, setTemp] = useState(0)

  useEffect(() => {
    // Placeholder - will be replaced with Tauri commands
    const interval = setInterval(() => {
      setTemp(Math.floor(Math.random() * 100))
    }, 2000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="app">
      <h1>RocmTop</h1>
      <div className="metric">
        <span>Temperature: {temp}°C</span>
      </div>
    </div>
  )
}

export default App
