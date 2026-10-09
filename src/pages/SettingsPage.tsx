import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '@/store/useAppStore';
import { useTheme } from '@/hooks/useTheme';
import type { AppSettings } from '@/types';
import {
  GeneralSection,
  AppearanceSection,
  UpdatesSection,
  SettingsNav,
  type SettingsTab,
} from '@/components/Settings';

export function SettingsPage() {
  const { settings, setSettings } = useAppStore();
  const { activeTheme, setActiveTheme } = useTheme();
  const appUpdateLastChecked = useAppStore((s) => s.appUpdateLastChecked);
  const [appVersion, setAppVersion] = useState('...');
  const [tab, setTab] = useState<SettingsTab>('general');

  // Load app version on mount
  useEffect(() => {
    invoke<string>('get_app_version')
      .then((v) => setAppVersion(v))
      .catch(() => {});
  }, []);

  const updateSetting = <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => {
    if (!settings) return;
    setSettings({ ...settings, [key]: value });
  };

  return (
    <div className="flex h-full overflow-hidden">
      <SettingsNav active={tab} onSelect={setTab} appVersion={appVersion} />

      <div className="flex-1 min-w-0 overflow-y-auto">
        <div className="flex flex-col gap-6 p-6 max-w-3xl">
          <div>
            <h1 className="text-2xl font-semibold tracking-tight">Settings</h1>
            <p className="text-muted-foreground mt-1">Configure your LlamaCaddy preferences.</p>
          </div>

          {tab === 'general' && (
            <GeneralSection settings={settings} updateSetting={updateSetting} />
          )}
          {tab === 'appearance' && (
            <AppearanceSection
              settings={settings}
              updateSetting={updateSetting}
              activeTheme={activeTheme}
              setActiveTheme={setActiveTheme}
            />
          )}
          {tab === 'updates' && (
            <UpdatesSection
              settings={settings}
              updateSetting={updateSetting}
              appUpdateLastChecked={appUpdateLastChecked}
            />
          )}
        </div>
      </div>
    </div>
  );
}
