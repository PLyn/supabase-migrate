// src/lib/stores.js
import { writable } from 'svelte/store';

// Global stores
export const projects = writable([]);
export const selectedSourceId = writable('');
export const selectedDestId = writable('');
export const migrationResults = writable([]);
export const migrationStatus = writable('Migration Status: Migration has not been run');

// Metrics stores
export const selectedProject = writable('');
export const projectMetrics = writable([]);
export const websocketEnabled = writable(false);
export const websocketStatus = writable('Disconnected');
export const secondsUntilUpdate = writable(60);

// Config items for migration
export const configItems = writable({
	Auth: false,
	Postgrest: false,
	EdgeFunctions: false,
	Secrets: false,
	Storage: false,
	Postgres: false,
	Branches: false
});
