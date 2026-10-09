import { useEffect, useState } from 'react';
import { HardDrive, FolderOpen } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { getStoragePath, openStorageFolder } from '@/services/settings';
import { useToast } from '@/hooks/use-toast';

/**
 * Storage section: shows where llama.cpp versions are stored and lets the
 * user open that folder in the file manager. The storage path itself is not
 * editable (changing it was dropped as a feature).
 */
export function StorageSection() {
  const { toast } = useToast();
  const [storagePath, setStoragePath] = useState('');

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
          <HardDrive className="h-5 w-5" />
          Storage
        </CardTitle>
        <CardDescription>Where llama.cpp versions are stored.</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label>Storage Path</Label>
          <div className="flex gap-2">
            <Input
              value={storagePath}
              readOnly
              className="bg-background/50 font-mono text-sm cursor-default"
            />
            <Button variant="outline" size="icon" title="Open storage folder" onClick={handleOpenFolder}>
              <FolderOpen className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
