import React, { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  TableProvider,
  DataTable,
  useColumns,
  createTableStore,
  NothingHere,
  createQueryParamsStore,
  useFormatDateTime,
  ColumnAlign,
  useAlertModal,
  useTranslation,
  useUrlQueryParams,
} from '@openmsupply-client/common';
import { usePatient, PatientRowFragment } from '../api';
import { AppBarButtons } from './AppBarButtons';
import { Toolbar } from './Toolbar';

const PatientListComponent: FC = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    filter,
    queryParams: { sortBy, page, first, offset },
  } = useUrlQueryParams();
  const { data, isError, isLoading } = usePatient.document.list();
  const pagination = { page, first, offset };
  const t = useTranslation('patients');
  const { localisedDate } = useFormatDateTime();
  const navigate = useNavigate();
  const alert = useAlertModal({
    title: t('error.something-wrong'),
    message: t('messages.no-patient-record'),
    onOk: () => {},
  });

  const columns = useColumns<PatientRowFragment>(
    [
      { key: 'code', label: 'label.code' },
      {
        key: 'firstName',
        label: 'label.first-name',
      },
      {
        key: 'lastName',
        label: 'label.last-name',
      },
      {
        key: 'gender',
        label: 'label.gender',
      },
      {
        key: 'dateOfBirth',
        label: 'label.date-of-birth',
        align: ColumnAlign.Right,
        width: 175,
        formatter: dateString =>
          dateString ? localisedDate((dateString as string) || '') : '',
      },
    ],
    { onChangeSortBy: updateSortQuery, sortBy },
    [sortBy]
  );

  return (
    <>
      <Toolbar filter={filter} />
      <AppBarButtons sortBy={sortBy} />
      <DataTable
        key="patient-list"
        pagination={{ ...pagination, total: data?.totalCount }}
        onChangePage={updatePaginationQuery}
        columns={columns}
        data={data?.nodes}
        isLoading={isLoading}
        isError={isError}
        onRowClick={row => {
          if (!row.id || !row.document?.name || !row.document?.type) alert();
          else navigate(`${row.id}?doc=${row.document.name}`);
        }}
        noDataElement={<NothingHere />}
      />
    </>
  );
};

export const PatientListView: FC = () => (
  <TableProvider
    createStore={createTableStore}
    queryParamsStore={createQueryParamsStore<PatientRowFragment>({
      initialSortBy: { key: 'name' },
    })}
  >
    <PatientListComponent />
  </TableProvider>
);
