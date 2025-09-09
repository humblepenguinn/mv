export function removeComments(sourceCode: string): string {
  let result = '';
  let i = 0;

  while (i < sourceCode.length) {
    if (
      i < sourceCode.length - 1 &&
      sourceCode[i] === '/' &&
      sourceCode[i + 1] === '/'
    ) {
      while (i < sourceCode.length && sourceCode[i] !== '\n') {
        i++;
      }
      continue;
    }

    if (
      i < sourceCode.length - 1 &&
      sourceCode[i] === '/' &&
      sourceCode[i + 1] === '*'
    ) {
      i += 2;
      while (i < sourceCode.length - 1) {
        if (sourceCode[i] === '*' && sourceCode[i + 1] === '/') {
          i += 2;
          break;
        }
        i++;
      }
      continue;
    }

    result += sourceCode[i];
    i++;
  }

  return result;
}

export function normalizeWhitespace(sourceCode: string): string {
  return sourceCode
    .replace(/\r\n/g, '\n')
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line.length > 0)
    .join('\n')
    .replace(/\s+/g, ' ')
    .trim();
}

export function compressSourceCode(sourceCode: string): string {
  const withoutComments = removeComments(sourceCode);
  return normalizeWhitespace(withoutComments);
}

export function createSourceCodeKey(sourceCode: string): string {
  const compressed = compressSourceCode(sourceCode);

  let hash = 0;
  for (let i = 0; i < compressed.length; i++) {
    const char = compressed.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash;
  }

  return `source_${Math.abs(hash).toString(36)}_${compressed.length}`;
}
