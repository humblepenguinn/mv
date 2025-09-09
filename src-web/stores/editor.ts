import { create } from 'zustand';

import { invokeCmd } from '@/lib/tauri';
import { appInfo } from '@/lib/appInfo';

export interface EditorSettings {
  fontFamily: string;
  fontSize: number;
  showHeapLockIcon: boolean;
}

const FALLBACK_FONTS = [
  'Arial',
  'Verdana',
  'Helvetica',
  'Times New Roman',
  'Courier New',
  'Georgia',
  'Tahoma',
  'Trebuchet MS',
  'Impact',
  'Comic Sans MS',
  'Lucida Sans Unicode',
  'Palatino Linotype',
  'Fira Code',
  'Consolas',
  'Segoe UI',
  'Gill Sans',
  'Monaco',
  'Droid Sans',
  'Roboto',
  'Calibri',
  'Cambria',
  'Franklin Gothic Medium',
  'Garamond',
];

const DEFAULT_SETTINGS: EditorSettings = {
  fontFamily: 'Courier New',
  fontSize: 14,
  showHeapLockIcon: true,
};

interface EditorStore {
  settings: EditorSettings;
  availableFonts: string[];
  fontsLoading: boolean;
  fontsError: string | null;

  updateSettings: (newSettings: Partial<EditorSettings>) => void;
  setFontFamily: (fontFamily: string) => void;
  setFontSize: (fontSize: number | ((currentSize: number) => number)) => void;
  toggleHeapLockIcon: () => void;
  resetToDefaults: () => void;
  fetchSystemFonts: () => Promise<void>;
  initializeStore: () => void;
}

export const useEditorSettingsStore = create<EditorStore>((set, get) => ({
  settings: DEFAULT_SETTINGS,
  availableFonts: FALLBACK_FONTS,
  fontsLoading: false,
  fontsError: null,

  updateSettings: (newSettings: Partial<EditorSettings>) => {
    set((state) => {
      const updated = { ...state.settings, ...newSettings };
      if (typeof window !== 'undefined') {
        localStorage.setItem('editor-settings', JSON.stringify(updated));
      }
      return { settings: updated };
    });
  },

  setFontFamily: (fontFamily: string) => {
    get().updateSettings({ fontFamily });
  },

  setFontSize: (fontSize: number | ((currentSize: number) => number)) => {
    const currentSize = get().settings.fontSize;
    const newSize =
      typeof fontSize === 'function' ? fontSize(currentSize) : fontSize;
    get().updateSettings({
      fontSize: Math.max(8, Math.min(32, newSize)),
    });
  },

  toggleHeapLockIcon: () => {
    const { settings } = get();
    get().updateSettings({ showHeapLockIcon: !settings.showHeapLockIcon });
  },

  resetToDefaults: () => {
    set({ settings: DEFAULT_SETTINGS });
    if (typeof window !== 'undefined') {
      localStorage.setItem('editor-settings', JSON.stringify(DEFAULT_SETTINGS));
    }
  },

  fetchSystemFonts: async () => {
    set({ fontsLoading: true, fontsError: null });

    if (!appInfo.isDesktop) {
      set({ availableFonts: FALLBACK_FONTS, fontsLoading: false });
      return;
    }

    try {
      const systemFonts = await invokeCmd<string[]>('cmd_get_system_fonts');
      set({ availableFonts: systemFonts });
    } catch (err) {
      set({
        fontsError:
          err instanceof Error ? err.message : 'Failed to fetch system fonts',
      });
    } finally {
      set({ fontsLoading: false });
    }
  },

  initializeStore: () => {
    if (typeof window !== 'undefined') {
      const stored = localStorage.getItem('editor-settings');
      if (stored) {
        try {
          const parsed = JSON.parse(stored);
          set({ settings: { ...DEFAULT_SETTINGS, ...parsed } });
        } catch {
          set({ settings: DEFAULT_SETTINGS });
        }
      }
    }

    get().fetchSystemFonts();
  },
}));
