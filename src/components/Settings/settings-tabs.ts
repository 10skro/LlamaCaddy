import { Settings2, Palette, RefreshCw } from 'lucide-react';

export type SettingsTab = 'general' | 'appearance' | 'updates';

export const SETTINGS_TABS: { id: SettingsTab; label: string; icon: typeof Settings2 }[] = [
  { id: 'general', label: 'General', icon: Settings2 },
  { id: 'appearance', label: 'Appearance', icon: Palette },
  { id: 'updates', label: 'Updates', icon: RefreshCw },
];
