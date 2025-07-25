import { Button } from '@/elements/button';
import { Dialog, DialogProps } from '@/elements/dialog';
import { Input } from '@/elements/inputs';
import { generateBackupName } from '@/lib/server';
import { useState } from 'react';

type Props = DialogProps & {
  onCreate: (name: string, ignoredFiles: string[]) => void;
};

export default ({ onCreate, open, onClose }: Props) => {
  const [name, setName] = useState(generateBackupName());
  const [ignoredFiles, setIgnoredFiles] = useState<string[]>([]);

  return (
    <Dialog title={'Create Backup'} onClose={onClose} open={open} hideCloseIcon>
      <div className={'mt-4'}>
        <Input.Label htmlFor={'name'}>Name</Input.Label>
        <Input.Text id={'name'} name={'name'} value={name} onChange={(e) => setName(e.target.value)} />
      </div>

      <div className={'mt-4'}>
        <Input.Label htmlFor={'ignoredFiles'}>Ignored Files</Input.Label>
        <Input.MultiInput placeholder={'Path or file'} options={ignoredFiles} onChange={setIgnoredFiles} />
      </div>

      <Dialog.Footer>
        <Button style={Button.Styles.Gray} onClick={onClose}>
          Close
        </Button>
        <Button style={Button.Styles.Green} onClick={() => onCreate(name, ignoredFiles)} disabled={!name}>
          Create
        </Button>
      </Dialog.Footer>
    </Dialog>
  );
};
