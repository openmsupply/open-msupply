import { LocaleKey } from '@common/intl';

export enum Platform {
  Android,
  Desktop,
  Web,
}
interface RouteMapping {
  title?: LocaleKey;
  docs: string;
}

const mapRoute = (route: string): RouteMapping => {
  const inRoute = (sub: string) => new RegExp(`/${sub}/|/${sub}\$`).test(route);
  switch (true) {
    case inRoute('dashboard'):
      return { title: 'dashboard', docs: '/dashboard/' };
    case inRoute('outbound-shipment'):
      return {
        title: 'outbound-shipments',
        docs: '/distribution/outbound-shipments/',
      };
    case inRoute('customer-requisition'):
      return {
        title: 'customer-requisition',
        docs: '/distribution/requisitions/',
      };
    case inRoute('customers'):
      return { title: 'customers', docs: '/distribution/customers/' };
    case inRoute('inbound-shipment'):
      return {
        title: 'inbound-shipments',
        docs: '/replenishment/inbound-shipments/',
      };
    case inRoute('internal-order'):
      return {
        title: 'internal-order',
        docs: '/replenishment/internal-orders/',
      };
    case inRoute('suppliers'):
      return { title: 'suppliers', docs: '/replenishment/suppliers/' };
    case inRoute('items'):
      return { title: 'items', docs: '/catalogue/items/' };
    case inRoute('master-lists'):
      return { title: 'master-lists', docs: '/catalogue/master-list/' };
    case inRoute('locations'):
      return { title: 'locations', docs: '/inventory/locations/' };
    case inRoute('stock'):
      return { title: 'stock', docs: '/inventory/stock-view/' };
    case inRoute('stocktakes'):
      return { title: 'stocktakes', docs: '/inventory/stock-takes/' };
    case inRoute('sync'):
      return { title: 'sync', docs: '/introduction/' };
    case inRoute('admin'):
      return { title: 'admin', docs: '/introduction/' };
    case inRoute('patients'):
      return { title: 'patients', docs: '/introduction/' };
    default:
      return { title: undefined, docs: '/introduction/' };
  }
};

const getPlatform = () => {
  // 'Mozilla/5.0 (Linux; Android 8.1.0; Lenovo TB-X304L Build/OPM1.171019.026; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/102.0.5005.78 Safari/537.36'
  const userAgent = navigator.userAgent.toLowerCase();
  switch (true) {
    case / android/i.test(userAgent):
      return Platform.Android;
    case / electron/i.test(userAgent):
      return Platform.Desktop;
    default:
      return Platform.Web;
  }
};

export const EnvUtils = {
  // Using isProduction rather than isDevelopment, as we also use a testing
  // environment while running jest, so easier to check !isProduction, generally.
  isProduction: (): boolean => process.env['NODE_ENV'] === 'production',
  mapRoute,
  platform: getPlatform(),
};
