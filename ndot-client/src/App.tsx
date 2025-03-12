import { useState, useEffect } from 'react';
import { make_svg_from_dot } from 'ndot-wasm';
import './App.css';

function App() {
  const defaultDotExample = `digraph graphname {
    a -> b -> c;
    b -> d;
}`;

  const [text, setText] = useState<string>(defaultDotExample);
  const [svgOutput, setSvgOutput] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    let svg = make_svg_from_dot(e.target.value);
    if (svg.svg) {
      console.log('Get svg successfully.');
      setSvgOutput(svg.svg);
      setError(null);
    } else if (svg.error) {
      console.log('Get error of ', svg.error);
      setSvgOutput(null);
      setError(svg.error);
    }
  };

  useEffect(() => {
    try {
      const result = make_svg_from_dot(text);
      if (result.svg) {
        setSvgOutput(result.svg);
        setError(null);
      } else if (result.error) {
        setError(result.error);
        setSvgOutput(null);
      }
    } catch (err) {
      setError(`Error: ${err instanceof Error ? err.message : String(err)}`);
      setSvgOutput(null);
    }
  }, [text]);

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
            {error ? (
              <div className="error-message">{error}</div>
            ) : (
              <div dangerouslySetInnerHTML={{ __html: svgOutput || '' }} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
