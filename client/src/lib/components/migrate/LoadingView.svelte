<!-- src/lib/components/migrate/LoadingView.svelte -->
<div class="flex h-screen flex-col items-center justify-center">
	<h3 class="mb-4 text-lg font-semibold text-gray-700">Loading...</h3>
	<svg width="80" height="80" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
		<rect x="15" y="30" width="10" height="20" fill="#3B82F6" rx="5">
			<animate attributeName="height" values="20;40;20" dur="1s" repeatCount="indefinite" />
			<animate attributeName="y" values="30;20;30" dur="1s" repeatCount="indefinite" />
		</rect>
		<rect x="35" y="30" width="10" height="20" fill="#8B5CF6" rx="5">
			<animate
				attributeName="height"
				values="20;40;20"
				dur="1s"
				begin="0.2s"
				repeatCount="indefinite"
			/>
			<animate attributeName="y" values="30;20;30" dur="1s" begin="0.2s" repeatCount="indefinite" />
		</rect>
		<rect x="55" y="30" width="10" height="20" fill="#EC4899" rx="5">
			<animate
				attributeName="height"
				values="20;40;20"
				dur="1s"
				begin="0.4s"
				repeatCount="indefinite"
			/>
			<animate attributeName="y" values="30;20;30" dur="1s" begin="0.4s" repeatCount="indefinite" />
		</rect>
	</svg>
</div>
