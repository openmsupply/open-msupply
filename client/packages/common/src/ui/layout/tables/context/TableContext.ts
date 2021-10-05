import create, { UseStore } from 'zustand';
import createContext from 'zustand/context';

export interface RowState {
  isSelected: boolean;
}

export interface TableStore {
  rowState: Record<string, RowState>;
  numberSelected: number;

  toggleSelected: (id: string) => void;
  toggleAll: () => void;
  setActiveRows: (id: string[]) => void;
}

export const { Provider: TableProvider, useStore: useTableStore } =
  createContext<TableStore>();

export const createTableStore = (): UseStore<TableStore> =>
  create<TableStore>(set => ({
    rowState: {},
    numberSelected: 0,

    toggleAll: () => {
      set(state => {
        const rowIds = Object.keys(state.rowState);
        const numberOfRows = rowIds.length;
        const isSelected = state.numberSelected !== numberOfRows;
        const numberSelected = isSelected ? numberOfRows : 0;

        return {
          ...state,
          numberSelected,
          rowState: Object.keys(state.rowState).reduce(
            (newState, id) => ({
              ...newState,
              [id]: { isSelected },
            }),
            state.rowState
          ),
        };
      });
    },

    setActiveRows: (ids: string[]) => {
      set(state => {
        const { rowState } = state;

        // Create a new row state, which is setting any newly active rows to unselected.
        const newRowState: Record<string, RowState> = ids.reduce(
          (newRowState, id) => {
            return {
              ...newRowState,
              [id]: { isSelected: rowState[id]?.isSelected ?? false },
            };
          },
          {}
        );

        const numberSelected = Object.values(newRowState).filter(
          ({ isSelected }) => isSelected
        ).length;

        return { ...state, numberSelected, rowState: newRowState };
      });
    },

    toggleSelected: (id: string) => {
      set(state => {
        const { numberSelected, rowState } = state;

        // How many rows in total are currently rendered to determine
        // if all rows, some or none are selected.
        const isSelected = !rowState[id]?.isSelected;

        // If this row is being toggled on, add one, otherwise reduce the number
        // of rows selected.
        const newNumberSelected = numberSelected + (isSelected ? 1 : -1);

        return {
          ...state,
          numberSelected: newNumberSelected,
          rowState: {
            ...state.rowState,
            [id]: { ...state.rowState[id], isSelected },
          },
        };
      });
    },
  }));