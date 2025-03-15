import { useState, useEffect } from 'react';
import init, { make_svg_from_dot } from 'ndot-wasm';
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

function App() {
  const [isLoading, setIsLoading] = useState(true);
  const [text, setText] = useState<string>(samples.digraph);
  const [svgOutput, setSvgOutput] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [selectedSample, setSelectedSample] = useState<SampleKey>('digraph');

  const handleTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Only update the text state, the useEffect hook will handle the SVG generation
    setText(e.target.value);
  };

  const handleSampleChange = (event: SelectChangeEvent) => {
    const sampleKey = event.target.value as SampleKey;
    setSelectedSample(sampleKey);
    setText(samples[sampleKey]);
  };

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
        {isLoading ? (
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

export default App;
