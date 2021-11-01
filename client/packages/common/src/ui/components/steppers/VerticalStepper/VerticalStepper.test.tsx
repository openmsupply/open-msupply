import React from 'react';
import { render } from '@testing-library/react';
import { VerticalStepper } from './VerticalStepper';
import { TestingProvider } from '../../../..';

describe('VerticalStepper', () => {
  it('renders the description of each step', () => {
    const { getByText } = render(
      // The stepper doesn't use any sort of role, so just querying by text
      // the description to ensure that at minimum we're rendering that.

      <TestingProvider>
        <VerticalStepper
          activeStep={0}
          steps={[
            { label: 'app.admin', description: 'admin' },
            { label: 'app.catalogue', description: 'catalogue' },
            { label: 'app.customers', description: 'customers' },
          ]}
        />
      </TestingProvider>
    );

    const node1 = getByText('admin');
    const node2 = getByText('catalogue');
    const node3 = getByText('customers');

    expect(node1).toBeInTheDocument();
    expect(node2).toBeInTheDocument();
    expect(node3).toBeInTheDocument();
  });

  it('renders the correct active/completed states correctly', () => {
    const { getByTestId } = render(
      <TestingProvider>
        <VerticalStepper
          activeStep={1}
          steps={[
            { label: 'app.admin', description: 'admin' },
            { label: 'app.catalogue', description: 'catalogue' },
            { label: 'app.customers', description: 'customers' },
          ]}
        />
      </TestingProvider>
    );

    const node1 = getByTestId('completed');
    const node2 = getByTestId('activecompleted');

    expect(node1).toBeInTheDocument();
    expect(node2).toBeInTheDocument();
  });
});