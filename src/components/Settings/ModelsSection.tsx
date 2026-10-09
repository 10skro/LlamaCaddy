import { Brain, AlertCircle, Check, FolderOpen } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import type { AppSettings } from '@/types';
import { useDebouncedFolderInput } from '@/hooks/Settings/useDebouncedFolderInput';

interface ModelsSectionProps {
  settings: AppSettings | null;
  updateSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => void;
}

/**
 * Models section — single model root folder configuration.
 * mmproj files are found inside the same root (sub-folders included);
 * the legacy mmproj_folder setting remains supported but is no longer exposed.
 */
export function ModelsSection({ settings, updateSetting }: ModelsSectionProps) {
  const modelInput = useDebouncedFolderInput({
    settings,
    settingKey: 'model_folder',
    updateSetting,
    label: 'Model',
    scanDescription: 'Models and mmproj files will be scanned from this folder.',
  });

  return (
    <Card className="border-border/50 bg-card/50">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Brain className="h-5 w-5" />
          Models
        </CardTitle>
        <CardDescription>
          Configure the root folder where your model files are stored.
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Model Folder */}
        <div className="space-y-2">
          <Label>Model Folder</Label>
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
            Model and mmproj files are searched in this folder and its sub-folders,
            grouped by folder in the file pickers. Recommended layout: one sub-folder
            per model, with its mmproj file inside — selecting a model then
            preselects the matching mmproj automatically.
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
      </CardContent>
    </Card>
  );
}
