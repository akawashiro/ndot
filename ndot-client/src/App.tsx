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
} from '@mui/material';

// Import Roboto font
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

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
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
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

async function initWasm() {
  await init();
}

function App() {
  const defaultDotExample = `digraph graphname {
    a -> b -> c;
    b -> d;
}`;

  const [isLoading, setIsLoading] = useState(true);
  const [text, setText] = useState<string>(defaultDotExample);
  const [svgOutput, setSvgOutput] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleTextChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setText(e.target.value);
    try {
      const svg = make_svg_from_dot(e.target.value);
      if (svg.svg) {
        console.log('Get svg successfully.');
        setSvgOutput(svg.svg);
        setError(null);
      } else if (svg.error) {
        console.log('Get error of ', svg.error);
        setSvgOutput(null);
        setError(svg.error);
      }
    } catch (err) {
      setError(`Error: ${err instanceof Error ? err.message : String(err)}`);
      setSvgOutput(null);
    }
  };

  useEffect(() => {
    initWasm().then(() => {
      setIsLoading(false);
    });
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
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      {isLoading ? (
        <Box
          sx={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            height: '100vh',
          }}
        >
          <CircularProgress />
        </Box>
      ) : (
        <Container fixed sx={{ p: 0, height: '100vh' }}>
          <Grid2 container spacing={0} sx={{ height: '100%' }}>
            <Grid2 size={6} sx={{ height: '100%' }}>
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
                  onChange={handleTextChange}
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
            </Grid2>

            <Grid2 size={6} sx={{ height: '100%' }}>
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
            </Grid2>
          </Grid2>
        </Container>
      )}
    </ThemeProvider>
  );
}

export default App;
