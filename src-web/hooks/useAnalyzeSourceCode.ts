import { useQuery } from '@tanstack/react-query';
import { invokeCmd } from '@/lib/tauri';
import { createSourceCodeKey } from '@/lib/source-code-compression';
import { appInfo } from '@/lib/appInfo';
export interface AnalyzeSourceCodeResponse {
  stack: any[];
  heap: any[];
  error?: {
    message: string;
    line_number?: number;
    column_number?: number;
  };
}

async function analyzeSourceDesktop(
  sourceCode: string
): Promise<AnalyzeSourceCodeResponse> {
  const response = await invokeCmd<AnalyzeSourceCodeResponse>(
    'cmd_analyze_source_code',
    { input: sourceCode }
  );
  return response;
}

async function analyzeSourceWeb(
  input: string
): Promise<AnalyzeSourceCodeResponse> {
  // @ts-ignore
  const wasm = await import(`@mv/wasm`);
  await wasm.default();

  return JSON.parse(await wasm.analyze_source_code(input));
}

export function useAnalyzeSourceCode(sourceCode: string) {
  return useQuery({
    queryKey: ['analyzeSourceCode', createSourceCodeKey(sourceCode)],
    queryFn: async () => {
      const response = appInfo.isDesktop
        ? await analyzeSourceDesktop(sourceCode)
        : await analyzeSourceWeb(sourceCode);

      if (response.error) throw response.error;
      return response;
    },
    retry: false,
  });
}
