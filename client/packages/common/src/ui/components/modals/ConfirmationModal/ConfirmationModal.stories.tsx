import React from 'react';
import { Grid } from '@mui/material';
import { Story } from '@storybook/react';
import { ConfirmationModal } from './ConfirmationModal';
import { BaseButton } from '../../buttons';
import { useToggle } from '../../../../hooks';
import { useConfirmationModal } from './useConfirmationModal';

export default {
  title: 'Modals/ConfirmationModal',
  component: ConfirmationModal,
};

const Template: Story = () => {
  const modalControl = useToggle(false);

  return (
    <Grid>
      <BaseButton onClick={modalControl.toggleOn}>Open Modal</BaseButton>
      <ConfirmationModal
        title={'Are you sure?'}
        open={modalControl.isOn}
        onConfirm={() => {
          alert('Confirmed');
          modalControl.toggleOff();
        }}
        onCancel={modalControl.toggleOff}
        message={'This will delete all your data.'}
      />
    </Grid>
  );
};

const Loading: Story = () => {
  const modalControl = useToggle(false);

  return (
    <Grid>
      <BaseButton onClick={modalControl.toggleOn}>Open Modal</BaseButton>
      <ConfirmationModal
        title={'Are you sure?'}
        open={modalControl.isOn}
        onConfirm={async () => {
          await new Promise(resolve => setTimeout(resolve, 1000));
          alert('Confirmed');
          modalControl.toggleOff();
        }}
        onCancel={modalControl.toggleOff}
        message={'This will delete all your data.'}
      />
    </Grid>
  );
};

const UseConfirmationModalHook: Story = () => {
  const getConfirmation = useConfirmationModal({
    title: 'Are you sure?',
    message: 'This will delete all your data.',
    onConfirm: () => alert('confirmed!'),
  });

  return <BaseButton onClick={() => getConfirmation()}>Open Modal</BaseButton>;
};

export const Primary = Template.bind({});
export const WithAsyncConfirmation = Loading.bind({});
export const WithContextAndHook = UseConfirmationModalHook.bind({});
