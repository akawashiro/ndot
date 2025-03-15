import { useState, useEffect } from 'react';
import init, { make_svg_from_dot } from 'ndot-wasm';
import { v7 as uuidv7 } from 'uuid';
import { Routes, Route, useParams, useNavigate } from 'react-router-dom';
import {
  Container,
  Grid2,
  Paper,
  TextField,
  CircularProgress,
  Alert,
  ThemeProvider,
  createTheme,
  CssBaseline,
  Box,
  GlobalStyles,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  SelectChangeEvent,
  Button,
} from '@mui/material';
import { samples, sampleNames, SampleKey } from './samples';
import React from 'react';

// Global styles to ensure full height
const globalStyles = (
  <GlobalStyles
    styles={{
      'html, body, #root': {
        height: '100%',
        margin: 0,
        padding: 0,
        overflow: 'hidden',
      },
    }}
  />
);

// Create dark theme
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    background: {
      default: '#121212',
      paper: '#1e1e1e',
    },
    primary: {
      main: '#90caf9',
    },
  },
  typography: {
    fontFamily: '"Helvetica", "Arial", sans-serif',
  },
  components: {
    MuiTextField: {
      defaultProps: {
        variant: 'outlined',
        fullWidth: true,
      },
      styleOverrides: {
        root: {
          '& .MuiOutlinedInput-root': {
            fontFamily: '"Courier New", monospace',
            fontSize: '14px',
          },
        },
      },
    },
  },
});

let wasmInitialized = false;

async function initWasm() {
  await init();
  wasmInitialized = true;
}

interface EditorProps {
  text: string;
  onTextChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

const Editor: React.FunctionComponent<EditorProps> = ({
  text,
  onTextChange,
}) => {
  return (
    <Paper
      elevation={3}
      sx={{
        p: 2,
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'background.paper',
        borderRadius: 0,
      }}
    >
      <TextField
        multiline
        value={text}
        onChange={onTextChange}
        sx={{
          flex: 1,
          '& .MuiInputBase-root': {
            height: '100%',
            bgcolor: '#2d2d2d',
            fontFamily: '"Courier New", monospace',
            fontSize: '14px',
            lineHeight: 1.5,
          },
          '& .MuiInputBase-input': {
            height: '100% !important',
            overflow: 'auto !important',
          },
        }}
      />
    </Paper>
  );
};

interface PreviewProps {
  svgOutput: string | null;
  error: string | null;
}

const Preview: React.FunctionComponent<PreviewProps> = ({
  svgOutput,
  error,
}) => {
  return (
    <Paper
      elevation={3}
      sx={{
        p: 2,
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'background.paper',
        borderRadius: 0,
      }}
    >
      <Paper
        elevation={0}
        sx={{
          flex: 1,
          p: 2,
          bgcolor: '#2d2d2d',
          overflow: 'auto',
          '& svg': {
            maxWidth: '100%',
            height: 'auto',
          },
        }}
      >
        {error ? (
          <Alert
            severity="error"
            sx={{
              bgcolor: 'transparent',
              color: '#ff5555',
              fontFamily: '"Courier New", monospace',
              fontSize: '14px',
              whiteSpace: 'pre-wrap',
              textAlign: 'left',
              '& .MuiAlert-icon': {
                color: '#ff5555',
              },
            }}
          >
            {error}
          </Alert>
        ) : (
          <Box
            sx={{ height: '100%' }}
            dangerouslySetInnerHTML={{ __html: svgOutput || '' }}
          />
        )}
      </Paper>
    </Paper>
  );
};

// EditorPage component that contains the editor functionality
function EditorPage() {
  const params = useParams();
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(true);
  const [isContentLoading, setIsContentLoading] = useState(false);
  const [text, setText] = useState<string>(samples.digraph);
  const [svgOutput, setSvgOutput] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedSample, setSelectedSample] = useState<SampleKey>('digraph');
  const [fileId, setFileId] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState<{
    type: 'success' | 'error';
    text: string;
  } | null>(null);

