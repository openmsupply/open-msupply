import React from 'react';
import { CustomerSearchModal } from '@openmsupply-client/system';
import {
  ButtonWithIcon,
  Grid,
  PlusCircleIcon,
  useNotification,
  useQuery,
  StatsPanel,
  Widget,
  FnUtils,
  useToggle,
} from '@openmsupply-client/common';
import { useFormatNumber, useTranslation } from '@common/intl';
import { useOutbound } from '../api';

export const OutboundShipmentWidget: React.FC = () => {
  const modalControl = useToggle(false);
  const { error } = useNotification();
  const t = useTranslation(['app', 'dashboard']);
  const formatNumber = useFormatNumber();
  const [hasError, setHasError] = React.useState(false);

  const api = useOutbound.utils.api();
  const { data, isLoading } = useQuery(
    ['outbound-shipment', 'count'],
    api.dashboard.shipmentCount,
    { retry: false, onError: () => setHasError(true) }
  );

  const { mutate: onCreate } = useOutbound.document.insert();

  return (
    <>
      <CustomerSearchModal
        open={modalControl.isOn}
        onClose={modalControl.toggleOff}
        onChange={async name => {
          modalControl.toggleOff();
          try {
            await onCreate({
              id: FnUtils.generateUUID(),
              otherPartyId: name?.id,
            });
          } catch (e) {
            const errorSnack = error(
              'Failed to create invoice! ' + (e as Error).message
            );
            errorSnack();
          }
        }}
      />

      <Widget title={t('outbound-shipments')}>
        <Grid
          container
          justifyContent="flex-start"
          flex={1}
          flexDirection="column"
        >
          <Grid item>
            {!hasError && (
              <StatsPanel
                isLoading={isLoading}
                title={t('heading.shipments-to-be-picked')}
                stats={[
                  {
                    label: t('label.today', { ns: 'dashboard' }),
                    value: formatNumber.round(data?.toBePicked),
                  },
                ]}
              />
            )}
          </Grid>
          <Grid
            item
            flex={1}
            container
            justifyContent="flex-end"
            alignItems="flex-end"
          >
            <ButtonWithIcon
              variant="contained"
              color="secondary"
              Icon={<PlusCircleIcon />}
              label={t('button.new-outbound-shipment')}
              onClick={modalControl.toggleOn}
            />
          </Grid>
        </Grid>
      </Widget>
    </>
  );
};
