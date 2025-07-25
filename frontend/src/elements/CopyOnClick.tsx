import { useToast } from '@/providers/ToastProvider';

export default ({ content, children }: { content: string; children: React.ReactNode }) => {
  const { addToast } = useToast();

  const handleCopy = (e: React.MouseEvent) => {
    e.preventDefault();

    if (!window.isSecureContext) {
      addToast('This feature is only available in secure contexts (HTTPS).', 'error');
      return;
    }

    try {
      navigator.clipboard.writeText(content);
      addToast('Copied to clipboard');
    } catch (err) {
      console.log(err);
    }
  };
  return (
    <button onClick={handleCopy} className={'cursor-pointer'}>
      {children}
    </button>
  );
};
