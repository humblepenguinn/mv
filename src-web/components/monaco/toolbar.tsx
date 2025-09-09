import React from 'react';

import {
  Type,
  Minus,
  Plus,
  Lock,
  Unlock,
  RotateCcw,
  ChevronDown,
  Code,
  Loader2,
  AlertCircle,
  Search,
} from 'lucide-react';

import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useEditorSettingsStore } from '@/stores/editor';
import { useLanguage } from '@/hooks/useLanguage';

interface EditorToolbarProps {
  className?: string;
  fontsLoading?: boolean;
  fontsError?: string | null;
}

export function EditorToolbar({
  className = '',
  fontsLoading = false,
  fontsError = null,
}: EditorToolbarProps) {
  const [fontSearchQuery, setFontSearchQuery] = React.useState('');

  const {
    settings,
    setFontFamily,
    setFontSize,
    toggleHeapLockIcon,
    availableFonts,
    resetToDefaults,
    fetchSystemFonts,
  } = useEditorSettingsStore();

  const {
    currentLanguage,
    switchLanguage,
    supportedLanguages,
    currentLanguageDisplayName,
    getLanguageConfig,
  } = useLanguage();

  const handleFontSizeChange = (delta: number) => {
    setFontSize(settings.fontSize + delta);
  };

  const filteredFonts = React.useMemo(() => {
    if (!fontSearchQuery.trim()) return availableFonts;
    return availableFonts.filter((font) =>
      font.toLowerCase().includes(fontSearchQuery.toLowerCase())
    );
  }, [availableFonts, fontSearchQuery]);

  return (
    <div
      className={`flex items-center justify-between px-3 py-2 bg-muted/30 flex-shrink-0 ${className}`}
    >
      <div className="flex items-center gap-1">
        <h2 className="text-xs sm:text-sm font-semibold text-foreground mr-4">
          Code Editor
        </h2>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-7 px-2 text-xs gap-1"
              disabled={fontsLoading}
            >
              {fontsLoading ? (
                <Loader2 className="w-3.5 h-3.5 animate-spin" />
              ) : fontsError ? (
                <AlertCircle className="w-3.5 h-3.5 text-destructive" />
              ) : (
                <Type className="w-3.5 h-3.5" />
              )}
              <span
                className="max-w-24 truncate"
                style={{ fontFamily: settings.fontFamily }}
              >
                {fontsLoading
                  ? 'Loading...'
                  : fontsError
                    ? 'Error'
                    : settings.fontFamily}
              </span>
              <ChevronDown className="w-3 h-3" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            align="start"
            className="w-64 max-h-80 overflow-hidden"
          >
            {fontsError ? (
              <div className="px-2 py-1.5 text-xs text-destructive">
                <div className="flex items-center gap-2">
                  <AlertCircle className="w-3 h-3" />
                  <span>Failed to load fonts</span>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={fetchSystemFonts}
                  className="h-6 px-2 mt-2 text-xs"
                >
                  Retry
                </Button>
              </div>
            ) : fontsLoading ? (
              <div className="px-2 py-1.5 text-xs text-muted-foreground">
                <div className="flex items-center gap-2">
                  <Loader2 className="w-3 h-3 animate-spin" />
                  <span>Loading fonts...</span>
                </div>
              </div>
            ) : (
              <>
                <div className="p-2 border-b">
                  <div className="relative">
                    <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 w-3 h-3 text-muted-foreground" />
                    <Input
                      placeholder="Search fonts..."
                      value={fontSearchQuery}
                      onChange={(e) => setFontSearchQuery(e.target.value)}
                      className="pl-7 h-7 text-xs"
                    />
                  </div>
                </div>
                <div className="max-h-48 overflow-y-auto">
                  {filteredFonts.length === 0 ? (
                    <div className="px-2 py-4 text-xs text-muted-foreground text-center">
                      No fonts found
                    </div>
                  ) : (
                    filteredFonts.map((font) => (
                      <DropdownMenuItem
                        key={font}
                        onClick={() => setFontFamily(font)}
                        className={`flex flex-col items-start gap-1 py-2 ${
                          settings.fontFamily === font ? 'bg-accent' : ''
                        }`}
                      >
                        <div
                          className="font-medium text-sm"
                          style={{ fontFamily: font }}
                        >
                          {font}
                        </div>
                        <div
                          className="text-xs text-muted-foreground"
                          style={{ fontFamily: font }}
                        >
                          Aa Bb Cc 123
                        </div>
                      </DropdownMenuItem>
                    ))
                  )}
                </div>
              </>
            )}
          </DropdownMenuContent>
        </DropdownMenu>

        <div className="w-px h-4 bg-border mx-2" />

        <div className="flex items-center gap-1">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => handleFontSizeChange(-1)}
            className="h-7 w-7 p-0"
            title="Decrease font size (Ctrl+-)"
          >
            <Minus className="w-3.5 h-3.5" />
          </Button>
          <div className="flex items-center gap-2 px-2">
            <input
              type="range"
              min="8"
              max="32"
              value={settings.fontSize}
              onChange={(e) => setFontSize(parseInt(e.target.value))}
              className="w-16 h-1 bg-muted-foreground/20 rounded-lg appearance-none cursor-pointer slider-thumb:bg-primary slider-thumb:border-0 slider-thumb:w-3 slider-thumb:h-3 slider-thumb:rounded-full slider-thumb:appearance-none slider-thumb:cursor-pointer slider-track:bg-muted-foreground/20 slider-track:rounded-lg slider-track:h-1"
              title="Font size slider"
            />
            <span className="text-xs text-muted-foreground min-w-8 text-center">
              {settings.fontSize}px
            </span>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => handleFontSizeChange(1)}
            className="h-7 w-7 p-0"
            title="Increase font size (Ctrl++)"
          >
            <Plus className="w-3.5 h-3.5" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={resetToDefaults}
            className="h-7 w-7 p-0"
            title="Reset to default font settings"
          >
            <RotateCcw className="w-3.5 h-3.5" />
          </Button>
        </div>

        <div className="w-px h-4 bg-border mx-2" />

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              size="sm"
              className="h-7 px-2 text-xs gap-1"
            >
              <Code className="w-3.5 h-3.5" />
              <span className="max-w-20 truncate">
                {currentLanguageDisplayName}
              </span>
              <ChevronDown className="w-3 h-3" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-32">
            {supportedLanguages.map((language) => {
              const config = getLanguageConfig(language);
              return (
                <DropdownMenuItem
                  key={language}
                  onClick={() => switchLanguage(language)}
                  className={currentLanguage === language ? 'bg-accent' : ''}
                >
                  {config.displayName}
                </DropdownMenuItem>
              );
            })}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="sm"
          onClick={toggleHeapLockIcon}
          className="h-7 px-2 text-xs gap-1"
          title={
            settings.showHeapLockIcon
              ? 'Hide heap lock icons'
              : 'Show heap lock icons'
          }
        >
          {settings.showHeapLockIcon ? (
            <Lock className="w-3.5 h-3.5 text-red-500" />
          ) : (
            <Unlock className="w-3.5 h-3.5 text-green-500" />
          )}
          <span className="hidden sm:inline">
            {settings.showHeapLockIcon
              ? 'Heap Lock Icons Visible'
              : 'Heap Lock Icons Hidden'}
          </span>
        </Button>

        <div className="w-px h-4 bg-border mx-2" />
      </div>
    </div>
  );
}
