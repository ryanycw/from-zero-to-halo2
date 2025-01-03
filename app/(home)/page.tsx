import Link from 'next/link';
import { redirect } from 'next/navigation';

export default function HomePage() {
  redirect('/docs');
  return (
    <main className="flex h-screen flex-col justify-center text-center">
      <h1 className="mb-4 text-2xl font-bold">From Zero to Halo2</h1>
      <p className="text-fd-muted-foreground">
        You can open{' '}
        <Link
          href="/docs"
          className="text-fd-foreground font-semibold underline"
        >
          /docs
        </Link>{' '}
        and see the documentation.
      </p>
    </main>
  );
}
