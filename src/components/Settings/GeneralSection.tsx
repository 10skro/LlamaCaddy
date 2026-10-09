import { useEffect, useState } from 'react';
import { FolderCog, FolderOpen, Brain, HardDrive, AlertCircle, Check } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Separator } from '@/components/ui/separator';
import type { AppSettings } from '@/types';
import { useDebouncedFolderInput } from '@/hooks/Settings/useDebouncedFolderInput';
import { getStoragePath, openStorageFolder } from '@/services/settings';
import { useToast } from '@/hooks/use-toast';

interface GeneralSectionProps {
  settings: AppSettings | null;
  updateSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => void;
}

/**
 * General section — folder locations:
 * - model root folder (editable, validated with debounce)
 * - llama.cpp storage path (read-only, with an open-folder button)
 */
export function GeneralSection({ settings, updateSetting }: GeneralSectionProps) {
  const { toast } = useToast();
  const [storagePath, setStoragePath] = useState('');

  const modelInput = useDebouncedFolderInput({
    settings,
    settingKey: 'model_folder',
    updateSetting,
    label: 'Model',
    scanDescription: 'Models and mmproj files will be scanned from this folder.',
  });

  useEffect(() => {
    getStoragePath()
      .then(setStoragePath)
      .catch((err) => console.error('Failed to load storage path:', err));
  }, []);

  const handleOpenFolder = async () => {
    try {
      await openStorageFolder();
    } catch (err) {
      toast({
        title: 'Unable to open folder',
        description: String(err),
        variant: 'destructive',
      });
    }
  };

  return (
    <Card className="border-border/50 bg-card/50">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <FolderCog className="h-5 w-5" />
          Folders
        </CardTitle>
        <CardDescription>Where your models and llama.cpp builds are stored.</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Model Folder */}
        <div className="space-y-2">
          <Label className="flex items-center gap-2">
            <Brain className="h-3.5 w-3.5 text-muted-foreground" />
            Model Folder
          </Label>
          <div className="flex gap-2">
            <Input
              value={modelInput.value}
              onChange={(e) => modelInput.handleChange(e.target.value)}
              placeholder="Select a folder containing your models"
              className="bg-background/50 font-mono text-sm"
            />
            <Button
              variant="outline"
              size="icon"
              title="Browse for model folder"
              onClick={modelInput.handleBrowse}
            >
              <FolderOpen className="h-4 w-4" />
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            Model and mmproj files are searched in this folder and its sub-folders, grouped by
            folder in the file pickers. Recommended layout: one sub-folder per model, with its
            mmproj file inside — selecting a model then preselects the matching mmproj
            automatically.
          </p>
          {modelInput.validation === 'invalid' && (
            <p className="text-xs text-destructive flex items-center gap-1">
              <AlertCircle className="h-3 w-3" />
              Folder does not exist or is not accessible.
            </p>
          )}
          {modelInput.validation === 'valid' && (
            <p className="text-xs text-green flex items-center gap-1">
              <Check className="h-3 w-3" />
              Folder exists and is accessible.
            </p>
          )}
        </div>

        <Separator className="border-border/50" />

        {/* Storage Path (read-only) */}
        <div className="space-y-2">
          <Label className="flex items-center gap-2">
            <HardDrive className="h-3.5 w-3.5 text-muted-foreground" />
            llama.cpp Storage Path
          </Label>
          <div className="flex items-center gap-2">
            <p className="flex-1 min-w-0 truncate rounded-md border border-border/40 bg-transparent px-3 py-2 font-mono text-sm text-muted-foreground select-all">
              {storagePath}
            </p>
            <Button
              variant="outline"
              size="icon"
              title="Open storage folder"
              onClick={handleOpenFolder}
            >
              <FolderOpen className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
