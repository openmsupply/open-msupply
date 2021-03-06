import { useEffect, useState } from 'react';
import create from 'zustand';
import { AppRoute } from '@openmsupply-client/config';
import {
  AuthenticationError,
  AuthError,
  LocalStorage,
  ServerStatus,
  useNavigate,
} from '@openmsupply-client/common';
import { useHost } from '../../api/hooks';

const SERVER_RESTART_TIMEOUT = 60000;
const POLLING_INTERVAL = 3000;
const POLLING_DELAY = 6000;

interface InitialiseForm {
  error?: AuthenticationError;
  isLoading: boolean;
  password: string;
  siteId?: number;
  username: string;
  url: string;
  setError: (error?: AuthenticationError) => void;
  setIsLoading: (isLoading: boolean) => void;
  setPassword: (password: string) => void;
  setUsername: (username: string) => void;
  setSiteId: (siteId: number) => void;
  setUrl: (url: string) => void;
}

const useInitialiseFormState = create<InitialiseForm>(set => ({
  error: undefined,
  isLoading: false,
  password: '',
  username: '',
  url: 'https://',
  siteId: undefined,
  setError: (error?: AuthenticationError) =>
    set(state => ({ ...state, error })),
  setIsLoading: (isLoading: boolean) => set(state => ({ ...state, isLoading })),
  setPassword: (password: string) => set(state => ({ ...state, password })),
  setUsername: (username: string) => set(state => ({ ...state, username })),
  setUrl: (url: string) => set(state => ({ ...state, url })),
  setSiteId: (siteId: number) => set(state => ({ ...state, siteId })),
}));

export const useInitialiseForm = () => {
  const state = useInitialiseFormState();
  const navigate = useNavigate();
  const { mutateAsync: restart } = useHost.utils.restart();
  const {
    setIsLoading,
    password,
    setPassword,
    username,
    error,
    setError,
    url,
    siteId,
  } = state;
  const [isPolling, setIsPolling] = useState(false);
  const [isBootstrap, setIsBootstrap] = useState(false);
  const { mutateAsync: update } = useHost.sync.update();
  const { data } = useHost.utils.settings({
    refetchInterval: POLLING_INTERVAL,
    enabled: isPolling,
  });

  const onSave = async () => {
    setError();
    setIsLoading(true);
    setIsBootstrap(false);
    const syncSettings = {
      centralServerSiteId: 1,
      intervalSec: 300,
      password,
      siteId: siteId || 2,
      url,
      username,
    };

    await update(syncSettings).catch(e => {
      console.error(e);
      setError({ message: 'Unable to save settings' });
      setIsLoading(false);
      return;
    });
    setPassword('');

    await restart().catch(e => {
      console.error(e);
      setError({ message: 'Unable to restart the server' });
      setIsLoading(false);
      return;
    });

    setTimeout(() => {
      setIsPolling(true);
    }, POLLING_DELAY);

    LocalStorage.removeItem('/auth/error');
    LocalStorage.addListener<AuthError>((key, value) => {
      if (key === '/auth/error' && value === AuthError.Unauthenticated) {
        // Server is up! and rejecting our request!
        setIsLoading(false);
        setIsPolling(false);

        navigate(`/${AppRoute.Login}`, { replace: true });
      }
    });

    setTimeout(() => {
      setIsLoading(false);
      setIsPolling(false);
      const message = isBootstrap
        ? 'Unable to sync! Please check your settings.'
        : 'Server restart has timed out';
      setError({ message });
    }, SERVER_RESTART_TIMEOUT);
  };

  const isValid = !!username && !!password && !!url;
  useEffect(() => {
    if (!!data) setIsBootstrap(data?.status === ServerStatus.Stage_0);
  }, [data]);
  return {
    isValid,
    onSave,
    ...state,
    error,
  };
};
