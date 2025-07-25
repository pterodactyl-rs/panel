import createLocation from '@/api/admin/locations/createLocation';
import updateLocation from '@/api/admin/locations/updateLocation';
import { httpErrorToHuman } from '@/api/axios';
import AdminSettingContainer from '@/elements/AdminSettingContainer';
import { Button } from '@/elements/button';
import { Input } from '@/elements/inputs';
import { useToast } from '@/providers/ToastProvider';
import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router';
import BackupS3 from './forms/BackupS3';
import getLocation from '@/api/admin/locations/getLocation';
import { Dialog } from '@/elements/dialog';
import deleteLocation from '@/api/admin/locations/deleteLocation';
import Code from '@/elements/Code';
import classNames from 'classnames';

export default () => {
  const params = useParams<'id'>();
  const { addToast } = useToast();
  const navigate = useNavigate();

  const [openDialog, setOpenDialog] = useState<'delete'>(null);
  const [location, setLocation] = useState<Location | null>(null);
  const [shortName, setShortName] = useState<string>('');
  const [name, setName] = useState<string>('');
  const [description, setDescription] = useState<string>('');
  const [backupDisk, setBackupDisk] = useState<LocationConfigBackupType>('local');
  const [backupConfigs, setBackupConfigs] = useState<LocationConfigBackup>(null);

  useEffect(() => {
    if (params.id) {
      getLocation(Number(params.id))
        .then((location) => {
          setLocation(location);
          setShortName(location.shortName);
          setName(location.name);
          setDescription(location.description || '');
          setBackupDisk(location.backupDisk);
          setBackupConfigs(location.backupConfigs[location.backupDisk]);
        })
        .catch((msg) => {
          addToast(httpErrorToHuman(msg), 'error');
        });
    }
  }, [params.id]);

  const doCreateOrUpdate = () => {
    if (location?.id) {
      updateLocation(location.id, {
        shortName,
        name,
        description,
        backupDisk,
        backupConfigs: {
          [backupDisk]: backupConfigs,
        },
      })
        .then(() => {
          addToast('Location updated.', 'success');
        })
        .catch((msg) => {
          addToast(httpErrorToHuman(msg), 'error');
        });
    } else {
      createLocation({
        shortName,
        name,
        description,
        backupDisk,
        backupConfigs: {
          [backupDisk]: backupConfigs,
        },
      })
        .then((location) => {
          addToast('Location created.', 'success');
          navigate(`/admin/locations/${location.id}`);
        })
        .catch((msg) => {
          addToast(httpErrorToHuman(msg), 'error');
        });
    }
  };

  const doDelete = () => {
    deleteLocation(location.id)
      .then(() => {
        addToast('Location deleted.', 'success');
        navigate('/admin/locations');
      })
      .catch((msg) => {
        addToast(httpErrorToHuman(msg), 'error');
      });
  };

  return (
    <>
      <Dialog.Confirm
        open={openDialog === 'delete'}
        hideCloseIcon
        onClose={() => setOpenDialog(null)}
        title={'Confirm Location Deletion'}
        confirm={'Delete'}
        onConfirmed={doDelete}
      >
        Are you sure you want to delete <Code>{location?.name}</Code>?
      </Dialog.Confirm>

      <div className={'mb-4'}>
        <h1 className={'text-4xl font-bold text-white'}>{params.id ? 'Update' : 'Create'} Location</h1>
      </div>
      <AdminSettingContainer title={'Location Settings'}>
        <div className={'mt-4'}>
          <Input.Label htmlFor={'shortName'}>Short Name</Input.Label>
          <Input.Text
            id={'shortName'}
            placeholder={'Short Name'}
            value={shortName}
            onChange={(e) => setShortName(e.target.value)}
          />
        </div>
        <div className={'mt-4'}>
          <Input.Label htmlFor={'name'}>Name</Input.Label>
          <Input.Text id={'name'} placeholder={'Name'} value={name} onChange={(e) => setName(e.target.value)} />
        </div>
        <div className={'mt-4'}>
          <Input.Label htmlFor={'description'}>Description</Input.Label>
          <Input.Text
            id={'description'}
            placeholder={'Description'}
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
        </div>
        <div className={'mt-4'}>
          <Input.Label htmlFor={'type'}>Type</Input.Label>
          <Input.Dropdown
            id={'type'}
            options={[
              { label: 'Local', value: 'local' },
              { label: 'S3', value: 's3' },
              { label: 'Ddup-Bak', value: 'ddup-bak' },
              { label: 'Btrfs', value: 'btrfs' },
              { label: 'Zfs', value: 'zfs' },
              { label: 'Restic', value: 'restic' },
            ]}
            selected={backupDisk}
            onChange={(e) => setBackupDisk(e.target.value as LocationConfigBackupType)}
          />
        </div>

        {backupDisk === 's3' ? (
          <BackupS3 backupConfigs={backupConfigs as LocationConfigBackupS3} setBackupConfigs={setBackupConfigs} />
        ) : null}

        <div className={classNames('mt-4 flex', location ? 'justify-between' : 'justify-end')}>
          {location && (
            <Button style={Button.Styles.Red} onClick={() => setOpenDialog('delete')}>
              Delete
            </Button>
          )}
          <Button onClick={doCreateOrUpdate}>Save</Button>
        </div>
      </AdminSettingContainer>
    </>
  );
};
