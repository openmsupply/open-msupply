import React from 'react';
import { LocaleKey } from '../../../../intl/intlHelpers';
import { ArrowRightIcon, CheckIcon, XCircleIcon } from '../../../icons';
import { ButtonWithIcon } from './ButtonWithIcon';

type DialogButtonVariant = 'cancel' | 'next' | 'ok';

interface DialogButtonProps {
  disabled?: boolean;
  onClick: (event: React.MouseEvent<HTMLButtonElement>) => void;
  variant: DialogButtonVariant;
}

const getButtonProps = (
  variant: DialogButtonVariant
): {
  icon: JSX.Element;
  labelKey: LocaleKey;
  variant: 'outlined' | 'contained';
} => {
  switch (variant) {
    case 'cancel':
      return {
        icon: <XCircleIcon />,
        labelKey: 'button.cancel',
        variant: 'outlined',
      };
    case 'ok':
      return {
        icon: <CheckIcon />,
        labelKey: 'button.ok',
        variant: 'contained',
      };
    case 'next':
      return {
        icon: <ArrowRightIcon />,
        labelKey: 'button.ok-and-next',
        variant: 'contained',
      };
  }
};

export const DialogButton: React.FC<DialogButtonProps> = ({
  onClick,
  variant,
  disabled = false,
}) => {
  const { variant: buttonVariant, icon, labelKey } = getButtonProps(variant);

  return (
    <ButtonWithIcon
      color="secondary"
      disabled={disabled}
      onClick={onClick}
      Icon={icon}
      variant={buttonVariant}
      labelKey={labelKey}
      sx={
        disabled
          ? {
              '& svg': { color: theme => theme.palette.midGrey },
              fontSize: '12px',
            }
          : {}
      }
    />
  );
};