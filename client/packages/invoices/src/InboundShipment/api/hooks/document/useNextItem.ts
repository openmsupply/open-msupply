import { InboundLineFragment } from '../../operations.generated';
import { useInboundItems } from '../line/useInboundItems';

type NextInboundLineItem =  InboundLineFragment["item"];

export const useNextItem = (
  currentItemId: string
): { next: NextInboundLineItem | null; disabled: boolean } => {
  const next: NextInboundLineItem | null = null;
  const disabled = true;
  const { data } = useInboundItems();

  if (!data) return { next, disabled };

  const numberOfItems = data.length;
  const currentIndex = data.findIndex(({ itemId }) => itemId === currentItemId);
  const nextIndex = currentIndex + 1;
  const nextItem = data?.[nextIndex];
  if (!nextItem) return { next, disabled };

  return {
    next: nextItem.lines[0]?.item || null,
    disabled: currentIndex === numberOfItems - 1,
  };
};
