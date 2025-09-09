import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import randomColor from 'randomcolor';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function generateRandomColor(theme?: string) {
  const hue = Math.random() * 240 + 60; // excluding red

  return randomColor({
    luminosity: theme === 'dark' ? 'light' : 'dark',
    hue: hue,
  });
}
