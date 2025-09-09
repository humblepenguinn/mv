import React from 'react';

import { Editor as MonacoEditor } from '@monaco-editor/react';
import * as monaco from 'monaco-editor';

import { useTheme } from '@/providers/theme-provider';
import { EditorToolbar } from '@/components/monaco/toolbar';
import { useEditorSettingsStore } from '@/stores/editor';
import { useLanguage } from '@/hooks/useLanguage';

type EditorProps = {
  onRun?: (code: string) => void;
  onChange?: (code: string) => void;
  code: string;
  analyzeError: {
    message: string;
    line_number?: number;
    column_number?: number;
  } | null;
};

export type EditorRef = {
  run: () => void;
  getCode: () => string;
};

export const Editor = React.forwardRef<EditorRef, EditorProps>((props, ref) => {
  const { onRun, onChange, code, analyzeError } = props;
  const editorRef = React.useRef<monaco.editor.IStandaloneCodeEditor | null>(
    null
  );
  const monacoRef = React.useRef<typeof monaco | null>(null);
  const { theme } = useTheme();
  const { settings, fontsError, fontsLoading, setFontSize } =
    useEditorSettingsStore();
  const { currentLanguage, currentLanguageInitialCode } = useLanguage();

  const getCurrentEditorTheme = React.useCallback(() => {
    if (theme === 'dark') return 'vs-dark';
    if (theme === 'light') return 'vs-light';

    return window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'vs-dark'
      : 'vs-light';
  }, [theme]);

  const handleRun = React.useCallback(() => {
    const editor = editorRef.current;
    if (!editor) return;

    const code = editor.getValue();
    onRun?.(code);
  }, [onRun]);

  const getCode = React.useCallback(() => {
    const editor = editorRef.current;
    if (!editor) return '';
    return editor.getValue();
  }, []);

  React.useImperativeHandle(
    ref,
    () => ({
      run: handleRun,
      getCode,
    }),
    [handleRun, getCode]
  );

  const handleOnMount = React.useCallback(
    async (
      editor: monaco.editor.IStandaloneCodeEditor,
      monacoInstance: typeof monaco
    ) => {
      editorRef.current = editor;
      monacoRef.current = monacoInstance;

      const currentTheme = getCurrentEditorTheme();
      monacoInstance.editor.setTheme(currentTheme);

      editor.focus();

      editor.addCommand(
        monacoInstance.KeyMod.CtrlCmd | monacoInstance.KeyCode.Enter,
        () => {
          handleRun();
        }
      );

      editor.addCommand(
        monacoInstance.KeyMod.CtrlCmd | monacoInstance.KeyCode.Equal,
        () => {
          setFontSize((currentSize) => currentSize + 1);
        }
      );

      editor.addCommand(
        monacoInstance.KeyMod.CtrlCmd | monacoInstance.KeyCode.Minus,
        () => {
          setFontSize((currentSize) => currentSize - 1);
        }
      );
    },
    [getCurrentEditorTheme, handleRun, setFontSize]
  );

  const handleOnChange = React.useCallback(
    (value: string | undefined) => {
      if (value !== undefined && onChange) {
        onChange(value);
      }
    },
    [onChange]
  );

  React.useEffect(() => {
    const editor = editorRef.current;
    const monaco = monacoRef.current;
    if (!editor || !monaco) return;

    const currentTheme = getCurrentEditorTheme();
    monaco.editor.setTheme(currentTheme);
  }, [theme, getCurrentEditorTheme]);

  // handle parser errors by setting markers
  React.useEffect(() => {
    const editor = editorRef.current;
    const monaco = monacoRef.current;

    if (!editor || !monaco || !analyzeError) {
      if (editor && monaco) {
        const model = editor.getModel();
        if (model) {
          monaco.editor.setModelMarkers(model, 'error', []);
        }
      }
      return;
    }

    const markers = [];

    if (analyzeError.line_number && analyzeError.column_number) {
      markers.push({
        startLineNumber: analyzeError.line_number,
        endLineNumber: analyzeError.line_number,
        startColumn: analyzeError.column_number,
        endColumn: analyzeError.column_number,
        message: analyzeError.message,
        severity: monaco.MarkerSeverity.Error,
      });
    }

    const model = editor.getModel();
    if (model) {
      monaco.editor.setModelMarkers(model, 'error', markers);
    }
  }, [analyzeError]);

  return (
    <div className="h-full w-full overflow-hidden flex flex-col bg-background">
      <EditorToolbar fontsLoading={fontsLoading} fontsError={fontsError} />

      <div className="flex-1 min-h-0 overflow-hidden bg-background">
        <MonacoEditor
          height="100%"
          language={currentLanguage}
          theme={getCurrentEditorTheme()}
          defaultValue={currentLanguageInitialCode}
          value={code}
          onChange={handleOnChange}
          onMount={handleOnMount}
          options={{
            fontFamily: settings.fontFamily,
            fontSize: settings.fontSize,
            lineNumbers: 'on' as const,
            folding: true,
            selectOnLineNumbers: true,
            renderLineHighlight: 'line' as const,
            bracketPairColorization: { enabled: true },
            minimap: { enabled: false },
            scrollbar: {
              useShadows: true,
              verticalHasArrows: true,
              horizontalHasArrows: true,
              vertical: 'visible' as const,
              horizontal: 'visible' as const,
              verticalScrollbarSize: 17,
              horizontalScrollbarSize: 17,
              arrowSize: 30,
            },
            automaticLayout: true,
            tabSize: 2,
            insertSpaces: true,
            wordWrap: 'off' as const,
            readOnly: false,
            contextmenu: true,
            mouseWheelZoom: false,
            smoothScrolling: true,
            cursorBlinking: 'blink' as const,
            cursorSmoothCaretAnimation: 'off' as const,
            renderWhitespace: 'selection' as const,
            renderControlCharacters: false,
            fontLigatures: true,
            formatOnPaste: true,
            formatOnType: false,
            suggestOnTriggerCharacters: true,
            acceptSuggestionOnEnter: 'on' as const,
            quickSuggestions: true,
            parameterHints: { enabled: true },
            hover: { enabled: true },
          }}
          loading={
            <div className="flex items-center justify-center h-full bg-background text-foreground">
              Loading editor...
            </div>
          }
        />
      </div>
    </div>
  );
});

Editor.displayName = 'Editor';
