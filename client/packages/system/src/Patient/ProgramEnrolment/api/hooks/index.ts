import { Document } from './document';
import { Utils } from './utils';

export const useProgramEnrolment = {
  utils: {
    api: Utils.useProgramEnrolmentApi,
  },

  document: {
    list: Document.useProgramEnrolments,
    insert: Document.useInsertProgramEnrolment,
    update: Document.useUpdateProgramEnrolment,
  },
};