  const handleTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Only update the text state, the useEffect hook will handle the SVG generation
    setText(e.target.value);
  };

  const handleSave = async () => {
    setIsSaving(true);
    setSaveMessage(null);

    try {
      // Generate a new UUID v7 if we don't have one
      const id = fileId || uuidv7();

      // Make the API request
      const response = await fetch(`${import.meta.env.VITE_API_URL}/api/save`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id,
          content: text,
        }),
      });

      const data = await response.json();

      if (data.success) {
        console.log('id=', id);
        // Save was successful
        setFileId(id); // Store the ID for future saves

        // Construct the full URL
        const fullUrl = `${window.location.origin}/ndot/${id}`;

        setSaveMessage({
          type: 'success',
          text: `File saved successfully! URL: ${fullUrl}`,
        });

        // Update the URL with the file ID
        navigate(`/ndot/${id}`);
      } else {
        // Save failed with an error from the server
        setSaveMessage({
          type: 'error',
          text: data.error || 'Failed to save file',
        });
      }
    } catch (err) {
      // Network or other error
      setSaveMessage({
        type: 'error',
        text: `Error: ${err instanceof Error ? err.message : String(err)}`,
      });
    } finally {
      setIsSaving(false);
    }
  };

  const handleSampleChange = (event: SelectChangeEvent) => {
    const sampleKey = event.target.value as SampleKey;
    setSelectedSample(sampleKey);
    setText(samples[sampleKey]);
  };

  // Fetch content when ID parameter is present
  useEffect(() => {
    const id = params.id;
    if (id && wasmInitialized) {
      setIsContentLoading(true);
      setFileId(id);

      fetch(`${import.meta.env.VITE_API_URL}/api/get/${id}`)
        .then(response => response.json())
        .then(data => {
          if (data.success && data.content) {
            setText(data.content);
          } else {
            setError(`Error: ${data.error || 'Failed to load content'}`);
          }
        })
        .catch(err => {
          setError(
            `Error: ${err instanceof Error ? err.message : String(err)}`
          );
        })
        .finally(() => {
          setIsContentLoading(false);
        });
    }
  }, [params.id, wasmInitialized]);

  // Initialize WebAssembly module
  useEffect(() => {
    initWasm()
      .then(() => {
        setIsLoading(false);
        // Only try to generate SVG after WebAssembly is initialized
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
          setError(
            `Error: ${err instanceof Error ? err.message : String(err)}`
          );
          setSvgOutput(null);
        }
      })
      .catch(err => {
        console.error('Failed to initialize WebAssembly module:', err);
        setError(
          `Failed to initialize WebAssembly module: ${err instanceof Error ? err.message : String(err)}`
        );
        setIsLoading(false);
      });
  }, []);

  // Update the SVG when text changes, but only if WebAssembly is initialized
  useEffect(() => {
    if (!wasmInitialized) return;

    try {
      console.log('Generating SVG from text:', text);
      const result = make_svg_from_dot(text);
      if (result.svg) {
        console.log('SVG generated successfully');
        setSvgOutput(result.svg);
        setError(null);
      } else if (result.error) {
        console.log('Error generating SVG:', result.error);
        setError(result.error);
        setSvgOutput(null);
      }
    } catch (err) {
      console.error('Exception while generating SVG:', err);
      setError(`Error: ${err instanceof Error ? err.message : String(err)}`);
      setSvgOutput(null);
    }
  }, [text]);

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      {globalStyles}
      <Box sx={{ height: '100vh', display: 'flex', flexDirection: 'column' }}>
        {isLoading || isContentLoading ? (
          <Box
            sx={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              flex: 1,
            }}
          >
            <CircularProgress />
          </Box>
        ) : (
          <Container
            maxWidth={false}
            sx={{
              height: '100%',
              p: 0,
              display: 'flex',
              flexDirection: 'column',
              overflow: 'hidden',
            }}
          >
            <Grid2
              container
              spacing={0}
              sx={{
                flex: 1,
                height: '100%',
                overflow: 'hidden',
              }}
            >
              <Grid2
                size={6}
                sx={{
                  height: '100%',
                  display: 'flex',
                  flexDirection: 'column',
                }}
              >
                <Paper
                  elevation={3}
                  sx={{
                    p: 2,
                    bgcolor: 'background.paper',
                    borderRadius: 0,
                    borderBottom: '1px solid rgba(255, 255, 255, 0.12)',
                  }}
                >
                  <FormControl fullWidth size="small">
                    <InputLabel id="sample-select-label">Sample</InputLabel>
                    <Select
                      labelId="sample-select-label"
                      id="sample-select"
                      value={selectedSample}
                      label="Sample"
                      onChange={handleSampleChange}
                    >
                      {Object.entries(sampleNames).map(([key, name]) => (
                        <MenuItem key={key} value={key}>
                          {name}
                        </MenuItem>
                      ))}
                    </Select>
                  </FormControl>
                </Paper>
                <Paper
                  elevation={3}
                  sx={{
                    p: 2,
                    bgcolor: 'background.paper',
                    borderRadius: 0,
                    borderBottom: '1px solid rgba(255, 255, 255, 0.12)',
                  }}
                >
                  <Button
                    fullWidth
                    variant="contained"
                    onClick={handleSave}
                    disabled={isSaving}
                  >
                    {isSaving ? 'Saving...' : 'Save'}
                  </Button>
                  {saveMessage && (
                    <Alert
                      severity={saveMessage.type}
                      sx={{ mt: 1 }}
                      onClose={() => setSaveMessage(null)}
                    >
                      {saveMessage.type === 'success' &&
                      saveMessage.text.includes('URL:') ? (
                        <>
                          File saved successfully!
                          <br />
                          <Box
                            component="a"
                            href={saveMessage.text.split('URL: ')[1]}
                            target="_blank"
                            rel="noopener"
                            sx={{
                              fontWeight: 'bold',
                              wordBreak: 'break-all',
                            }}
                          >
                            {saveMessage.text.split('URL: ')[1]}
                          </Box>
                        </>
                      ) : (
                        saveMessage.text
                      )}
                    </Alert>
                  )}
                </Paper>
                <Box sx={{ flex: 1, overflow: 'hidden' }}>
                  <Editor text={text} onTextChange={handleTextChange} />
                </Box>
              </Grid2>
              <Grid2 size={6} sx={{ height: '100%' }}>
                <Preview svgOutput={svgOutput} error={error} />
              </Grid2>
            </Grid2>
          </Container>
        )}
      </Box>
    </ThemeProvider>
  );
}

// Main App component with routes
function App() {
  return (
    <Routes>
      <Route path="/ndot/" element={<EditorPage />} />
      <Route path="/ndot/:id" element={<EditorPage />} />
    </Routes>
  );
}

export default App;
