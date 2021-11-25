import React, { FC } from 'react';
import {
  AppBarContentPortal,
  Box,
  InputWithLabelRow,
  BasicTextInput,
  Grid,
  DropdownMenu,
  DropdownMenuItem,
  DeleteIcon,
  useTranslation,
  useNotification,
  useTableStore,
} from '@openmsupply-client/common';
import { NameSearchInput } from '@openmsupply-client/system/src/Name';
import { InboundShipment, InboundShipmentItem } from '../../types';
import { isInboundEditable } from '../../utils';

interface ToolbarProps {
  draft: InboundShipment;
}

export const Toolbar: FC<ToolbarProps> = ({ draft }) => {
  const t = useTranslation(['distribution', 'common']);
  const { success, info } = useNotification();

  const { selectedRows } = useTableStore(state => ({
    selectedRows: Object.keys(state.rowState)
      .filter(id => state.rowState[id]?.isSelected)
      .map(selectedId => draft.items.find(({ id }) => selectedId === id))
      .filter(Boolean) as InboundShipmentItem[],
  }));

  const deleteAction = () => {
    if (selectedRows && selectedRows?.length > 0) {
      selectedRows.forEach(item =>
        Object.values(item.batches).forEach(line => draft.deleteLine?.(line))
      );
      const successSnack = success(`Deleted ${selectedRows?.length} lines`);
      successSnack();
    } else {
      const infoSnack = info(t('label.select-rows-to-delete-them'));
      infoSnack();
    }
  };

  return (
    <AppBarContentPortal sx={{ display: 'flex', flex: 1, marginBottom: 1 }}>
      <Grid
        container
        flexDirection="row"
        display="flex"
        flex={1}
        alignItems="flex-end"
      >
        <Grid item display="flex" flex={1}>
          <Box display="flex" flex={1} flexDirection="column" gap={1}>
            {draft.otherParty && (
              <InputWithLabelRow
                label={t('label.supplier-name')}
                Input={
                  <NameSearchInput
                    type="supplier"
                    disabled={!isInboundEditable(draft)}
                    value={draft.otherParty}
                    onChange={name => {
                      draft.update?.('otherParty', name);
                    }}
                  />
                }
              />
            )}
            <InputWithLabelRow
              label={t('label.supplier-ref')}
              Input={
                <BasicTextInput
                  disabled={!isInboundEditable(draft)}
                  size="small"
                  sx={{ width: 250 }}
                  value={draft?.theirReference ?? ''}
                  onChange={event => {
                    draft.update?.('theirReference', event.target.value);
                  }}
                />
              }
            />
          </Box>
        </Grid>
        <DropdownMenu
          disabled={!isInboundEditable(draft)}
          label={t('label.select')}
        >
          <DropdownMenuItem IconComponent={DeleteIcon} onClick={deleteAction}>
            {t('button.delete-lines', { ns: 'distribution' })}
          </DropdownMenuItem>
        </DropdownMenu>
      </Grid>
    </AppBarContentPortal>
  );
};