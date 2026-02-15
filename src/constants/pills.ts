import type { PillType } from '@/types';

export const PILL_TYPES: PillType[] = [
  { mg: 1, colorClass: 'bg-gray-300' },
  { mg: 2, colorClass: 'bg-orange-300' },
  { mg: 3, colorClass: 'bg-sky-400' },
  { mg: 5, colorClass: 'bg-pink-400' },
];

export const DEFAULT_AVAILABLE_PILLS: Record<number, boolean> = {
  1: false,
  2: true,
  3: true,
  5: true,
};
