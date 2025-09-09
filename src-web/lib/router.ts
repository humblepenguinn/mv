import { createRouter } from '@tanstack/react-router';
import { routeTree } from '../routeTree.gen';

export const router = createRouter({
  routeTree,
  basepath: import.meta.env.BASE_URL || '/',
});

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}
