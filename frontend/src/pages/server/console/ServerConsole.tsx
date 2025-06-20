import Container from '@/elements/Container';
import Console from './Console';
import Spinner from '@/elements/Spinner';
import ServerDetails from './ServerDetails';
import ServerPowerControls from './ServerPowerControls';

export default function ServerConsole() {
  return (
    <Container>
      <div className="mb-4 flex justify-between">
        <h1 className="text-4xl font-header font-bold text-white">Server Console</h1>
        <ServerPowerControls />
      </div>
      <div className="grid grid-cols-4 gap-4 mb-4">
        <div className="col-span-3 h-full">
          <Spinner.Suspense>
            <Console />
          </Spinner.Suspense>
        </div>

        <Spinner.Suspense>
          <ServerDetails />
        </Spinner.Suspense>
      </div>
      <div className="bg-green-500 h-48">Stats</div>
    </Container>
  );
}
