import { useState } from 'react'
// import reactLogo from './assets/react.svg'
import './App.css'

function App() {
  const [file, setFile] = useState<File | null>(null)
  const [samples, setSamples] = useState(10)
  const [tolerance, setTolerance] = useState(0.1)
  const [feedrate, setFeedrate] = useState(1000)
  const [penDown, setPenDown] = useState('M300 S50')
  const [penUp, setPenUp] = useState('M300 S30')
  const [pattern, setPattern] = useState('hatch:1.0:45')
  const [gcode, setGcode] = useState('')
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!file) return

    setLoading(true)
    const formData = new FormData()
    formData.append('file', file)
    formData.append('samples', samples.toString())
    formData.append('tolerance', tolerance.toString())
    formData.append('feedrate', feedrate.toString())
    formData.append('pen_down', penDown)
    formData.append('pen_up', penUp)
    formData.append('pattern', pattern)

    try {
      const response = await fetch('http://localhost:3000/api/convert', {
        method: 'POST',
        body: formData,
      })
      const text = await response.text()
      setGcode(text)
    } catch (error) {
      console.error('Error:', error)
      setGcode('Error generating G-code')
    }
    setLoading(false)
  }

  const downloadGcode = () => {
    const blob = new Blob([gcode], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'plotter.gcode'
    a.click()
    URL.revokeObjectURL(url)
  }

  return (
    <div className="App">
      <h1>PenPlotter</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label>SVG File:</label>
          <input type="file" accept=".svg" onChange={(e) => setFile(e.target.files?.[0] || null)} required />
        </div>
        <div>
          <label>Samples:</label>
          <input type="number" value={samples} onChange={(e) => setSamples(Number(e.target.value))} min="1" />
        </div>
        <div>
          <label>Tolerance:</label>
          <input type="number" step="0.01" value={tolerance} onChange={(e) => setTolerance(Number(e.target.value))} min="0" />
        </div>
        <div>
          <label>Feedrate:</label>
          <input type="number" value={feedrate} onChange={(e) => setFeedrate(Number(e.target.value))} min="1" />
        </div>
        <div>
          <label>Pen Down:</label>
          <textarea value={penDown} onChange={(e) => setPenDown(e.target.value)} rows={2} placeholder="M300 S50&#10;G4 P100" />
        </div>
        <div>
          <label>Pen Up:</label>
          <textarea value={penUp} onChange={(e) => setPenUp(e.target.value)} rows={2} placeholder="M300 S30" />
        </div>
        <div>
          <label>Pattern:</label>
          <input type="text" value={pattern} onChange={(e) => setPattern(e.target.value)} placeholder="hatch:1.0:45" />
        </div>
        <button type="submit" disabled={loading || !file}>
          {loading ? 'Generating...' : 'Convert to G-code'}
        </button>
      </form>
      {gcode && (
        <div>
          <h2>Generated G-code:</h2>
          <textarea readOnly value={gcode} rows={20} cols={80} />
          <button onClick={downloadGcode}>Download G-code</button>
        </div>
      )}
    </div>
  )
}

export default App