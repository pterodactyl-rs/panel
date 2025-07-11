import getFileContent from '@/api/server/files/getFileContent';
import Spinner from '@/elements/Spinner';
import { getLanguageFromExtension } from '@/lib/files';
import { urlPathToAction, urlPathToFilePath } from '@/lib/path';
import { useServerStore } from '@/stores/server';
import { Editor } from '@monaco-editor/react';
import { useEffect, useRef, useState } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router';
import { FileBreadcrumbs } from './FileBreadcrumbs';
import { Button } from '@/elements/button';
import saveFileContent from '@/api/server/files/saveFileContent';
import FileNameDialog from './dialogs/FileNameDialog';

export default () => {
  const params = useParams<'path'>();
  const location = useLocation();
  const navigate = useNavigate();
  const action = urlPathToAction(location.pathname);
  const server = useServerStore(state => state.server);
  const { browsingDirectory, setBrowsingDirectory } = useServerStore();

  const [loading, setLoading] = useState(false);
  const [nameDialogOpen, setNameDialogOpen] = useState(false);
  const [content, setContent] = useState('');
  const [language, setLanguage] = useState('plaintext');

  const editorRef = useRef(null);
  const contentRef = useRef(content);

  useEffect(() => {
    setBrowsingDirectory(decodeURIComponent(params.path || '/'));
  }, [params]);

  useEffect(() => {
    if (action === 'new') return;

    setLoading(true);
    getFileContent(server.uuid, browsingDirectory).then(content => {
      setContent(content);
      setLanguage(getLanguageFromExtension(browsingDirectory.split('.').pop()));
      setLoading(false);
    });
  }, [browsingDirectory]);

  useEffect(() => {
    contentRef.current = content;
  }, [content]);

  const saveFile = (name?: string) => {
    if (!editorRef.current) return;

    const currentContent = editorRef.current.getValue();
    setLoading(true);

    saveFileContent(server.uuid, name ?? browsingDirectory, currentContent).then(() => {
      setLoading(false);
      setNameDialogOpen(false);

      if (name) {
        navigate(`/server/${server.uuidShort}/files/edit/${encodeURIComponent(name)}`);
      }
    });
  };

  return loading ? (
    <div className="w-full h-screen flex items-center justify-center">
      <Spinner size={75} />
    </div>
  ) : (
    <div className="flex flex-col w-full">
      <FileNameDialog
        onFileNamed={(name: string) => saveFile(name)}
        open={nameDialogOpen}
        onClose={() => setNameDialogOpen(false)}
      />

      <div className="flex justify-between w-full p-4">
        <FileBreadcrumbs path={decodeURIComponent(browsingDirectory)} />
        <div>
          {action === 'edit' ? (
            <Button style={Button.Styles.Green} onClick={() => saveFile()}>
              Save
            </Button>
          ) : (
            <Button style={Button.Styles.Green} onClick={() => setNameDialogOpen(true)}>
              Create
            </Button>
          )}
        </div>
      </div>
      <Editor
        height="100%"
        theme="vs-dark"
        defaultLanguage={language}
        defaultValue={content}
        onChange={setContent}
        onMount={(editor, monaco) => {
          editorRef.current = editor;

          editor.onDidChangeModelContent(() => {
            contentRef.current = editor.getValue();
          });

          editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
            if (action === 'new') {
              setNameDialogOpen(true);
            } else {
              saveFile();
            }
          });
        }}
      />
    </div>
  );
};
