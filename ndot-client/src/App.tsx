import { useState } from 'react'
import './App.css'

function App() {
  const [text, setText] = useState<string>('// Enter your code here...')

  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value)
  }

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>ndot Editor</h1>
      </header>

      <div className="editor-container">
        <div className="editor-pane">
          <h2>Editor</h2>
          <textarea
            className="text-editor"
            value={text}
            onChange={handleTextChange}
            spellCheck={false}
          />
        </div>

        <div className="preview-pane">
          <h2>Preview</h2>
          <div className="preview-content">
            {/* Dummy preview content for now */}
            <pre>{text}</pre>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
