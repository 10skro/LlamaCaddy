import { Settings2, Palette, RefreshCw } from 'lucide-react';

export type SettingsTab = 'general' | 'appearance' | 'updates';

export const SETTINGS_TABS: { id: SettingsTab; label: string; icon: typeof Settings2 }[] = [
  { id: 'general', label: 'General', icon: Settings2 },
  { id: 'appearance', label: 'Appearance', icon: Palette },
  { id: 'updates', label: 'Updates', icon: RefreshCw },
];

interface SettingsNavProps {
  active: SettingsTab;
  onSelect: (tab: SettingsTab) => void;
  appVersion: string;
}

/**
 * Left-hand navigation for the Settings page: category list plus a discreet
 * version footer (replaces the old About card).
 */
export function SettingsNav({ active, onSelect, appVersion }: SettingsNavProps) {
  return (
    <nav className="flex flex-col justify-between w-56 shrink-0 border-r border-border bg-card/40 p-4">
      <div className="space-y-1 pt-2">
        {SETTINGS_TABS.map(({ id, label, icon: Icon }) => {
          const isActive = active === id;
          return (
            <button
              key={id}
              type="button"
              onClick={() => onSelect(id)}
              className={`flex items-center gap-2.5 w-full px-3 py-2 rounded-md text-sm transition-colors border ${
                isActive
                  ? 'bg-accent/10 text-foreground border-accent/30 font-medium'
                  : 'text-muted-foreground border-transparent hover:bg-accent/5 hover:text-foreground'
              }`}
            >
              <Icon className="h-4 w-4" />
              {label}
            </button>
          );
        })}
      </div>
      <div className="px-3 py-2 text-xs text-muted-foreground space-y-0.5">
        <p>
          LlamaCaddy{' '}
          {appVersion && appVersion !== '...' ? `v${appVersion}` : ''}
        </p>
        <p className="text-[11px] opacity-70">Built with Tauri, React, and Rust.</p>
      </div>
    </nav>
  );
}
