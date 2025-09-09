import React from 'react';

export type SupportedLanguage = 'cpp';

const LANGUAGE_CONFIG = {
  cpp: {
    displayName: 'C++',
    initialCode: `// MV currently supports a small subset of C++, 
// particularly related to pointers and variable 
// declarations. Here are the supported features:
  
// - Variable declarations:
//   e.g, int x = 12;
//        int p = x; (p will be assigned the value of x)
  
// - Pointer declarations:
//   e.g., int* p = nullptr;
  
// - Pointer references to variables:
//   e.g., p = &x;

// - Heap pointer declarations:
//   e.g., p = new int;

// - Setting a pointer to nullptr:
//   e.g., p = nullptr;

// - Dereferencing a pointer:
//   e.g., *p = 8;

// - Deleting a pointer:
//   e.g., delete p;

// Each of these actions will have visualizations 
// to help you understand memory management.

// Try this out, for starters:
// int x = 12;
// int* p = &x;
// *p = 8;
// p = new int;
// *p = 10;
// delete p;
// p = nullptr;

// Type it out line by line and see how the visualizations
// dynamically change as you type each line. 

// Experiment with other primitive datatypes  
// too! (int, double, char, bool) (e.g., double* p = new double; or double x = 3.14;)

// We are actively working on expanding the 
// current parser to support more advanced 
// features of the language.

// Begin typing your code below:

  `,
  },
} as const;

const SUPPORTED_LANGUAGES: SupportedLanguage[] = Object.keys(
  LANGUAGE_CONFIG
) as SupportedLanguage[];

export const useLanguage = () => {
  const [currentLanguage, setCurrentLanguage] =
    React.useState<SupportedLanguage>('cpp');

  const switchLanguage = React.useCallback((language: SupportedLanguage) => {
    setCurrentLanguage(language);
  }, []);

  const getLanguageConfig = React.useCallback((language: SupportedLanguage) => {
    return LANGUAGE_CONFIG[language];
  }, []);

  const getCurrentLanguageConfig = React.useCallback(() => {
    return LANGUAGE_CONFIG[currentLanguage];
  }, [currentLanguage]);

  return {
    currentLanguage,
    switchLanguage,
    supportedLanguages: SUPPORTED_LANGUAGES,
    getLanguageConfig,
    getCurrentLanguageConfig,
    currentLanguageDisplayName: LANGUAGE_CONFIG[currentLanguage].displayName,
    currentLanguageInitialCode: LANGUAGE_CONFIG[currentLanguage].initialCode,
  };
};
