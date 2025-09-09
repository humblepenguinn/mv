import './global.css';
import '@xyflow/react/dist/base.css';

import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from '@tanstack/react-router';

import { router } from './lib/router';
import { QueryProvider } from './providers/query-provider';
import { ThemeProvider } from './providers/theme-provider';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryProvider>
      <ThemeProvider>
        <RouterProvider router={router} />
      </ThemeProvider>
    </QueryProvider>
  </React.StrictMode>
);
