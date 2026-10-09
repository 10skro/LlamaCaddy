import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '@/store/useAppStore';
import { saveSettings } from '@/services/settings';

export interface PendingChangelog {
  version: string;
  body: string;
}

/**
 * Shows the changelog of a just-installed update on next startup.
 * install_app_update persists pending_changelog_version/body in the database
 * before exiting; this hook reads them once after settings load, shows the
 * changelog only if the app version now matches the pending version (i.e. the
 * update actually landed), and clears the pending entry so it never reappears.
 */
export function usePendingChangelog() {
  const settings = useAppStore((s) => s.settings);
  const [pending, setPending] = useState<PendingChangelog | null>(null);
  const [handled, setHandled] = useState(false);

  useEffect(() => {
    if (handled || !settings) return;
    const version = settings.pending_changelog_version;
    const body = settings.pending_changelog_body;
    if (!version || !body) return;
    setHandled(true);

    (async () => {
      try {
        const current = await invoke<string>('get_app_version');
        // Only announce the changelog when the update actually landed.
        if (current === version) {
          setPending({ version, body });
        }
      } catch (err) {
        console.error('Failed to get app version for pending changelog:', err);
      }
      // Always clear the pending entry (store + backend) so it shows at most once.
      const next = {
        ...settings,
        pending_changelog_version: undefined,
        pending_changelog_body: undefined,
      };
      useAppStore.getState().setSettings(next);
      await saveSettings(next).catch((err) =>
        console.error('Failed to clear pending changelog:', err)
      );
    })();
  }, [settings, handled]);

  const dismiss = () => setPending(null);

  return { pending, dismiss };
}
