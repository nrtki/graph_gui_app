document.addEventListener('DOMContentLoaded', () => {
    if (window.__TAURI__) {
        console.log('Tauri API is available');
    } else {
        console.log('Tauri API is not available');
    }
    // ...existing code...
});