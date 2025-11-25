document.addEventListener('DOMContentLoaded', () => {
    // Find the existing menu bar (the one that already has Print, Theme, etc.)
    const menuBar = document.querySelector('.menu-bar');
    if (!menuBar) return;

    // Create the RSS button
    const rssLink = document.createElement('a');
    rssLink.href = 'https://saylesss88.github.io/rss.xml';
    rssLink.target = '_blank';
    rssLink.rel = 'noopener';
    rssLink.title = 'Subscribe to RSS feed';
    rssLink.className = 'header-btn';           // same class as the other buttons
    rssLink.innerHTML = `
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom:-3px">
            <circle cx="6" cy="18" r="3"></circle>
            <path d="M6 6c6.627 0 12 5.373 12 12"></path>
            <path d="M6 12c3.314 0 6 2.686 6 6"></path>
        </svg>
    `;

    // Insert it just before the print button (or wherever you prefer)
    const printButton = menuBar.querySelector('.print-btn');
    if (printButton) {
        printButton.before(rssLink);
    } else {
        // fallback: just append it
        menuBar.appendChild(rssLink);
    }
});
