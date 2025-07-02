import { createClient } from '@supabase/supabase-js';

export const supabase = createClient(
	'https://qqihtoflgpacnrksygeu.supabase.co',
	'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWh0b2ZsZ3BhY25ya3N5Z2V1Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NTE0MDc3OTgsImV4cCI6MjA2Njk4Mzc5OH0.xkZXZ8aGsfaf-DtuQ1XvHIgOCYtMv1d5KF1QStdo6Og'
);
