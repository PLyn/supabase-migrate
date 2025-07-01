// src/lib/api.ts

const API_BASE = '/api';

export interface Project {
	id: string;
	name: string;
	region: string;
	status: string;
}

export interface DiffEntry {
	key: string;
	source_value: string;
	dest_value: string;
}

export interface ProjectConfig {
	name: string;
	diffs: DiffEntry[];
}

export interface ProjectMetrics {
	timestamp: string;
	value: string;
	metric_name: string;
	labels: string;
}

// Helper to handle server function calls
async function callServerFunction(functionName: string, params: Record<string, any> = {}) {
	const response = await fetch(`${API_BASE}/${functionName}`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(params)
	});

	if (!response.ok) {
		const error = await response.text();
		throw new Error(error || `Server error: ${response.status}`);
	}

	return response.json();
}

// Server functions
export const serverFunctions = {
	// Projects
	async getProjects(): Promise<Project[]> {
		return callServerFunction('get_projects');
	},

	// Migration functions
	async generatePreview(
		sourceId: string,
		destId: string,
		configItems: boolean[]
	): Promise<ProjectConfig[]> {
		return callServerFunction('generate_preview', {
			source_id: sourceId,
			dest_id: destId,
			config_items_rw: configItems
		});
	},

	async migrateConfig(
		projectConfig: ProjectConfig[],
		sourceId: string,
		destId: string
	): Promise<ProjectConfig[]> {
		return callServerFunction('migrate_config', {
			project_config: projectConfig,
			source_id: sourceId,
			dest_id: destId
		});
	},

	// Monitor functions
	async callSupabaseCli(command: string, dbString: string): Promise<string> {
		return callServerFunction('call_supabase_cli', {
			command,
			db_string: dbString
		});
	},

	// Metrics WebSocket
	async websocketMetricsStream(projectRef: string): Promise<ProjectMetrics[]> {
		// This will need special handling for WebSocket
		// For now, return a mock or implement WebSocket client
		return new Promise((resolve, reject) => {
			const ws = new WebSocket(`ws://localhost:3000/ws/metrics`);

			ws.onopen = () => {
				ws.send(JSON.stringify({ project_ref: projectRef }));
			};

			ws.onmessage = (event) => {
				const data = JSON.parse(event.data);
				resolve(data);
			};

			ws.onerror = (error) => {
				reject(error);
			};
		});
	}
};
