<script lang="ts" module>
    import { ModeWatcher } from 'mode-watcher';
</script>

<script lang="ts">
    import '../app.css';

    const { children } = $props();
</script>

<ModeWatcher modeStorageKey="APP_MODE_WATCHER__MODE" themeStorageKey="APP_MODE_WATCHER__THEME" />

{@render children()}
<!-- layerchart@next @tanstack/table-core -->
