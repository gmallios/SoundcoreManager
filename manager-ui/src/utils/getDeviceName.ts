import productNames from '@assets/productNames.json';

export const getDeviceName = (model: keyof typeof productNames | null | undefined): string => {
  if (!model) return '';
  return productNames[model] ?? '';
};